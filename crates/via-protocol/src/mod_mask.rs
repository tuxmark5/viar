//! A QMK modifier mask and named constants for the individual modifiers.

use crate::keycodes::mod_mask_to_string;

/// A QMK modifier mask: bits 0–3 are Ctrl/Shift/Alt/GUI, bit 4 ([`ModMask::RIGHT`])
/// selects the right-hand variants.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModMask(pub u8);

impl ModMask {
    pub const CTRL: Self = Self(0x01);
    pub const SHIFT: Self = Self(0x02);
    pub const ALT: Self = Self(0x04);
    pub const GUI: Self = Self(0x08);
    /// Bit that flips a mask to the right-hand modifiers.
    pub const RIGHT: Self = Self(0x10);

    pub const RCTRL: Self = Self::CTRL.and(Self::RIGHT);
    pub const RSHIFT: Self = Self::SHIFT.and(Self::RIGHT);
    pub const RALT: Self = Self::ALT.and(Self::RIGHT);
    pub const RGUI: Self = Self::GUI.and(Self::RIGHT);

    /// Combine two masks (bitwise OR).
    pub const fn and(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Human-readable form, e.g. `Ctrl+Shift`.
    pub fn name(self) -> String {
        mod_mask_to_string(self.0)
    }
}

/// `Display` the human-readable modifier form (`Ctrl+Shift`); use `.0` for the
/// raw mask byte.
impl std::fmt::Display for ModMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name())
    }
}
