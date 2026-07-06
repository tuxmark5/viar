//! Applying keycode edits to the keymap: local-model update, undo tracking, and
//! writing the change through to the device. Kept separate from `keymap_tab`,
//! which is concerned with rendering and interaction.

use tracing::{
    info,
    warn,
};
use via_protocol::{
    KeyAction,
    ViaProtocol,
};

use crate::{
    types::{
        EditChange,
        EditTarget,
        StatusMessage,
        ViarApp,
    },
    util::is_disconnect_error,
};

impl ViarApp {
    /// Apply a keycode to any slot (key, encoder direction, or push) on the
    /// active layer as a single undoable action, writing it through to the device.
    pub fn apply_edit(&mut self, target: EditTarget, new_keycode: KeyAction) {
        let Some(layer) = self.keymap_data.as_ref().map(|d| d.selected_layer) else {
            return;
        };
        if let Some(change) = self.apply_one(layer, target, new_keycode)
            && let Some(data) = &mut self.keymap_data
        {
            data.undo.record(&[change]);
        }
    }

    /// Apply one keycode edit to `layer`: update the local model and write it to
    /// the device. Returns the [`EditChange`] to undo it, or None if the value
    /// was unchanged (so nothing was applied). Does not touch the undo history —
    /// the caller decides how to group changes into actions.
    fn apply_one(
        &mut self,
        layer: usize,
        target: EditTarget,
        new_keycode: KeyAction,
    ) -> Option<EditChange> {
        let data = self.keymap_data.as_mut()?;
        let old = data.target_keycode(layer, target);
        if old == new_keycode {
            return None;
        }
        data.set_target_keycode(layer, target, new_keycode);
        data.dirty = true;
        let matrix = data.target_matrix(target);
        self.write_target(target, layer, matrix, new_keycode, "Set");
        Some(EditChange { layer, target, old })
    }

    /// Copy a slot's current keycode into the clipboard (shift + right-click).
    pub fn copy_slot(&mut self, target: EditTarget) {
        let Some(data) = &self.keymap_data else {
            return;
        };
        let kc = data.target_keycode(data.selected_layer, target);
        self.copied_keycode = Some(kc);
        self.set_status(StatusMessage::info(format!("Copied {}", kc.name())));
    }

    /// Paste the clipboard keycode into a slot (shift + left-click).
    pub fn paste_slot(&mut self, target: EditTarget) {
        match self.copied_keycode {
            Some(kc) => self.apply_edit(target, kc),
            None => self.set_status(StatusMessage::info("Nothing to paste")),
        }
    }

    /// Copy a whole layer (matrix + encoders) into the clipboard.
    pub fn copy_layer(&mut self, layer: usize) {
        let Some(copied) = self
            .keymap_data
            .as_ref()
            .and_then(|d| d.layers.get(layer))
            .cloned()
        else {
            return;
        };
        self.copied_layer = Some(copied);
        self.set_status(StatusMessage::info(format!("Copied layer {layer}")));
    }

    /// Paste the clipboard layer into `layer` as a single undoable action,
    /// writing each changed keycode to the device.
    pub fn paste_layer(&mut self, layer: usize) {
        let Some(source) = self.copied_layer.clone() else {
            self.set_status(StatusMessage::info("No layer copied"));
            return;
        };
        if !self
            .keymap_data
            .as_ref()
            .is_some_and(|d| layer < d.layers.len())
        {
            return;
        }

        // Route every cell through `apply_one`; it skips unchanged slots and
        // returns a change to undo for each one that actually moved.
        let mut changes = Vec::new();
        for (row, codes) in source.matrix.iter().enumerate() {
            for (col, &kc) in codes.iter().enumerate() {
                let target = EditTarget::Push {
                    row: row as u8,
                    col: col as u8,
                };
                if let Some(change) = self.apply_one(layer, target, kc) {
                    changes.push(change);
                }
            }
        }
        for (index, &[ccw, cw]) in source.encoders.iter().enumerate() {
            for (clockwise, kc) in [(false, ccw), (true, cw)] {
                let target = EditTarget::Encoder {
                    index: index as u8,
                    clockwise,
                };
                if let Some(change) = self.apply_one(layer, target, kc) {
                    changes.push(change);
                }
            }
        }

        let count = changes.len();
        // If a write disconnected us mid-paste, keymap_data is gone — leave the
        // disconnect status in place rather than overwriting it.
        if let Some(data) = &mut self.keymap_data {
            data.undo.record(&changes);
            self.set_status(StatusMessage::info(format!(
                "Pasted into layer {layer} ({count} changes)"
            )));
        }
    }

    /// Undo the most recent action, restoring every slot it changed (one slot for
    /// a normal edit, all changed slots for a layer paste).
    pub fn undo(&mut self) {
        let changes = match &mut self.keymap_data {
            Some(data) => data.undo.pop_group(),
            None => return,
        };
        if changes.is_empty() {
            return;
        }
        for change in &changes {
            if let Some(data) = &mut self.keymap_data {
                data.set_target_keycode(change.layer, change.target, change.old);
            }
            let matrix = self
                .keymap_data
                .as_ref()
                .and_then(|d| d.target_matrix(change.target));
            self.write_target(change.target, change.layer, matrix, change.old, "Undo");
        }
        if let Some(data) = &mut self.keymap_data {
            data.dirty = !data.undo.is_empty();
        }
        if changes.len() > 1 {
            self.set_status(StatusMessage::info(format!(
                "Undid {} changes",
                changes.len()
            )));
        }
    }

    /// Write one keycode to the device for `target`, updating the status line.
    /// `verb` labels the status message ("Set" / "Undo").
    fn write_target(
        &mut self,
        target: EditTarget,
        layer: usize,
        matrix: Option<(u8, u8)>,
        keycode: KeyAction,
        verb: &str,
    ) {
        // Encode the action into a raw keycode in the device's scheme.
        let raw = self.encoding.encode(keycode);
        let Some(dev) = &self.connected_device else {
            return;
        };
        let proto = ViaProtocol::new(dev);
        let result = match target {
            EditTarget::Encoder { index, clockwise } => {
                // Vial keyboards use the Vial encoder command; VIA-only use VIA's.
                if self.vial_protocol_version.is_some() {
                    proto.vial_set_encoder(layer as u8, index, clockwise, raw)
                } else {
                    proto.set_encoder(layer as u8, index, clockwise, raw)
                }
            }
            EditTarget::Key(_) | EditTarget::Push { .. } => match matrix {
                Some((row, col)) => proto.set_keycode(layer as u8, row, col, raw),
                None => return,
            },
        };
        let desc = target_desc(target, matrix);
        match result {
            Ok(()) => {
                let name = keycode.name();
                info!(?target, layer, keycode = name, "keycode written to device");
                self.set_status(StatusMessage::info(format!("{verb} {desc} -> {name}")));
            }
            Err(e) => {
                let err_str = format!("{e}");
                warn!(error = %e, "failed to write keycode to device");
                self.set_status(StatusMessage::error(format!("{verb} failed: {e}")));
                if is_disconnect_error(&err_str) {
                    self.handle_disconnect();
                }
            }
        }
    }
}

/// Short slot description for status messages (e.g. `[0,4]` or `Enc0 CW`).
fn target_desc(target: EditTarget, matrix: Option<(u8, u8)>) -> String {
    match target {
        EditTarget::Encoder { index, clockwise } => {
            format!("Enc{index} {}", if clockwise { "CW" } else { "CCW" })
        }
        _ => matrix
            .map(|(row, col)| format!("[{row},{col}]"))
            .unwrap_or_default(),
    }
}
