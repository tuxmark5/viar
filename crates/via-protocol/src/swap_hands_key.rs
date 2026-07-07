//! QMK swap-hands keycodes — the `0x5600` block. Offsets `0xF0–0xF6` are the
//! swap-hands *mode* keys (toggle, momentary, one-shot, …); every lower offset
//! is "swap hands while the given key is held" — `SH(kc)`.

use crate::basic_key::BasicKey;

/// A QMK swap-hands keycode. Stored as the offset within the
/// [`SwapHandsKey::BLOCK`] (`0x5600`) block; the full keycode is
/// [`raw`](SwapHandsKey::raw). Scheme-independent (`0x5600` is free in both).
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SwapHandsKey(pub u8);

impl SwapHandsKey {
    /// Base of the swap-hands keycode block; the full keycode is `BLOCK | offset`.
    pub const BLOCK: u16 = 0x5600;

    pub const SH_TG: Self = Self(0xF0);
    pub const SH_TT: Self = Self(0xF1);
    pub const SH_MON: Self = Self(0xF2);
    pub const SH_MOFF: Self = Self(0xF3);
    pub const SH_OFF: Self = Self(0xF4);
    pub const SH_ON: Self = Self(0xF5);
    pub const SH_OS: Self = Self(0xF6);

    /// The full `u16` keycode value (`BLOCK | offset`).
    pub const fn raw(self) -> u16 {
        Self::BLOCK | self.0 as u16
    }

    /// The canonical QMK name for a mode key (`SH_TG`, …); `None` for the
    /// parametric `SH(kc)` forms, whose name is composed at runtime.
    pub fn qmk_name(self) -> Option<&'static str> {
        Some(match self.0 {
            0xF0 => "SH_TG",
            0xF1 => "SH_TT",
            0xF2 => "SH_MON",
            0xF3 => "SH_MOFF",
            0xF4 => "SH_OFF",
            0xF5 => "SH_ON",
            0xF6 => "SH_OS",
            _ => return None,
        })
    }

    /// Longer human description for tooltips.
    pub fn description(self) -> String {
        match self.0 {
            0xF0 => "Toggle swap hands".to_string(),
            0xF1 => "Tap-toggle swap hands".to_string(),
            0xF2 => "Momentary swap on".to_string(),
            0xF3 => "Momentary swap off".to_string(),
            0xF4 => "Turn off swap hands".to_string(),
            0xF5 => "Turn on swap hands".to_string(),
            0xF6 => "One-shot swap hands".to_string(),
            _ => format!("{} on tap, swap hands on hold", BasicKey(self.0)),
        }
    }
}

/// Short display name, e.g. `SH_TG` or `SH(A)`.
impl std::fmt::Display for SwapHandsKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.qmk_name() {
            Some(n) => f.write_str(n),
            None => write!(f, "SH({})", BasicKey(self.0)),
        }
    }
}
