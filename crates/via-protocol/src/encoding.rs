//! Keycode encoding schemes.
//!
//! VIA/QMK changed the numeric values of its *quantum* keycodes (layer, mod-tap,
//! …) at VIA protocol version 12. Basic keycodes (`0x00–0xFF`), plain modifiers
//! (`0x0100–0x1FFF`) and layer-tap (`0x4000`) are identical across schemes; only
//! the range bases below differ.
//!
//! A [`KeycodeEncoding`] turns a raw `u16` from the device into a [`KeyAction`]
//! ([`decode`](KeycodeEncoding::decode)) and back
//! ([`encode`](KeycodeEncoding::encode)). Because the in-memory keymap stores
//! `KeyAction`s, copying a key from a board using one scheme and pasting it to a
//! board using another re-encodes correctly.

use crate::{
    basic_key::BasicKey,
    key_action::{
        KeyAction,
        LayerId,
    },
    mod_mask::ModMask,
    quantum_key::QuantumKey,
    rgb_key::RgbKey,
    tap_dance_key::TapDanceKey,
};

/// The per-scheme range bases the encoding differs on. Everything else about
/// decode/encode is shared logic below.
struct Ranges {
    mod_tap: u16,
    layer_tap: u16,
    to: u16,
    /// Layer mask for `TO`: old folds the ON_PRESS bit into the base so only the
    /// low nibble is the layer (`0xF`); new uses the low five bits (`0x1F`).
    to_mask: u16,
    momentary: u16,
    def_layer: u16,
    toggle_layer: u16,
    one_shot_layer: u16,
    one_shot_mod: u16,
    tap_toggle: u16,
    layer_mod: u16,
    /// Modifier mask for `LayerMod` (`0xF` old, `0x1F` new); the layer is the
    /// four bits above it.
    layer_mod_mask: u16,
}

/// Old scheme (VIA protocol ≤ 11 / Vial).
static OLD: Ranges = Ranges {
    mod_tap: 0x6000,
    layer_tap: 0x4000,
    to: 0x5010,
    to_mask: 0x000F,
    momentary: 0x5100,
    def_layer: 0x5200,
    toggle_layer: 0x5300,
    one_shot_layer: 0x5400,
    one_shot_mod: 0x5500,
    tap_toggle: 0x5800,
    layer_mod: 0x5900,
    layer_mod_mask: 0x000F,
};

/// New scheme (VIA protocol ≥ 12 / mainline QMK).
static NEW: Ranges = Ranges {
    mod_tap: 0x2000,
    layer_tap: 0x4000,
    to: 0x5200,
    to_mask: 0x001F,
    momentary: 0x5220,
    def_layer: 0x5240,
    toggle_layer: 0x5260,
    one_shot_layer: 0x5280,
    one_shot_mod: 0x52A0,
    tap_toggle: 0x52C0,
    layer_mod: 0x5000,
    layer_mod_mask: 0x001F,
};

/// A keycode numbering scheme: converts between raw `u16` device values and
/// encoding-independent [`KeyAction`]s.
pub trait KeycodeEncoding: Sync {
    fn decode(&self, raw: u16) -> KeyAction;
    fn encode(&self, action: KeyAction) -> u16;
}

/// A handle to one of the (static, zero-sized) encodings, cheap to copy and store.
pub type KeycodeEncodingRef = &'static dyn KeycodeEncoding;

/// Old VIA/Vial scheme (protocol ≤ 11).
pub struct OldEncoding;
/// New mainline-QMK scheme (protocol ≥ 12).
pub struct NewEncoding;

/// Pick the encoding for a keyboard's VIA protocol version. VIA switched to the
/// new scheme at protocol 12. (Rust promotes the borrowed ZSTs to `'static`.)
pub fn encoding_for_protocol(protocol_version: Option<u16>) -> KeycodeEncodingRef {
    match protocol_version {
        Some(v) if v >= 12 => &NewEncoding,
        _ => &OldEncoding,
    }
}

impl KeycodeEncoding for OldEncoding {
    fn decode(&self, raw: u16) -> KeyAction {
        decode_with(&OLD, raw)
    }

    fn encode(&self, action: KeyAction) -> u16 {
        encode_with(&OLD, action)
    }
}

impl KeycodeEncoding for NewEncoding {
    fn decode(&self, raw: u16) -> KeyAction {
        decode_with(&NEW, raw)
    }

    fn encode(&self, action: KeyAction) -> u16 {
        encode_with(&NEW, action)
    }
}

/// Layer ops (MO/DF/TG/OSL/OSM/TT) span 32 values.
const LAYER_BLOCK: u16 = 0x20;

fn layer_mod_span(mask: u16) -> u16 {
    // 4-bit layer above the mask bits, plus the mask itself.
    ((0xF << mask.count_ones()) | mask) + 1
}

fn decode_with(r: &Ranges, raw: u16) -> KeyAction {
    use KeyAction::*;
    // Offset of `raw` within the block starting at `base`, if it falls inside it.
    let block = |base: u16, size: u16| raw.checked_sub(base).filter(|&v| v < size);

    if raw <= 0x00FF {
        Basic(BasicKey(raw as u8))
    } else if (0x0100..=0x1FFF).contains(&raw) {
        Modified {
            mods: ModMask(((raw >> 8) & 0x1F) as u8),
            key:  BasicKey((raw & 0xFF) as u8),
        }
    } else if let Some(v) = block(r.mod_tap, 0x2000) {
        ModTap {
            mods: ModMask(((v >> 8) & 0x1F) as u8),
            key:  BasicKey((v & 0xFF) as u8),
        }
    } else if let Some(v) = block(r.layer_tap, 0x1000) {
        LayerTap {
            layer: LayerId(((v >> 8) & 0x0F) as u8),
            key:   BasicKey((v & 0xFF) as u8),
        }
    } else if let Some(v) = block(r.to, r.to_mask + 1) {
        ToLayer(LayerId(v as u8))
    } else if let Some(v) = block(r.momentary, LAYER_BLOCK) {
        Momentary(LayerId(v as u8))
    } else if let Some(v) = block(r.def_layer, LAYER_BLOCK) {
        DefLayer(LayerId(v as u8))
    } else if let Some(v) = block(r.toggle_layer, LAYER_BLOCK) {
        ToggleLayer(LayerId(v as u8))
    } else if let Some(v) = block(r.one_shot_layer, LAYER_BLOCK) {
        OneShotLayer(LayerId(v as u8))
    } else if let Some(v) = block(r.one_shot_mod, LAYER_BLOCK) {
        OneShotMod(ModMask(v as u8))
    } else if let Some(v) = block(r.tap_toggle, LAYER_BLOCK) {
        TapToggleLayer(LayerId(v as u8))
    } else if let Some(v) = block(r.layer_mod, layer_mod_span(r.layer_mod_mask)) {
        let bits = r.layer_mod_mask.count_ones();
        LayerMod {
            layer: LayerId(((v >> bits) & 0xF) as u8),
            mods:  ModMask((v & r.layer_mod_mask) as u8),
        }
    } else if let Some(v) = block(TapDanceKey::BLOCK, 0x100) {
        // Tap-dance is scheme-independent (0x5700 is free in both schemes).
        TapDance(TapDanceKey(v as u8))
    } else if let Some(v) = block(RgbKey::BLOCK, 0x100) {
        // Only reached in the new scheme; in the old scheme 0x7800 is mod-tap.
        Rgb(RgbKey(v as u8))
    } else if let Some(v) = block(QuantumKey::BLOCK, 0x100) {
        // Only reached in the new scheme; in the old scheme 0x7C00 is mod-tap.
        Quantum(QuantumKey(v as u8))
    } else {
        Raw(raw)
    }
}

fn encode_with(r: &Ranges, action: KeyAction) -> u16 {
    use KeyAction::*;
    match action {
        Basic(k) => k.0 as u16,
        Modified { mods, key } => ((mods.0 as u16 & 0x1F) << 8) | key.0 as u16,
        ModTap { mods, key } => r.mod_tap | ((mods.0 as u16 & 0x1F) << 8) | key.0 as u16,
        LayerTap { layer, key } => r.layer_tap | ((layer.0 as u16 & 0x0F) << 8) | key.0 as u16,
        LayerMod { layer, mods } => {
            let bits = r.layer_mod_mask.count_ones();
            r.layer_mod | ((layer.0 as u16 & 0xF) << bits) | (mods.0 as u16 & r.layer_mod_mask)
        }
        Momentary(l) => r.momentary | (l.0 as u16 & 0x1F),
        ToggleLayer(l) => r.toggle_layer | (l.0 as u16 & 0x1F),
        DefLayer(l) => r.def_layer | (l.0 as u16 & 0x1F),
        ToLayer(l) => r.to | (l.0 as u16 & r.to_mask),
        OneShotLayer(l) => r.one_shot_layer | (l.0 as u16 & 0x1F),
        OneShotMod(m) => r.one_shot_mod | (m.0 as u16 & 0x1F),
        TapToggleLayer(l) => r.tap_toggle | (l.0 as u16 & 0x1F),
        Rgb(k) => k.raw(),
        TapDance(k) => k.raw(),
        Quantum(k) => k.raw(),
        Raw(raw) => raw,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn old_scheme_layer_keycodes() {
        let e = &OldEncoding;
        // Data points reported from real Vial hardware.
        assert_eq!(e.decode(0x540B), KeyAction::OneShotLayer(LayerId(11)));
        assert_eq!(e.encode(KeyAction::OneShotLayer(LayerId(11))), 0x540B);
        assert_eq!(e.decode(0x5010), KeyAction::ToLayer(LayerId(0)));
        assert_eq!(e.decode(0x501A), KeyAction::ToLayer(LayerId(10)));
        assert_eq!(e.encode(KeyAction::ToLayer(LayerId(10))), 0x501A);
        assert_eq!(e.decode(0x5100), KeyAction::Momentary(LayerId(0)));
    }

    #[test]
    fn new_scheme_layer_keycodes() {
        let e = &NewEncoding;
        assert_eq!(e.decode(0x528B), KeyAction::OneShotLayer(LayerId(11)));
        assert_eq!(e.encode(KeyAction::OneShotLayer(LayerId(11))), 0x528B);
        assert_eq!(e.decode(0x5200), KeyAction::ToLayer(LayerId(0)));
        assert_eq!(e.encode(KeyAction::ToLayer(LayerId(0))), 0x5200);
    }

    #[test]
    fn shared_ranges_match_in_both_schemes() {
        for e in [
            &OldEncoding as &dyn KeycodeEncoding,
            &NewEncoding as &dyn KeycodeEncoding,
        ] {
            assert_eq!(e.decode(0x0004), KeyAction::Basic(BasicKey(0x04))); // KC_A
            assert_eq!(e.encode(KeyAction::Basic(BasicKey(0x04))), 0x0004);
            // LT(1, KC_A) = 0x4104 in both schemes.
            assert_eq!(
                e.decode(0x4104),
                KeyAction::LayerTap {
                    layer: LayerId(1),
                    key:   BasicKey(0x04),
                }
            );
            assert_eq!(
                e.encode(KeyAction::LayerTap {
                    layer: LayerId(1),
                    key:   BasicKey(0x04),
                }),
                0x4104
            );
        }
    }

    #[test]
    fn round_trips_within_each_scheme() {
        for e in [
            &OldEncoding as &dyn KeycodeEncoding,
            &NewEncoding as &dyn KeycodeEncoding,
        ] {
            for raw in 0u16..=0x7FFF {
                let back = e.encode(e.decode(raw));
                assert_eq!(back, raw, "raw={raw:#06x} action={:?}", e.decode(raw));
            }
        }
    }

    #[test]
    fn cross_scheme_reencode() {
        // Copy OSL(11) off an old board, paste onto a new board.
        let action = OldEncoding.decode(0x540B);
        assert_eq!(action, KeyAction::OneShotLayer(LayerId(11)));
        assert_eq!(NewEncoding.encode(action), 0x528B);
    }

    #[test]
    fn lighting_block_decodes_to_rgb_new_scheme() {
        // The 0x7800 lighting block is only free in the new scheme; RgbKey stores
        // the offset within it.
        let e = &NewEncoding;
        assert_eq!(e.decode(0x7800), KeyAction::Rgb(RgbKey::QK_BACKLIGHT_ON));
        assert_eq!(e.encode(KeyAction::Rgb(RgbKey::QK_BACKLIGHT_ON)), 0x7800);
        assert_eq!(
            e.decode(0x7820),
            KeyAction::Rgb(RgbKey::QK_UNDERGLOW_TOGGLE)
        );
        assert_eq!(
            e.encode(KeyAction::Rgb(RgbKey::QK_UNDERGLOW_TOGGLE)),
            0x7820
        );
        assert_eq!(
            e.decode(0x7842),
            KeyAction::Rgb(RgbKey::QK_RGB_MATRIX_TOGGLE)
        );
        // In the old scheme 0x6000..=0x7FFF is mod-tap, so 0x7800 is NOT lighting.
        assert!(matches!(
            OldEncoding.decode(0x7800),
            KeyAction::ModTap { .. }
        ));
    }

    #[test]
    fn tap_dance_and_quantum_blocks() {
        // Tap-dance (0x5700) is scheme-independent.
        for e in [
            &OldEncoding as &dyn KeycodeEncoding,
            &NewEncoding as &dyn KeycodeEncoding,
        ] {
            assert_eq!(e.decode(0x5703), KeyAction::TapDance(TapDanceKey(3)));
            assert_eq!(e.encode(KeyAction::TapDance(TapDanceKey(3))), 0x5703);
        }
        // Quantum (0x7C00) is only free in the new scheme; old = mod-tap.
        let e = &NewEncoding;
        assert_eq!(
            e.decode(0x7C00),
            KeyAction::Quantum(QuantumKey::QK_BOOTLOADER)
        );
        assert_eq!(
            e.encode(KeyAction::Quantum(QuantumKey::QK_BOOTLOADER)),
            0x7C00
        );
        assert!(matches!(
            OldEncoding.decode(0x7C00),
            KeyAction::ModTap { .. }
        ));
    }

    #[test]
    fn protocol_selection() {
        assert_eq!(
            encoding_for_protocol(Some(9)).decode(0x540B),
            KeyAction::OneShotLayer(LayerId(11))
        );
        assert_eq!(
            encoding_for_protocol(Some(12)).decode(0x528B),
            KeyAction::OneShotLayer(LayerId(11))
        );
        assert_eq!(
            encoding_for_protocol(None).decode(0x540B),
            KeyAction::OneShotLayer(LayerId(11))
        );
    }
}
