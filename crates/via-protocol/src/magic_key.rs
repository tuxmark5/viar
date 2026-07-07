//! QMK "magic" keycodes — the `0x7000` block: runtime toggles for control/GUI
//! swaps, NKRO, grave-escape, backspace/backslash swap, etc.

use crate::keycode_macros::keycode_block;

/// A QMK magic keycode. Stored as the offset within the [`MagicKey::BLOCK`]
/// (`0x7000`) block; the full keycode is [`raw`](MagicKey::raw). Only free in the
/// new scheme — in the old scheme `0x7000` is mod-tap.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MagicKey(pub u8);

keycode_block! {
    MagicKey, block 0x7000, category "Magic",
    {
        0x00 => "MG_SWNU", MG_SWNU, "Swap Control and GUI (on)",
        0x01 => "MG_SWCU", MG_SWCU, "Swap Control and Caps Lock (on)",
        0x02 => "MG_SWLA", MG_SWLA, "Swap Left Alt and GUI (on)",
        0x03 => "MG_SWRA", MG_SWRA, "Swap Right Alt and GUI (on)",
        0x04 => "MG_NKRO", MG_NKRO, "N-Key Rollover (on)",
        0x05 => "MG_GESC", MG_GESC, "Grave Escape (on)",
        0x06 => "MG_BSPC", MG_BSPC, "Swap Backspace and Backslash (on)",
        0x07 => "CG_NORM", CG_NORM, "Unswap Control and GUI",
        0x08 => "MG_UNCC", MG_UNCC, "Unswap Caps Lock",
        0x09 => "MG_UNLA", MG_UNLA, "Unswap Left Alt and GUI",
        0x0A => "MG_UNRA", MG_UNRA, "Unswap Right Alt and GUI",
        0x0B => "MG_UNNK", MG_UNNK, "N-Key Rollover (off)",
        0x0C => "MG_UNGE", MG_UNGE, "Grave Escape (off)",
        0x0D => "MG_UNBS", MG_UNBS, "Unswap Backspace",
        0x0E => "CG_TOGG", CG_TOGG, "Toggle Control and GUI swap",
        0x0F => "MG_TOGN", MG_TOGN, "Toggle NKRO",
    }
}

