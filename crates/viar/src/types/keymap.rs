use eframe::egui;
use via_protocol::{
    KeyAction,
    KeyboardLayout,
};

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

impl Default for EditTarget {
    /// A placeholder used only for an idle [`Flash`]; never a real edit.
    fn default() -> Self {
        Self::Key(0)
    }
}

/// A single keycode change for undo tracking (holds the value to restore).
#[derive(Clone, Copy)]
pub struct EditChange {
    pub layer:  usize,
    pub target: EditTarget,
    pub old:    KeyAction,
}

/// Undo history, grouped by action. A single-slot edit is a group of one; a
/// layer paste is one group covering all of its changed slots, so a single undo
/// reverts the whole paste.
#[derive(Default)]
pub struct UndoHistory {
    /// All recorded changes, oldest first, concatenated across groups.
    changes:     Vec<EditChange>,
    /// The size of each recorded group, in the order they were applied.
    group_sizes: Vec<usize>,
}

impl UndoHistory {
    /// Whether there is anything to undo.
    pub fn is_empty(&self) -> bool {
        self.group_sizes.is_empty()
    }

    /// Forget all history (e.g. after a reload).
    pub fn clear(&mut self) {
        self.changes.clear();
        self.group_sizes.clear();
    }

    /// Record `changes` as one undoable action. Empty groups are ignored so an
    /// action that changed nothing leaves no undo entry.
    pub fn record(&mut self, changes: &[EditChange]) {
        if !changes.is_empty() {
            self.group_sizes.push(changes.len());
            self.changes.extend_from_slice(changes);
        }
    }

    /// Remove the most recent action's changes, returned in reverse (restore)
    /// order. Empty when there is nothing to undo.
    pub fn pop_group(&mut self) -> Vec<EditChange> {
        let Some(n) = self.group_sizes.pop() else {
            return Vec::new();
        };
        let mut group = self.changes.split_off(self.changes.len() - n);
        group.reverse();
        group
    }
}

/// How long a copy/paste pulse animation lasts, in seconds.
pub const FLASH_DURATION: f64 = 0.45;

/// Which copy/paste action triggered a flash animation.
#[derive(Clone, Copy, Default)]
pub enum FlashKind {
    #[default]
    Copy,
    Paste,
}

impl FlashKind {
    /// The pulse color for this flash kind.
    pub fn color(self) -> egui::Color32 {
        match self {
            Self::Copy => egui::Color32::from_rgb(120, 200, 255),
            Self::Paste => egui::Color32::from_rgb(130, 230, 150),
        }
    }
}

/// A transient highlight after a copy/paste, animated out over [`FLASH_DURATION`].
/// `T` is what is flashing — an [`EditTarget`] for a key slot ([`KeyFlash`]) or a
/// layer index ([`LayerFlash`]). Always present; `start` is `None` when idle.
#[derive(Clone, Copy, Default)]
pub struct Flash<T> {
    pub subject: T,
    /// egui time (seconds) when the flash started, or None when idle.
    pub start:   Option<f64>,
    pub kind:    FlashKind,
}

impl<T: Copy> Flash<T> {
    /// Begin a pulse on `subject` at time `now`.
    pub fn trigger(&mut self, subject: T, now: f64, kind: FlashKind) {
        self.subject = subject;
        self.start = Some(now);
        self.kind = kind;
    }

    /// Stop the pulse once its animation has run its course.
    pub fn clear_if_finished(&mut self, now: f64) {
        if self.start.is_some_and(|s| now - s >= FLASH_DURATION) {
            self.start = None;
        }
    }

    /// The active pulse as `(subject, progress 0..1, color)`, or None when idle
    /// or finished.
    pub fn active(&self, now: f64) -> Option<(T, f32, egui::Color32)> {
        let start = self.start?;
        let progress = ((now - start) / FLASH_DURATION) as f32;
        (progress < 1.0).then(|| (self.subject, progress, self.kind.color()))
    }
}

/// A copy/paste flash on a key slot.
pub type KeyFlash = Flash<EditTarget>;
/// A copy/paste flash on a layer tab.
pub type LayerFlash = Flash<usize>;

/// The keycodes for one layer: the key matrix plus per-encoder rotation codes.
#[derive(Clone, Default)]
pub struct KeymapLayer {
    /// `matrix[row][col]` = decoded key action.
    pub matrix:   Vec<Vec<KeyAction>>,
    /// Per-encoder `[ccw, cw]` actions, indexed by encoder index. Empty until
    /// encoder data has been read from the device.
    pub encoders: Vec<[KeyAction; 2]>,
}

impl KeymapLayer {
    /// Build a layer from a matrix, with no encoder data yet.
    pub fn from_matrix(matrix: Vec<Vec<KeyAction>>) -> Self {
        Self {
            matrix,
            encoders: Vec::new(),
        }
    }

    /// Action at `(row, col)`, or `KC_NO` if out of range.
    pub fn keycode(&self, row: u8, col: u8) -> KeyAction {
        self.matrix
            .get(row as usize)
            .and_then(|r| r.get(col as usize))
            .copied()
            .unwrap_or_default()
    }

    /// Set the action at `(row, col)` if it is in range.
    pub fn set_keycode(&mut self, row: u8, col: u8, keycode: KeyAction) {
        if let Some(cell) = self
            .matrix
            .get_mut(row as usize)
            .and_then(|r| r.get_mut(col as usize))
        {
            *cell = keycode;
        }
    }

    /// Rotation action for `index` in the given direction, or `KC_NO` if out of
    /// range.
    pub fn encoder(&self, index: u8, clockwise: bool) -> KeyAction {
        self.encoders
            .get(index as usize)
            .map(|e| e[clockwise as usize])
            .unwrap_or_default()
    }

    /// Set the rotation action for `index` / direction if it is in range.
    pub fn set_encoder(&mut self, index: u8, clockwise: bool, keycode: KeyAction) {
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
    /// Undo history, grouped by action.
    pub undo:           UndoHistory,
}

impl KeymapData {
    /// Action at `(layer, row, col)`, or `KC_NO` if any index is out of range.
    pub fn keycode_at(&self, layer: usize, row: u8, col: u8) -> KeyAction {
        self.layers
            .get(layer)
            .map(|l| l.keycode(row, col))
            .unwrap_or_default()
    }

    /// Matrix `(row, col)` a target resolves to, if it is a key or push switch.
    pub fn target_matrix(&self, target: EditTarget) -> Option<(u8, u8)> {
        match target {
            EditTarget::Key(idx) => self.layout.keys.get(idx).map(|k| (k.row, k.col)),
            EditTarget::Push { row, col } => Some((row, col)),
            EditTarget::Encoder { .. } => None,
        }
    }

    /// Current action assigned to `target` on `layer`, or `KC_NO` if out of range.
    pub fn target_keycode(&self, layer: usize, target: EditTarget) -> KeyAction {
        match target {
            EditTarget::Encoder { index, clockwise } => self
                .layers
                .get(layer)
                .map(|l| l.encoder(index, clockwise))
                .unwrap_or_default(),
            EditTarget::Key(_) | EditTarget::Push { .. } => self
                .target_matrix(target)
                .map(|(row, col)| self.keycode_at(layer, row, col))
                .unwrap_or_default(),
        }
    }

    /// Write `keycode` to `target` on `layer` in the local model.
    pub fn set_target_keycode(&mut self, layer: usize, target: EditTarget, keycode: KeyAction) {
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
