//! The decoded, encoding-independent meaning of a keycode.
//!
//! A [`KeyAction`] is what a raw `u16` keycode *means* (`OneShotLayer(11)`,
//! `ModTap { .. }`, `Basic(KC_A)`), independent of the numeric scheme the
//! keyboard uses on the wire. Conversion to/from raw values lives in
//! [`crate::encoding`]. The raw [`Keycode`] newtype stays around as a naming
//! helper for basic/raw values that a `KeyAction` delegates to.

use crate::{
    basic_key::BasicKey,
    keycodes::{
        Keycode,
        KeycodeCategory,
        mod_tap_prefix,
    },
    mod_mask::ModMask,
    quantum_key::QuantumKey,
    rgb_key::RgbKey,
    tap_dance_key::TapDanceKey,
};

/// A layer index.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LayerId(pub u8);

/// `Display` the wrapped byte directly, so it reads as a plain number in
/// formatted output (`MO(2)`, `LT(1,…)`).
impl std::fmt::Display for LayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A decoded keycode action, independent of the numeric encoding scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyAction {
    /// Basic HID keycode `0x00–0xFF` (letters, mods, media, mouse, NONE/TRNS).
    Basic(BasicKey),
    /// A basic key with modifiers applied, e.g. `C(KC_A)`.
    Modified { mods: ModMask, key: BasicKey },
    /// Mod-tap: tap emits `key`, hold acts as `mods`.
    ModTap { mods: ModMask, key: BasicKey },
    /// Layer-tap: tap emits `key`, hold activates `layer`.
    LayerTap { layer: LayerId, key: BasicKey },
    /// Activate `layer` with `mods` applied.
    LayerMod { layer: LayerId, mods: ModMask },
    /// MO(layer) — momentary layer.
    Momentary(LayerId),
    /// TG(layer) — toggle layer.
    ToggleLayer(LayerId),
    /// DF(layer) — set default layer.
    DefLayer(LayerId),
    /// TO(layer) — activate layer, deactivating others.
    ToLayer(LayerId),
    /// OSL(layer) — one-shot layer.
    OneShotLayer(LayerId),
    /// OSM(mods) — one-shot modifier.
    OneShotMod(ModMask),
    /// TT(layer) — layer tap-toggle.
    TapToggleLayer(LayerId),
    /// RGB underglow / backlight lighting keycode (`0x7800` block).
    Rgb(RgbKey),
    /// TD(n) — tap-dance keycode (`0x5700` block).
    TapDance(TapDanceKey),
    /// QMK quantum keycode — bootloader/audio/haptic/… (`0x7C00` block).
    Quantum(QuantumKey),
    /// Any keycode not modelled above (magic/macro/custom/…),
    /// kept as its raw value.
    Raw(u16),
}

impl Default for KeyAction {
    /// `KC_NO` — an empty / unassigned slot.
    fn default() -> Self {
        KeyAction::Basic(BasicKey(0))
    }
}

impl KeyAction {
    /// Whether this is an empty slot — `KC_NO` (transparent nothing) or
    /// `KC_TRANSPARENT` (falls through to the layer below).
    pub fn is_empty(self) -> bool {
        matches!(self, KeyAction::Basic(BasicKey(0 | 1)))
    }

    /// Human-readable name (e.g. `KC_A`, `OSL(11)`, `LT(1,KC_SPC)`).
    pub fn name(self) -> String {
        match self {
            KeyAction::Basic(k) => k.name(),
            // Raw naming (magic/lighting/quantum) still goes through the legacy table.
            KeyAction::Raw(raw) => Keycode(raw).name(),
            KeyAction::Modified { mods, key } => format!("{mods}({})", key.name()),
            KeyAction::ModTap { mods, key } => {
                format!("{}({})", mod_tap_prefix(mods.0), key.name())
            }
            KeyAction::LayerTap { layer, key } => format!("LT({layer},{})", key.name()),
            KeyAction::LayerMod { layer, mods } => format!("LM({layer},{mods})"),
            KeyAction::Momentary(l) => format!("MO({l})"),
            KeyAction::ToggleLayer(l) => format!("TG({l})"),
            KeyAction::DefLayer(l) => format!("DF({l})"),
            KeyAction::ToLayer(l) => format!("TO({l})"),
            KeyAction::OneShotLayer(l) => format!("OSL({l})"),
            KeyAction::OneShotMod(m) => format!("OSM({m})"),
            KeyAction::TapToggleLayer(l) => format!("TT({l})"),
            KeyAction::Rgb(k) => k.name(),
            KeyAction::TapDance(k) => k.name(),
            KeyAction::Quantum(k) => k.name(),
        }
    }

    /// Canonical QMK name (`KC_A`) for basic/raw keycodes; `None` for parametric
    /// actions (their name is composed, e.g. `OSL(11)`).
    pub fn qmk_name(self) -> Option<&'static str> {
        match self {
            KeyAction::Basic(k) => k.qmk_name(),
            KeyAction::Rgb(k) => k.qmk_name(),
            KeyAction::Quantum(k) => k.qmk_name(),
            KeyAction::Raw(raw) => Keycode(raw).qmk_name(),
            _ => None,
        }
    }

    /// Human description for basic/raw keycodes; empty for parametric actions
    /// (their [`name`](Self::name) already spells them out).
    pub fn description(self) -> String {
        match self {
            KeyAction::Basic(k) => k.description(),
            KeyAction::Rgb(k) => k.description(),
            KeyAction::TapDance(k) => k.description(),
            KeyAction::Quantum(k) => k.description(),
            KeyAction::Raw(raw) => Keycode(raw).description(),
            _ => String::new(),
        }
    }

    /// Broad category, for picker grouping and keycap coloring.
    pub fn category(self) -> KeycodeCategory {
        match self {
            // Basic/raw values self-classify via the legacy tables.
            KeyAction::Basic(k) => Keycode(k.0 as u16).category(),
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
            KeyAction::Rgb(_) => KeycodeCategory::Lighting,
            KeyAction::TapDance(_) => KeycodeCategory::TapDance,
            KeyAction::Quantum(_) => KeycodeCategory::Quantum,
        }
    }

    /// For dual-function keys, the `(tap, hold)` labels for a split keycap;
    /// `None` for simple keys.
    pub fn dual_labels(self) -> Option<(String, String)> {
        match self {
            KeyAction::ModTap { mods, key } => {
                Some((key.name(), mod_tap_prefix(mods.0).to_string()))
            }
            KeyAction::LayerTap { layer, key } => Some((key.name(), format!("LT{layer}"))),
            KeyAction::LayerMod { layer, mods } => Some((format!("LM{layer}"), mods.to_string())),
            KeyAction::TapToggleLayer(l) => Some((format!("TT{l}"), format!("L{l}"))),
            KeyAction::OneShotLayer(l) => Some(("OSL".to_string(), format!("L{l}"))),
            KeyAction::OneShotMod(m) => Some(("OSM".to_string(), m.to_string())),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_layer_and_basic_actions() {
        assert_eq!(KeyAction::OneShotLayer(LayerId(11)).name(), "OSL(11)");
        assert_eq!(KeyAction::ToLayer(LayerId(3)).name(), "TO(3)");
        assert_eq!(KeyAction::Basic(BasicKey(0x04)).name(), "A");
    }
}
