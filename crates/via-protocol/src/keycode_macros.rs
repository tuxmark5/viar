//! Macro for keycode "block" newtypes whose constants' identifiers double as
//! their canonical names (e.g. `RgbKey`, `QuantumKey`).

/// Generate a block newtype's `BLOCK`, constants, `raw()`, `qmk_name()`,
/// `name()` and `description()` from a table of `0xOFF => SHORT, CANONICAL`
/// entries (`0xOFF` is the offset within the block). `CANONICAL` is both the
/// constant's identifier and its `qmk_name()`; `SHORT` is the user-facing
/// `name()` (the QMK short alias — canonical names are too long for
/// keycaps/buttons). Comments and blank lines may separate entries for grouping.
///
/// Entries may optionally carry a free-form description literal as a fourth
/// field (`0xOFF => SHORT, CANONICAL, "DESC"`); then `description()` returns that
/// text instead of the default `"{category}: {canonical}"`. A block is
/// all-or-nothing: either every entry has a description or none do.
macro_rules! keycode_block {
    // With a per-entry description literal.
    (
        $t:ty, block $block:literal, category $cat:literal,
        { $( $off:literal => $short:literal, $long:ident, $desc:literal ),+ $(,)? }
    ) => {
        keycode_block!(@common $t, block $block, category $cat, { $( $off => $short, $long ),+ });
        impl $t {
            /// Longer human description for tooltips.
            pub fn description(self) -> String {
                match self.0 {
                    $( $off => $desc.to_string(), )+
                    _ => format!("{} keycode (0x{:04X})", $cat, self.raw()),
                }
            }
        }
    };

    // Without descriptions — `description()` falls back to the canonical name.
    (
        $t:ty, block $block:literal, category $cat:literal,
        { $( $off:literal => $short:literal, $long:ident ),+ $(,)? }
    ) => {
        keycode_block!(@common $t, block $block, category $cat, { $( $off => $short, $long ),+ });
        impl $t {
            /// Longer human description for tooltips (the canonical name).
            pub fn description(self) -> String {
                match self.qmk_name() {
                    Some(n) => format!("{}: {n}", $cat),
                    None => format!("{} keycode (0x{:04X})", $cat, self.raw()),
                }
            }
        }
    };

    // Shared: everything except `description()`.
    (@common
        $t:ty, block $block:literal, category $cat:literal,
        { $( $off:literal => $short:literal, $long:ident ),+ }
    ) => {
        impl $t {
            /// Base of this keycode block; the full keycode is `BLOCK | offset`.
            pub const BLOCK: u16 = $block;

            $( pub const $long: Self = Self($off); )+

            /// The full `u16` keycode value (`BLOCK | offset`).
            pub const fn raw(self) -> u16 {
                Self::BLOCK | self.0 as u16
            }

            /// The canonical QMK name, e.g. `QK_UNDERGLOW_TOGGLE`.
            pub fn qmk_name(self) -> Option<&'static str> {
                Some(match self.0 {
                    $( $off => stringify!($long), )+
                    _ => return None,
                })
            }

            /// Short user-facing display name, e.g. `UG_TOGG` (hex fallback).
            pub fn name(self) -> String {
                match self.0 {
                    $( $off => $short.to_string(), )+
                    _ => format!("0x{:04X}", self.raw()),
                }
            }
        }
    };
}

pub(crate) use keycode_block;

/// Generate `BasicKey`'s constants, `name()` and `description()` from a table of
/// `0xVAL => "SHORT", CANONICAL, "DESC"` entries (`0xVAL` is the full HID code).
/// Unlike [`keycode_block`], the short display name is a free-form string (basic
/// keys use friendly mixed-case labels like `VolUp`, not idents), and each entry
/// carries an explicit description literal. `CANONICAL` is the constant's
/// identifier (its `KC_*` name). `qmk_name()` is kept separate (delegates to the
/// full `qmk_names` table).
macro_rules! basic_keys {
    ( $( $val:literal => $short:literal, $long:ident, $desc:literal ),+ $(,)? ) => {
        impl BasicKey {
            $( pub const $long: Self = Self($val); )+

            /// Short user-facing display name, e.g. `A`, `Bksp`, `VolUp`.
            pub fn name(self) -> String {
                match self.0 {
                    $( $val => $short, )+
                    v => return format!("0x{v:02X}"),
                }
                .to_string()
            }

            /// Longer human description for tooltips.
            pub fn description(self) -> String {
                match self.0 {
                    $( $val => $desc, )+
                    v => return format!("{} (0x{v:04X})", self.name()),
                }
                .to_string()
            }
        }
    };
}

pub(crate) use basic_keys;
