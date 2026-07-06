//! QMK "quantum" keycodes — bootloader, auto-shift, unicode, haptic, macros,
//! etc. (the `0x7C00` block) — with the canonical `QK_*` names from
//! `quantum/keycodes.h`.

use crate::keycode_macros::keycode_block;

/// A QMK quantum keycode. Stored as the offset within the [`QuantumKey::BLOCK`]
/// (`0x7C00`) block; the full keycode is [`raw`](QuantumKey::raw).
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuantumKey(pub u8);

keycode_block! {
    QuantumKey, block 0x7C00, category "Quantum",
    {
        0x00 => "QK_BOOT", QK_BOOTLOADER,
        0x01 => "QK_RBT", QK_REBOOT,
        0x02 => "DB_TOGG", QK_DEBUG_TOGGLE,
        0x03 => "EE_CLR", QK_CLEAR_EEPROM,
        0x04 => "QK_MAKE", QK_MAKE,

        0x10 => "AS_DOWN", QK_AUTO_SHIFT_DOWN,
        0x11 => "AS_UP", QK_AUTO_SHIFT_UP,
        0x12 => "AS_RPT", QK_AUTO_SHIFT_REPORT,
        0x13 => "AS_ON", QK_AUTO_SHIFT_ON,
        0x14 => "AS_OFF", QK_AUTO_SHIFT_OFF,
        0x15 => "AS_TOGG", QK_AUTO_SHIFT_TOGGLE,
        0x16 => "QK_GESC", QK_GRAVE_ESCAPE,
        0x17 => "VK_TOGG", QK_VELOCIKEY_TOGGLE,
        0x18 => "SC_LCPO", QK_SPACE_CADET_LEFT_CTRL_PARENTHESIS_OPEN,
        0x19 => "SC_RCPC", QK_SPACE_CADET_RIGHT_CTRL_PARENTHESIS_CLOSE,
        0x1A => "SC_LSPO", QK_SPACE_CADET_LEFT_SHIFT_PARENTHESIS_OPEN,
        0x1B => "SC_RSPC", QK_SPACE_CADET_RIGHT_SHIFT_PARENTHESIS_CLOSE,
        0x1C => "SC_LAPO", QK_SPACE_CADET_LEFT_ALT_PARENTHESIS_OPEN,
        0x1D => "SC_RAPC", QK_SPACE_CADET_RIGHT_ALT_PARENTHESIS_CLOSE,
        0x1E => "SC_SENT", QK_SPACE_CADET_RIGHT_SHIFT_ENTER,

        0x30 => "UC_NEXT", QK_UNICODE_MODE_NEXT,
        0x31 => "UC_PREV", QK_UNICODE_MODE_PREVIOUS,
        0x32 => "UC_MAC", QK_UNICODE_MODE_MACOS,
        0x33 => "UC_LINX", QK_UNICODE_MODE_LINUX,
        0x34 => "UC_WIN", QK_UNICODE_MODE_WINDOWS,
        0x35 => "UC_BSD", QK_UNICODE_MODE_BSD,
        0x36 => "UC_WINC", QK_UNICODE_MODE_WINCOMPOSE,
        0x37 => "UC_EMAC", QK_UNICODE_MODE_EMACS,

        0x40 => "HF_ON", QK_HAPTIC_ON,
        0x41 => "HF_OFF", QK_HAPTIC_OFF,
        0x42 => "HF_TOGG", QK_HAPTIC_TOGGLE,
        0x43 => "HF_RST", QK_HAPTIC_RESET,
        0x44 => "HF_FDBK", QK_HAPTIC_FEEDBACK_TOGGLE,
        0x45 => "HF_BUZZ", QK_HAPTIC_BUZZ_TOGGLE,
        0x46 => "HF_NEXT", QK_HAPTIC_MODE_NEXT,
        0x47 => "HF_PREV", QK_HAPTIC_MODE_PREVIOUS,
        0x48 => "HF_CONT", QK_HAPTIC_CONTINUOUS_TOGGLE,
        0x49 => "HF_CONU", QK_HAPTIC_CONTINUOUS_UP,
        0x4A => "HF_COND", QK_HAPTIC_CONTINUOUS_DOWN,
        0x4B => "HF_DWLU", QK_HAPTIC_DWELL_UP,
        0x4C => "HF_DWLD", QK_HAPTIC_DWELL_DOWN,

        0x50 => "CM_ON", QK_COMBO_ON,
        0x51 => "CM_OFF", QK_COMBO_OFF,
        0x52 => "CM_TOGG", QK_COMBO_TOGGLE,
        0x53 => "DM_REC1", QK_DYNAMIC_MACRO_RECORD_START_1,
        0x54 => "DM_REC2", QK_DYNAMIC_MACRO_RECORD_START_2,
        0x55 => "DM_RSTP", QK_DYNAMIC_MACRO_RECORD_STOP,
        0x56 => "DM_PLY1", QK_DYNAMIC_MACRO_PLAY_1,
        0x57 => "DM_PLY2", QK_DYNAMIC_MACRO_PLAY_2,
        0x58 => "QK_LEAD", QK_LEADER,
        0x59 => "QK_LOCK", QK_LOCK,
        0x5A => "OS_ON", QK_ONE_SHOT_ON,
        0x5B => "OS_OFF", QK_ONE_SHOT_OFF,
        0x5C => "OS_TOGG", QK_ONE_SHOT_TOGGLE,
        0x5D => "KO_TOGG", QK_KEY_OVERRIDE_TOGGLE,
        0x5E => "KO_ON", QK_KEY_OVERRIDE_ON,
        0x5F => "KO_OFF", QK_KEY_OVERRIDE_OFF,

        0x60 => "SE_LOCK", QK_SECURE_LOCK,
        0x61 => "SE_UNLK", QK_SECURE_UNLOCK,
        0x62 => "SE_TOGG", QK_SECURE_TOGGLE,
        0x63 => "SE_REQ", QK_SECURE_REQUEST,

        0x70 => "DT_PRNT", QK_DYNAMIC_TAPPING_TERM_PRINT,
        0x71 => "DT_UP", QK_DYNAMIC_TAPPING_TERM_UP,
        0x72 => "DT_DOWN", QK_DYNAMIC_TAPPING_TERM_DOWN,
        0x73 => "CW_TOGG", QK_CAPS_WORD_TOGGLE,
        0x74 => "AC_ON", QK_AUTOCORRECT_ON,
        0x75 => "AC_OFF", QK_AUTOCORRECT_OFF,
        0x76 => "AC_TOGG", QK_AUTOCORRECT_TOGGLE,
        0x77 => "TL_LOWR", QK_TRI_LAYER_LOWER,
        0x78 => "TL_UPPR", QK_TRI_LAYER_UPPER,
        0x79 => "QK_REP", QK_REPEAT_KEY,
        0x7A => "QK_AREP", QK_ALT_REPEAT_KEY,
        0x7B => "QK_LLCK", QK_LAYER_LOCK,
    }
}

impl std::fmt::Display for QuantumKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name())
    }
}
