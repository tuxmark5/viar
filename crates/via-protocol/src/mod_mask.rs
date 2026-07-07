//! A QMK modifier mask and named constants for the individual modifiers.

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

    /// Human-readable form, e.g. `C+S` (Ctrl+Shift); `MOD` when empty. A set
    /// right-hand bit prefixes each label with `R` (`RC`, `RS`, …).
    pub fn name(self) -> String {
        let mods = self.0;
        let mut parts = Vec::new();
        if mods & 0x01 != 0 {
            parts.push("C");
        } // Ctrl
        if mods & 0x02 != 0 {
            parts.push("S");
        } // Shift
        if mods & 0x04 != 0 {
            parts.push("A");
        } // Alt
        if mods & 0x08 != 0 {
            parts.push("G");
        } // GUI
        // Right-side flag — relabel to the right-hand variants.
        if mods & 0x10 != 0 {
            parts.iter_mut().for_each(|p| {
                *p = match *p {
                    "C" => "RC",
                    "S" => "RS",
                    "A" => "RA",
                    "G" => "RG",
                    _ => *p,
                }
            });
        }
        if parts.is_empty() {
            "MOD".to_string()
        } else {
            parts.join("+")
        }
    }

    /// QMK-style Mod-Tap prefix for this mask, e.g. `LSFT_T`, `LCTL_T`, `MEH_T`;
    /// falls back to `MT` for masks without a named alias.
    pub fn mod_tap_prefix(self) -> &'static str {
        match self.0 {
            0x01 => "LCTL_T",
            0x02 => "LSFT_T",
            0x04 => "LALT_T",
            0x08 => "LGUI_T",
            0x11 => "RCTL_T",
            0x12 => "RSFT_T",
            0x14 => "RALT_T",
            0x18 => "RGUI_T",
            0x03 => "C_S_T",
            0x05 => "LCA_T",
            0x09 => "LCG_T",
            0x06 => "LSA_T",
            0x0A => "LSG_T",
            0x0C => "LAG_T",
            0x07 => "MEH_T",
            0x0F => "HYPR_T",
            0x13 => "RCS_T",
            0x15 => "RCA_T",
            0x19 => "RCG_T",
            0x16 => "RSA_T",
            0x1A => "RSG_T",
            0x1C => "RAG_T",
            0x17 => "RMEH_T",
            0x1F => "RHYP_T",
            _ => "MT",
        }
    }
}

/// `Display` the human-readable modifier form (`Ctrl+Shift`); use `.0` for the
/// raw mask byte.
impl std::fmt::Display for ModMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name())
    }
}
