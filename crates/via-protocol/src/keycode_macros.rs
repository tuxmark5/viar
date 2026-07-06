//! Macro for keycode "block" newtypes whose constants' identifiers double as
//! their canonical names (e.g. `RgbKey`, `QuantumKey`).

/// Generate a block newtype's `BLOCK`, constants, `raw()`, `qmk_name()`,
/// `name()` and `description()` from a table of `IDENT = offset` entries. Each
/// constant's identifier *is* its canonical QMK name, so `qmk_name()` /
/// `name()` come straight from `stringify!`. Comments and blank lines may
/// separate entries for grouping.
macro_rules! keycode_block {
    (
        $t:ty, block $block:literal, category $cat:literal,
        { $( $id:ident = $off:literal ),+ $(,)? }
    ) => {
        impl $t {
            /// Base of this keycode block; the full keycode is `BLOCK | offset`.
            pub const BLOCK: u16 = $block;

            $( pub const $id: Self = Self($off); )+

            /// The full `u16` keycode value (`BLOCK | offset`).
            pub const fn raw(self) -> u16 {
                Self::BLOCK | self.0 as u16
            }

            /// The canonical QMK name, e.g. `QK_UNDERGLOW_TOGGLE`.
            pub fn qmk_name(self) -> Option<&'static str> {
                Some(match self.0 {
                    $( $off => stringify!($id), )+
                    _ => return None,
                })
            }

            /// Short display name (the canonical name; hex fallback).
            pub fn name(self) -> String {
                self.qmk_name()
                    .map(str::to_string)
                    .unwrap_or_else(|| format!("0x{:04X}", self.raw()))
            }

            /// Longer human description for tooltips.
            pub fn description(self) -> String {
                match self.qmk_name() {
                    Some(n) => format!("{}: {n}", $cat),
                    None => format!("{} keycode (0x{:04X})", $cat, self.raw()),
                }
            }
        }
    };
}

pub(crate) use keycode_block;
