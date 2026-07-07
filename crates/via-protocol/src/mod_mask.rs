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

    /// Ctrl+Shift+Alt (Meh).
    pub const MEH: Self = Self::CTRL.and(Self::SHIFT).and(Self::ALT);
    /// Ctrl+Shift+Alt+GUI (Hyper).
    pub const HYPER: Self = Self::MEH.and(Self::GUI);

    /// Combine two masks (bitwise OR). The `const` primitive behind the `|`
    /// operator, for use in `const` contexts (const items can't call operators).
    pub const fn and(self, other: Self) -> Self {
        Self(self.0 | other.0)
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

impl std::ops::BitOr for ModMask {
    type Output = Self;

    /// Combine two masks, e.g. `ModMask::CTRL | ModMask::SHIFT`.
    fn bitor(self, rhs: Self) -> Self {
        self.and(rhs)
    }
}

/// Human-readable modifier form, e.g. `C+S` (Ctrl+Shift); `MOD` when empty. A set
/// right-hand bit prefixes each label with `R` (`RC`, `RS`, …). Use `.0` for the
/// raw mask byte.
impl std::fmt::Display for ModMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // The right-side flag prefixes every label with `R` (`C` -> `RC`).
        let prefix = if self.0 & Self::RIGHT.0 != 0 { "R" } else { "" };
        let mut wrote_any = false;
        for (bit, label) in [
            (Self::CTRL, "C"),
            (Self::SHIFT, "S"),
            (Self::ALT, "A"),
            (Self::GUI, "G"),
        ] {
            if self.0 & bit.0 != 0 {
                if wrote_any {
                    f.write_str("+")?;
                }
                write!(f, "{prefix}{label}")?;
                wrote_any = true;
            }
        }
        // No modifier bits set — a bare placeholder.
        if wrote_any {
            Ok(())
        } else {
            f.write_str("MOD")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_formats_mod_combinations() {
        assert_eq!(ModMask::CTRL.to_string(), "C");
        assert_eq!((ModMask::CTRL | ModMask::SHIFT).to_string(), "C+S");
        assert_eq!(ModMask::HYPER.to_string(), "C+S+A+G");
        assert_eq!(ModMask::RCTRL.to_string(), "RC");
        assert_eq!((ModMask::RCTRL | ModMask::RSHIFT).to_string(), "RC+RS");
        // No modifier bits (incl. a lone right-side flag) reads as the placeholder.
        assert_eq!(ModMask(0x00).to_string(), "MOD");
        assert_eq!(ModMask::RIGHT.to_string(), "MOD");
    }
}
