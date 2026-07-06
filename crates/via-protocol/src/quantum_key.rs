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
        QK_BOOTLOADER = 0x00,
        QK_REBOOT = 0x01,
        QK_DEBUG_TOGGLE = 0x02,
        QK_CLEAR_EEPROM = 0x03,
        QK_MAKE = 0x04,

        QK_AUTO_SHIFT_DOWN = 0x10,
        QK_AUTO_SHIFT_UP = 0x11,
        QK_AUTO_SHIFT_REPORT = 0x12,
        QK_AUTO_SHIFT_ON = 0x13,
        QK_AUTO_SHIFT_OFF = 0x14,
        QK_AUTO_SHIFT_TOGGLE = 0x15,
        QK_GRAVE_ESCAPE = 0x16,
        QK_VELOCIKEY_TOGGLE = 0x17,
        QK_SPACE_CADET_LEFT_CTRL_PARENTHESIS_OPEN = 0x18,
        QK_SPACE_CADET_RIGHT_CTRL_PARENTHESIS_CLOSE = 0x19,
        QK_SPACE_CADET_LEFT_SHIFT_PARENTHESIS_OPEN = 0x1A,
        QK_SPACE_CADET_RIGHT_SHIFT_PARENTHESIS_CLOSE = 0x1B,
        QK_SPACE_CADET_LEFT_ALT_PARENTHESIS_OPEN = 0x1C,
        QK_SPACE_CADET_RIGHT_ALT_PARENTHESIS_CLOSE = 0x1D,
        QK_SPACE_CADET_RIGHT_SHIFT_ENTER = 0x1E,

        QK_UNICODE_MODE_NEXT = 0x30,
        QK_UNICODE_MODE_PREVIOUS = 0x31,
        QK_UNICODE_MODE_MACOS = 0x32,
        QK_UNICODE_MODE_LINUX = 0x33,
        QK_UNICODE_MODE_WINDOWS = 0x34,
        QK_UNICODE_MODE_BSD = 0x35,
        QK_UNICODE_MODE_WINCOMPOSE = 0x36,
        QK_UNICODE_MODE_EMACS = 0x37,

        QK_HAPTIC_ON = 0x40,
        QK_HAPTIC_OFF = 0x41,
        QK_HAPTIC_TOGGLE = 0x42,
        QK_HAPTIC_RESET = 0x43,
        QK_HAPTIC_FEEDBACK_TOGGLE = 0x44,
        QK_HAPTIC_BUZZ_TOGGLE = 0x45,
        QK_HAPTIC_MODE_NEXT = 0x46,
        QK_HAPTIC_MODE_PREVIOUS = 0x47,
        QK_HAPTIC_CONTINUOUS_TOGGLE = 0x48,
        QK_HAPTIC_CONTINUOUS_UP = 0x49,
        QK_HAPTIC_CONTINUOUS_DOWN = 0x4A,
        QK_HAPTIC_DWELL_UP = 0x4B,
        QK_HAPTIC_DWELL_DOWN = 0x4C,

        QK_COMBO_ON = 0x50,
        QK_COMBO_OFF = 0x51,
        QK_COMBO_TOGGLE = 0x52,
        QK_DYNAMIC_MACRO_RECORD_START_1 = 0x53,
        QK_DYNAMIC_MACRO_RECORD_START_2 = 0x54,
        QK_DYNAMIC_MACRO_RECORD_STOP = 0x55,
        QK_DYNAMIC_MACRO_PLAY_1 = 0x56,
        QK_DYNAMIC_MACRO_PLAY_2 = 0x57,
        QK_LEADER = 0x58,
        QK_LOCK = 0x59,
        QK_ONE_SHOT_ON = 0x5A,
        QK_ONE_SHOT_OFF = 0x5B,
        QK_ONE_SHOT_TOGGLE = 0x5C,
        QK_KEY_OVERRIDE_TOGGLE = 0x5D,
        QK_KEY_OVERRIDE_ON = 0x5E,
        QK_KEY_OVERRIDE_OFF = 0x5F,

        QK_SECURE_LOCK = 0x60,
        QK_SECURE_UNLOCK = 0x61,
        QK_SECURE_TOGGLE = 0x62,
        QK_SECURE_REQUEST = 0x63,

        QK_DYNAMIC_TAPPING_TERM_PRINT = 0x70,
        QK_DYNAMIC_TAPPING_TERM_UP = 0x71,
        QK_DYNAMIC_TAPPING_TERM_DOWN = 0x72,
        QK_CAPS_WORD_TOGGLE = 0x73,
        QK_AUTOCORRECT_ON = 0x74,
        QK_AUTOCORRECT_OFF = 0x75,
        QK_AUTOCORRECT_TOGGLE = 0x76,
        QK_TRI_LAYER_LOWER = 0x77,
        QK_TRI_LAYER_UPPER = 0x78,
        QK_REPEAT_KEY = 0x79,
        QK_ALT_REPEAT_KEY = 0x7A,
        QK_LAYER_LOCK = 0x7B,
    }
}

impl std::fmt::Display for QuantumKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name())
    }
}
