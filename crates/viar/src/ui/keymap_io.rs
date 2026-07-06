//! Import / export of the keymap to and from a JSON file on disk.

use tracing::{
    info,
    warn,
};
use via_protocol::{
    Keycode,
    ViaProtocol,
};

use crate::types::{
    StatusMessage,
    ViarApp,
};

impl ViarApp {
    pub fn reload_keymap(&mut self) {
        if let (Some(dev), Some(data)) = (&self.connected_device, &mut self.keymap_data) {
            let proto = ViaProtocol::new(dev);
            match proto.read_entire_keymap(data.layer_count, data.layout.rows, data.layout.cols) {
                Ok(km) => {
                    info!("keymap reloaded");
                    // Replace each layer's matrix, keeping already-loaded encoder data.
                    for (layer, matrix) in data.layers.iter_mut().zip(km) {
                        layer.matrix = matrix;
                    }
                    data.dirty = false;
                    data.undo_stack.clear();
                    self.set_status(StatusMessage::info("Keymap reloaded from device"));
                }
                Err(e) => {
                    warn!(error = %e, "failed to reload keymap");
                    self.set_status(StatusMessage::error(format!("Reload failed: {e}")));
                }
            }
        }
    }

    pub fn export_keymap(&mut self) {
        let Some(data) = &self.keymap_data else {
            return;
        };

        let mut layers = Vec::new();
        for (layer_idx, layer) in data.layers.iter().enumerate() {
            let mut rows = Vec::new();
            for (row_idx, row) in layer.matrix.iter().enumerate() {
                let keys: Vec<serde_json::Value> = row
                    .iter()
                    .enumerate()
                    .map(|(col_idx, &raw_kc)| {
                        serde_json::json!({
                            "col": col_idx,
                            "raw": raw_kc,
                            "name": Keycode(raw_kc).name(),
                        })
                    })
                    .collect();
                rows.push(serde_json::json!({
                    "row": row_idx,
                    "keys": keys,
                }));
            }
            let encoders: Vec<serde_json::Value> = layer
                .encoders
                .iter()
                .enumerate()
                .map(|(index, &[ccw, cw])| {
                    serde_json::json!({
                        "index": index,
                        "ccw": ccw,
                        "ccw_name": Keycode(ccw).name(),
                        "cw": cw,
                        "cw_name": Keycode(cw).name(),
                    })
                })
                .collect();
            layers.push(serde_json::json!({
                "layer": layer_idx,
                "rows": rows,
                "encoders": encoders,
            }));
        }

        let dump = serde_json::json!({
            "viar_version": 2,
            "layout": data.layout.name,
            "matrix_rows": data.layout.rows,
            "matrix_cols": data.layout.cols,
            "layer_count": data.layer_count,
            "layers": layers,
        });

        let path = "viar_keymap.json";
        let json_str = match serde_json::to_string_pretty(&dump) {
            Ok(s) => s,
            Err(e) => {
                warn!(error = %e, "failed to serialize keymap");
                self.set_status(StatusMessage::error(format!("Export failed: {e}")));
                return;
            }
        };
        match std::fs::write(path, json_str) {
            Ok(_) => {
                info!("keymap exported to {path}");
                if let Some(data) = &mut self.keymap_data {
                    data.dirty = false;
                }
                self.set_status(StatusMessage::info(format!("Exported to {path}")));
            }
            Err(e) => {
                warn!(error = %e, "failed to export keymap");
                self.set_status(StatusMessage::error(format!("Export failed: {e}")));
            }
        }
    }

    pub fn import_keymap(&mut self) {
        let path = "viar_keymap.json";
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                warn!(error = %e, "failed to read keymap file");
                self.set_status(StatusMessage::error(format!("Import failed: {e}")));
                return;
            }
        };

        let json: serde_json::Value = match serde_json::from_str(&content) {
            Ok(v) => v,
            Err(e) => {
                self.set_status(StatusMessage::error(format!("Invalid JSON: {e}")));
                return;
            }
        };

        let Some(data) = &self.keymap_data else {
            return;
        };

        let file_rows = json["matrix_rows"].as_u64().unwrap_or(0) as u8;
        let file_cols = json["matrix_cols"].as_u64().unwrap_or(0) as u8;
        let expected_rows = data.layout.rows;
        let expected_cols = data.layout.cols;
        let _ = data;

        if file_rows != expected_rows || file_cols != expected_cols {
            self.set_status(StatusMessage::error(format!(
                "Matrix mismatch: file is {file_rows}x{file_cols}, keyboard is {expected_rows}x{expected_cols}",
            )));
            return;
        }

        let Some(layers) = json["layers"].as_array() else {
            self.set_status(StatusMessage::error("No layers array in file"));
            return;
        };

        let Some(data) = &mut self.keymap_data else {
            return;
        };

        let mut new_matrix: Vec<Vec<Vec<u16>>> =
            data.layers.iter().map(|l| l.matrix.clone()).collect();
        let mut new_encoders: Vec<Vec<[u16; 2]>> =
            data.layers.iter().map(|l| l.encoders.clone()).collect();
        for layer_obj in layers {
            let layer_idx = layer_obj["layer"].as_u64().unwrap_or(0) as usize;
            if layer_idx >= new_matrix.len() {
                continue;
            }
            if let Some(rows) = layer_obj["rows"].as_array() {
                for row_obj in rows {
                    let row_idx = row_obj["row"].as_u64().unwrap_or(0) as usize;
                    if row_idx >= new_matrix[layer_idx].len() {
                        continue;
                    }
                    let Some(keys) = row_obj["keys"].as_array() else {
                        continue;
                    };
                    for key_obj in keys {
                        let col_idx = key_obj["col"].as_u64().unwrap_or(0) as usize;
                        let raw = key_obj["raw"].as_u64().unwrap_or(0) as u16;
                        if col_idx < new_matrix[layer_idx][row_idx].len() {
                            new_matrix[layer_idx][row_idx][col_idx] = raw;
                        }
                    }
                }
            }
            // Encoders are optional (absent in v1 files).
            if let Some(encoders) = layer_obj["encoders"].as_array() {
                for enc_obj in encoders {
                    let index = enc_obj["index"].as_u64().unwrap_or(0) as usize;
                    if index < new_encoders[layer_idx].len() {
                        let ccw = enc_obj["ccw"].as_u64().unwrap_or(0) as u16;
                        let cw = enc_obj["cw"].as_u64().unwrap_or(0) as u16;
                        new_encoders[layer_idx][index] = [ccw, cw];
                    }
                }
            }
        }

        let mut changed = 0usize;
        let mut errors = 0usize;
        if let Some(dev) = &self.connected_device {
            let proto = ViaProtocol::new(dev);
            for (layer, layer_keys) in new_matrix.iter().enumerate() {
                for (row, row_keys) in layer_keys.iter().enumerate() {
                    for (col, &new) in row_keys.iter().enumerate() {
                        let old = data.keycode_at(layer, row as u8, col as u8);
                        if old != new {
                            match proto.set_keycode(layer as u8, row as u8, col as u8, new) {
                                Ok(()) => changed += 1,
                                Err(e) => {
                                    warn!(error = %e, layer, row, col, "failed to write key");
                                    errors += 1;
                                }
                            }
                        }
                    }
                }
            }
            for (layer, layer_encs) in new_encoders.iter().enumerate() {
                for (index, &[ccw, cw]) in layer_encs.iter().enumerate() {
                    for (clockwise, new) in [(false, ccw), (true, cw)] {
                        let old = data
                            .layers
                            .get(layer)
                            .map(|l| l.encoder(index as u8, clockwise))
                            .unwrap_or(0);
                        if old != new {
                            match proto.set_encoder(layer as u8, index as u8, clockwise, new) {
                                Ok(()) => changed += 1,
                                Err(e) => {
                                    warn!(error = %e, layer, index, clockwise, "failed to write encoder");
                                    errors += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        // Write the imported values back into local state.
        for ((layer, matrix), encoders) in data.layers.iter_mut().zip(new_matrix).zip(new_encoders)
        {
            layer.matrix = matrix;
            layer.encoders = encoders;
        }

        if errors > 0 {
            self.set_status(StatusMessage::error(format!(
                "Imported with {errors} write errors ({changed} slots updated)"
            )));
        } else {
            info!(changed, "keymap imported from {path}");
            self.set_status(StatusMessage::info(format!(
                "Imported {changed} changes from {path}"
            )));
        }
    }
}
