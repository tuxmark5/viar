//! VIA tap-dance keycodes — `TD(n)` — indexed into the user's tap-dance table.

/// A VIA tap-dance keycode `TD(n)`. Stored as the index `n` within the
/// [`TapDanceKey::BLOCK`] (`0x5700`) block; the full keycode is
/// [`raw`](TapDanceKey::raw). Scheme-independent (same range in both).
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TapDanceKey(pub u8);

impl TapDanceKey {
    /// Base of the tap-dance keycode block; the full keycode is `BLOCK | index`.
    pub const BLOCK: u16 = 0x5700;

    /// The full `u16` keycode value (`BLOCK | index`).
    pub const fn raw(self) -> u16 {
        Self::BLOCK | self.0 as u16
    }

    /// Short display name, e.g. `TD(3)`.
    pub fn name(self) -> String {
        format!("TD({})", self.0)
    }

    /// Longer human description for tooltips.
    pub fn description(self) -> String {
        format!(
            "Tap Dance {} — different actions for tap/hold/double-tap",
            self.0
        )
    }
}

impl std::fmt::Display for TapDanceKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name())
    }
}
