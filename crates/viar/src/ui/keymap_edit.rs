//! Applying keycode edits to the keymap: local-model update, undo tracking, and
//! writing the change through to the device. Kept separate from `keymap_tab`,
//! which is concerned with rendering and interaction.

use tracing::{
    info,
    warn,
};
use via_protocol::{
    Keycode,
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
    /// Apply a keycode to any slot (key, encoder direction, or push), recording
    /// undo and writing it through to the device.
    pub fn apply_edit(&mut self, target: EditTarget, new_keycode: u16) {
        let Some(data) = &mut self.keymap_data else {
            return;
        };
        let layer = data.selected_layer;
        let old_keycode = data.target_keycode(layer, target);
        if old_keycode == new_keycode {
            return;
        }
        data.set_target_keycode(layer, target, new_keycode);
        data.undo_stack.push(EditChange {
            layer,
            target,
            old: old_keycode,
        });
        data.dirty = true;
        let matrix = data.target_matrix(target);
        self.write_target(target, layer, matrix, new_keycode, "Set");
    }

    pub fn undo(&mut self) {
        let Some(data) = &mut self.keymap_data else {
            return;
        };
        let Some(change) = data.undo_stack.pop() else {
            return;
        };
        data.set_target_keycode(change.layer, change.target, change.old);
        if data.undo_stack.is_empty() {
            data.dirty = false;
        }
        let matrix = data.target_matrix(change.target);
        self.write_target(change.target, change.layer, matrix, change.old, "Undo");
    }

    /// Write one keycode to the device for `target`, updating the status line.
    /// `verb` labels the status message ("Set" / "Undo").
    fn write_target(
        &mut self,
        target: EditTarget,
        layer: usize,
        matrix: Option<(u8, u8)>,
        keycode: u16,
        verb: &str,
    ) {
        let Some(dev) = &self.connected_device else {
            return;
        };
        let proto = ViaProtocol::new(dev);
        let result = match target {
            EditTarget::Encoder { index, clockwise } => {
                proto.set_encoder(layer as u8, index, clockwise, keycode)
            }
            EditTarget::Key(_) | EditTarget::Push { .. } => match matrix {
                Some((row, col)) => proto.set_keycode(layer as u8, row, col, keycode),
                None => return,
            },
        };
        let desc = target_desc(target, matrix);
        match result {
            Ok(()) => {
                let name = Keycode(keycode).name();
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
