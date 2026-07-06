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
        QK_BACKLIGHT_ON = 0x00,
        QK_BACKLIGHT_OFF = 0x01,
        QK_BACKLIGHT_TOGGLE = 0x02,
        QK_BACKLIGHT_DOWN = 0x03,
        QK_BACKLIGHT_UP = 0x04,
        QK_BACKLIGHT_STEP = 0x05,
        QK_BACKLIGHT_TOGGLE_BREATHING = 0x06,

        QK_LED_MATRIX_ON = 0x10,
        QK_LED_MATRIX_OFF = 0x11,
        QK_LED_MATRIX_TOGGLE = 0x12,
        QK_LED_MATRIX_MODE_NEXT = 0x13,
        QK_LED_MATRIX_MODE_PREVIOUS = 0x14,
        QK_LED_MATRIX_BRIGHTNESS_UP = 0x15,
        QK_LED_MATRIX_BRIGHTNESS_DOWN = 0x16,
        QK_LED_MATRIX_SPEED_UP = 0x17,
        QK_LED_MATRIX_SPEED_DOWN = 0x18,

        QK_UNDERGLOW_TOGGLE = 0x20,
        QK_UNDERGLOW_MODE_NEXT = 0x21,
        QK_UNDERGLOW_MODE_PREVIOUS = 0x22,
        QK_UNDERGLOW_HUE_UP = 0x23,
        QK_UNDERGLOW_HUE_DOWN = 0x24,
        QK_UNDERGLOW_SATURATION_UP = 0x25,
        QK_UNDERGLOW_SATURATION_DOWN = 0x26,
        QK_UNDERGLOW_VALUE_UP = 0x27,
        QK_UNDERGLOW_VALUE_DOWN = 0x28,
        QK_UNDERGLOW_SPEED_UP = 0x29,
        QK_UNDERGLOW_SPEED_DOWN = 0x2A,
        RGB_MODE_PLAIN = 0x2B,
        RGB_MODE_BREATHE = 0x2C,
        RGB_MODE_RAINBOW = 0x2D,
        RGB_MODE_SWIRL = 0x2E,
        RGB_MODE_SNAKE = 0x2F,

        RGB_MODE_KNIGHT = 0x30,
        RGB_MODE_XMAS = 0x31,
        RGB_MODE_GRADIENT = 0x32,
        RGB_MODE_RGBTEST = 0x33,
        RGB_MODE_TWINKLE = 0x34,

        QK_RGB_MATRIX_ON = 0x40,
        QK_RGB_MATRIX_OFF = 0x41,
        QK_RGB_MATRIX_TOGGLE = 0x42,
        QK_RGB_MATRIX_MODE_NEXT = 0x43,
        QK_RGB_MATRIX_MODE_PREVIOUS = 0x44,
        QK_RGB_MATRIX_HUE_UP = 0x45,
        QK_RGB_MATRIX_HUE_DOWN = 0x46,
        QK_RGB_MATRIX_SATURATION_UP = 0x47,
        QK_RGB_MATRIX_SATURATION_DOWN = 0x48,
        QK_RGB_MATRIX_VALUE_UP = 0x49,
        QK_RGB_MATRIX_VALUE_DOWN = 0x4A,
        QK_RGB_MATRIX_SPEED_UP = 0x4B,
        QK_RGB_MATRIX_SPEED_DOWN = 0x4C,
    }
}

impl std::fmt::Display for RgbKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name())
    }
}
