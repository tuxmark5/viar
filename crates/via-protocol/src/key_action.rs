//! The decoded, encoding-independent meaning of a keycode.
//!
//! A [`KeyAction`] is what a raw `u16` keycode *means* (`OneShotLayer(11)`,
//! `ModTap { .. }`, `Basic(KC_A)`), independent of the numeric scheme the
//! keyboard uses on the wire. Conversion to/from raw values lives in
//! [`crate::encoding`]. The raw [`Keycode`] newtype stays around as a naming
//! helper for basic/raw values that a `KeyAction` delegates to.

use crate::keycodes::{
    Keycode,
    KeycodeCategory,
    mod_mask_to_string,
    mod_tap_prefix,
};

/// A decoded keycode action, independent of the numeric encoding scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyAction {
    /// Basic HID keycode `0x00–0xFF` (letters, mods, media, mouse, NONE/TRNS).
    Basic(u8),
    /// A basic key with modifiers applied, e.g. `C(KC_A)`.
    Modified { mods: u8, key: u8 },
    /// Mod-tap: tap emits `key`, hold acts as `mods`.
    ModTap { mods: u8, key: u8 },
    /// Layer-tap: tap emits `key`, hold activates `layer`.
    LayerTap { layer: u8, key: u8 },
    /// Activate `layer` with `mods` applied.
    LayerMod { layer: u8, mods: u8 },
    /// MO(layer) — momentary layer.
    Momentary(u8),
    /// TG(layer) — toggle layer.
    ToggleLayer(u8),
    /// DF(layer) — set default layer.
    DefLayer(u8),
    /// TO(layer) — activate layer, deactivating others.
    ToLayer(u8),
    /// OSL(layer) — one-shot layer.
    OneShotLayer(u8),
    /// OSM(mods) — one-shot modifier.
    OneShotMod(u8),
    /// TT(layer) — layer tap-toggle.
    TapToggleLayer(u8),
    /// Any keycode not modelled above (magic/lighting/quantum/macro/custom/…),
    /// kept as its raw value.
    Raw(u16),
}

impl Default for KeyAction {
    /// `KC_NO` — an empty / unassigned slot.
    fn default() -> Self {
        KeyAction::Basic(0)
    }
}

impl KeyAction {
    /// Whether this is an empty slot — `KC_NO` (transparent nothing) or
    /// `KC_TRANSPARENT` (falls through to the layer below).
    pub fn is_empty(self) -> bool {
        matches!(self, KeyAction::Basic(0) | KeyAction::Basic(1))
    }

    /// Human-readable name (e.g. `KC_A`, `OSL(11)`, `LT(1,KC_SPC)`).
    pub fn name(self) -> String {
        match self {
            // Basic and raw naming is scheme-independent — reuse the legacy tables.
            KeyAction::Basic(k) => Keycode(k as u16).name(),
            KeyAction::Raw(raw) => Keycode(raw).name(),
            KeyAction::Modified { mods, key } => {
                format!(
                    "{}({})",
                    mod_mask_to_string(mods),
                    Keycode(key as u16).name()
                )
            }
            KeyAction::ModTap { mods, key } => {
                format!("{}({})", mod_tap_prefix(mods), Keycode(key as u16).name())
            }
            KeyAction::LayerTap { layer, key } => {
                format!("LT({layer},{})", Keycode(key as u16).name())
            }
            KeyAction::LayerMod { layer, mods } => {
                format!("LM({layer},{})", mod_mask_to_string(mods))
            }
            KeyAction::Momentary(l) => format!("MO({l})"),
            KeyAction::ToggleLayer(l) => format!("TG({l})"),
            KeyAction::DefLayer(l) => format!("DF({l})"),
            KeyAction::ToLayer(l) => format!("TO({l})"),
            KeyAction::OneShotLayer(l) => format!("OSL({l})"),
            KeyAction::OneShotMod(m) => format!("OSM({})", mod_mask_to_string(m)),
            KeyAction::TapToggleLayer(l) => format!("TT({l})"),
        }
    }

    /// Broad category, for picker grouping and keycap coloring.
    pub fn category(self) -> KeycodeCategory {
        match self {
            // Basic/raw values self-classify via the legacy tables.
            KeyAction::Basic(k) => Keycode(k as u16).category(),
            KeyAction::Raw(raw) => Keycode(raw).category(),
            KeyAction::Modified { .. } => KeycodeCategory::Mod,
            KeyAction::ModTap { .. } => KeycodeCategory::ModTap,
            KeyAction::LayerTap { .. } => KeycodeCategory::LayerTap,
            KeyAction::LayerMod { .. } => KeycodeCategory::LayerMod,
            KeyAction::Momentary(_) => KeycodeCategory::LayerMomentary,
            KeyAction::ToggleLayer(_) => KeycodeCategory::LayerToggle,
            KeyAction::DefLayer(_) => KeycodeCategory::LayerDefault,
            KeyAction::ToLayer(_) => KeycodeCategory::LayerOn,
            KeyAction::OneShotLayer(_) => KeycodeCategory::LayerOneShotLayer,
            KeyAction::OneShotMod(_) => KeycodeCategory::LayerOneShotMod,
            KeyAction::TapToggleLayer(_) => KeycodeCategory::LayerTapToggle,
        }
    }

    /// For dual-function keys, the `(tap, hold)` labels for a split keycap;
    /// `None` for simple keys.
    pub fn dual_labels(self) -> Option<(String, String)> {
        match self {
            KeyAction::ModTap { mods, key } => {
                Some((Keycode(key as u16).name(), mod_tap_prefix(mods).to_string()))
            }
            KeyAction::LayerTap { layer, key } => {
                Some((Keycode(key as u16).name(), format!("LT{layer}")))
            }
            KeyAction::LayerMod { layer, mods } => {
                Some((format!("LM{layer}"), mod_mask_to_string(mods)))
            }
            KeyAction::TapToggleLayer(l) => Some((format!("TT{l}"), format!("L{l}"))),
            KeyAction::OneShotLayer(l) => Some(("OSL".to_string(), format!("L{l}"))),
            KeyAction::OneShotMod(m) => Some(("OSM".to_string(), mod_mask_to_string(m))),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_layer_and_basic_actions() {
        assert_eq!(KeyAction::OneShotLayer(11).name(), "OSL(11)");
        assert_eq!(KeyAction::ToLayer(3).name(), "TO(3)");
        assert_eq!(KeyAction::Basic(0x04).name(), "A");
    }
}
