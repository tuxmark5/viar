use via_protocol::KeyboardLayout;

/// What a keycode edit applies to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EditTarget {
    /// A matrix key, by index into `layout.keys`.
    Key(usize),
    /// A rotary encoder rotation in one direction.
    Encoder { index: u8, clockwise: bool },
    /// An encoder's push switch — a matrix position not present in `layout.keys`.
    Push { row: u8, col: u8 },
}

/// A single keycode change for undo tracking (holds the value to restore).
#[derive(Clone, Copy)]
pub struct EditChange {
    pub layer:  usize,
    pub target: EditTarget,
    pub old:    u16,
}

/// The keycodes for one layer: the key matrix plus per-encoder rotation codes.
#[derive(Clone, Default)]
pub struct KeymapLayer {
    /// `matrix[row][col]` = raw keycode.
    pub matrix:   Vec<Vec<u16>>,
    /// Per-encoder `[ccw, cw]` keycodes, indexed by encoder index. Empty until
    /// encoder data has been read from the device.
    pub encoders: Vec<[u16; 2]>,
}

impl KeymapLayer {
    /// Build a layer from a matrix, with no encoder data yet.
    pub fn from_matrix(matrix: Vec<Vec<u16>>) -> Self {
        Self {
            matrix,
            encoders: Vec::new(),
        }
    }

    /// Keycode at `(row, col)`, or 0 if out of range.
    pub fn keycode(&self, row: u8, col: u8) -> u16 {
        self.matrix
            .get(row as usize)
            .and_then(|r| r.get(col as usize))
            .copied()
            .unwrap_or(0)
    }

    /// Set the keycode at `(row, col)` if it is in range.
    pub fn set_keycode(&mut self, row: u8, col: u8, keycode: u16) {
        if let Some(cell) = self
            .matrix
            .get_mut(row as usize)
            .and_then(|r| r.get_mut(col as usize))
        {
            *cell = keycode;
        }
    }

    /// Rotation keycode for `index` in the given direction, or 0 if out of range.
    pub fn encoder(&self, index: u8, clockwise: bool) -> u16 {
        self.encoders
            .get(index as usize)
            .map(|e| e[clockwise as usize])
            .unwrap_or(0)
    }

    /// Set the rotation keycode for `index` / direction if it is in range.
    pub fn set_encoder(&mut self, index: u8, clockwise: bool, keycode: u16) {
        if let Some(e) = self.encoders.get_mut(index as usize) {
            e[clockwise as usize] = keycode;
        }
    }
}

/// Loaded keymap data for display.
pub struct KeymapData {
    pub layout:         KeyboardLayout,
    /// Per-layer keycodes (matrix + encoders).
    pub layers:         Vec<KeymapLayer>,
    pub layer_count:    u8,
    pub selected_layer: usize,
    /// The slot (key / encoder direction / push) whose picker is open.
    pub selected:       Option<EditTarget>,
    /// Whether keymap has unsaved changes
    pub dirty:          bool,
    /// Undo history
    pub undo_stack:     Vec<EditChange>,
}

impl KeymapData {
    /// Keycode at `(layer, row, col)`, or 0 if any index is out of range.
    pub fn keycode_at(&self, layer: usize, row: u8, col: u8) -> u16 {
        self.layers
            .get(layer)
            .map(|l| l.keycode(row, col))
            .unwrap_or(0)
    }

    /// Matrix `(row, col)` a target resolves to, if it is a key or push switch.
    pub fn target_matrix(&self, target: EditTarget) -> Option<(u8, u8)> {
        match target {
            EditTarget::Key(idx) => self.layout.keys.get(idx).map(|k| (k.row, k.col)),
            EditTarget::Push { row, col } => Some((row, col)),
            EditTarget::Encoder { .. } => None,
        }
    }

    /// Current keycode assigned to `target` on `layer`, or 0 if out of range.
    pub fn target_keycode(&self, layer: usize, target: EditTarget) -> u16 {
        match target {
            EditTarget::Encoder { index, clockwise } => self
                .layers
                .get(layer)
                .map(|l| l.encoder(index, clockwise))
                .unwrap_or(0),
            EditTarget::Key(_) | EditTarget::Push { .. } => self
                .target_matrix(target)
                .map(|(row, col)| self.keycode_at(layer, row, col))
                .unwrap_or(0),
        }
    }

    /// Write `keycode` to `target` on `layer` in the local model.
    pub fn set_target_keycode(&mut self, layer: usize, target: EditTarget, keycode: u16) {
        match target {
            EditTarget::Encoder { index, clockwise } => {
                if let Some(l) = self.layers.get_mut(layer) {
                    l.set_encoder(index, clockwise, keycode);
                }
            }
            EditTarget::Key(_) | EditTarget::Push { .. } => {
                if let Some((row, col)) = self.target_matrix(target)
                    && let Some(l) = self.layers.get_mut(layer)
                {
                    l.set_keycode(row, col, keycode);
                }
            }
        }
    }
}
