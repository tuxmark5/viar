//! QMK lighting keycodes — backlight, LED matrix, RGB underglow and RGB matrix
//! (the `0x7800` block) — with the canonical names from `quantum/keycodes.h`.
//!
//! These are new-scheme (mainline QMK) values. In the old VIA/Vial scheme the
//! `0x7800` block is mod-tap, so lighting is only modelled for the new scheme.

use crate::keycode_macros::keycode_block;

/// A QMK lighting keycode: backlight / LED-matrix / RGB-underglow / RGB-matrix
/// control. Stored as the offset within the [`RgbKey::BLOCK`] (`0x7800`) block;
/// the full keycode is [`raw`](RgbKey::raw).
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RgbKey(pub u8);

keycode_block! {
    RgbKey, block 0x7800, category "Lighting",
    {
        0x00 => "BL_ON", QK_BACKLIGHT_ON,
        0x01 => "BL_OFF", QK_BACKLIGHT_OFF,
        0x02 => "BL_TOGG", QK_BACKLIGHT_TOGGLE,
        0x03 => "BL_DOWN", QK_BACKLIGHT_DOWN,
        0x04 => "BL_UP", QK_BACKLIGHT_UP,
        0x05 => "BL_STEP", QK_BACKLIGHT_STEP,
        0x06 => "BL_BRTG", QK_BACKLIGHT_TOGGLE_BREATHING,

        0x10 => "LM_ON", QK_LED_MATRIX_ON,
        0x11 => "LM_OFF", QK_LED_MATRIX_OFF,
        0x12 => "LM_TOGG", QK_LED_MATRIX_TOGGLE,
        0x13 => "LM_NEXT", QK_LED_MATRIX_MODE_NEXT,
        0x14 => "LM_PREV", QK_LED_MATRIX_MODE_PREVIOUS,
        0x15 => "LM_BRIU", QK_LED_MATRIX_BRIGHTNESS_UP,
        0x16 => "LM_BRID", QK_LED_MATRIX_BRIGHTNESS_DOWN,
        0x17 => "LM_SPDU", QK_LED_MATRIX_SPEED_UP,
        0x18 => "LM_SPDD", QK_LED_MATRIX_SPEED_DOWN,

        0x20 => "UG_TOGG", QK_UNDERGLOW_TOGGLE,
        0x21 => "UG_NEXT", QK_UNDERGLOW_MODE_NEXT,
        0x22 => "UG_PREV", QK_UNDERGLOW_MODE_PREVIOUS,
        0x23 => "UG_HUEU", QK_UNDERGLOW_HUE_UP,
        0x24 => "UG_HUED", QK_UNDERGLOW_HUE_DOWN,
        0x25 => "UG_SATU", QK_UNDERGLOW_SATURATION_UP,
        0x26 => "UG_SATD", QK_UNDERGLOW_SATURATION_DOWN,
        0x27 => "UG_VALU", QK_UNDERGLOW_VALUE_UP,
        0x28 => "UG_VALD", QK_UNDERGLOW_VALUE_DOWN,
        0x29 => "UG_SPDU", QK_UNDERGLOW_SPEED_UP,
        0x2A => "UG_SPDD", QK_UNDERGLOW_SPEED_DOWN,
        0x2B => "RGB_M_P", RGB_MODE_PLAIN,
        0x2C => "RGB_M_B", RGB_MODE_BREATHE,
        0x2D => "RGB_M_R", RGB_MODE_RAINBOW,
        0x2E => "RGB_M_SW", RGB_MODE_SWIRL,
        0x2F => "RGB_M_SN", RGB_MODE_SNAKE,

        0x30 => "RGB_M_K", RGB_MODE_KNIGHT,
        0x31 => "RGB_M_X", RGB_MODE_XMAS,
        0x32 => "RGB_M_G", RGB_MODE_GRADIENT,
        0x33 => "RGB_M_T", RGB_MODE_RGBTEST,
        0x34 => "RGB_M_TW", RGB_MODE_TWINKLE,

        0x40 => "RM_ON", QK_RGB_MATRIX_ON,
        0x41 => "RM_OFF", QK_RGB_MATRIX_OFF,
        0x42 => "RM_TOGG", QK_RGB_MATRIX_TOGGLE,
        0x43 => "RM_NEXT", QK_RGB_MATRIX_MODE_NEXT,
        0x44 => "RM_PREV", QK_RGB_MATRIX_MODE_PREVIOUS,
        0x45 => "RM_HUEU", QK_RGB_MATRIX_HUE_UP,
        0x46 => "RM_HUED", QK_RGB_MATRIX_HUE_DOWN,
        0x47 => "RM_SATU", QK_RGB_MATRIX_SATURATION_UP,
        0x48 => "RM_SATD", QK_RGB_MATRIX_SATURATION_DOWN,
        0x49 => "RM_VALU", QK_RGB_MATRIX_VALUE_UP,
        0x4A => "RM_VALD", QK_RGB_MATRIX_VALUE_DOWN,
        0x4B => "RM_SPDU", QK_RGB_MATRIX_SPEED_UP,
        0x4C => "RM_SPDD", QK_RGB_MATRIX_SPEED_DOWN,
    }
}

impl std::fmt::Display for RgbKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name())
    }
}
