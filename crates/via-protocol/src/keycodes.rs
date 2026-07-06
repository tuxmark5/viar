use crate::{
    basic_key::BasicKey,
    quantum_key::QuantumKey,
    rgb_key::RgbKey,
};

/// A QMK keycode value (u16).
///
/// QMK keycodes are 16-bit values where the upper bits encode the type
/// (basic, mod-tap, layer-tap, etc.) and the lower bits encode the specific key.
/// Legacy shim: for basic/lighting/quantum ranges it delegates to [`BasicKey`],
/// [`RgbKey`] and [`QuantumKey`]; only parametric and magic/swap-hands values
/// are decoded here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Keycode(pub u16);

impl Keycode {
    pub const NONE: Self = Self(0x0000);
    pub const TRANSPARENT: Self = Self(0x0001);

    pub fn raw(self) -> u16 {
        self.0
    }

    /// Construct a Layer-Tap keycode: LT(layer, kc)
    /// tap produces `kc`, hold activates `layer`.
    pub fn layer_tap(layer: u8, kc: u8) -> Self {
        Self(0x4000 | ((layer as u16 & 0x0F) << 8) | kc as u16)
    }

    /// Construct a Mod-Tap keycode: MT(mod_mask, kc)
    /// tap produces `kc`, hold activates modifier(s).
    pub fn mod_tap(mods: u8, kc: u8) -> Self {
        Self(0x2000 | ((mods as u16 & 0x1F) << 8) | kc as u16)
    }

    /// Construct a Layer-Mod keycode: LM(layer, mod)
    /// Activates `layer` with modifier(s) applied.
    pub fn layer_mod(layer: u8, mods: u8) -> Self {
        Self(0x5000 | ((layer as u16 & 0xF) << 4) | (mods as u16 & 0xF))
    }

    /// Construct a One-Shot Modifier keycode: OSM(mod_mask)
    pub fn one_shot_mod(mods: u8) -> Self {
        Self(0x52A0 | (mods as u16 & 0x1F))
    }

    /// Construct a Mod+Key combo: e.g. C(kc), S(kc), C+S(kc)
    pub fn mod_key(mods: u8, kc: u8) -> Self {
        Self(((mods as u16 & 0x1F) << 8) | kc as u16)
    }

    /// Construct a Swap Hands keycode: SH(kc)
    pub fn swap_hands(kc: u8) -> Self {
        Self(0x5600 | kc as u16)
    }

    // ── Convenience Mod-Tap constructors (QMK-style aliases) ──

    /// LCTL_T(kc) — tap produces `kc`, hold activates Left Control.
    pub fn lctl_t(kc: u8) -> Self {
        Self::mod_tap(0x01, kc)
    }

    /// LSFT_T(kc) — tap produces `kc`, hold activates Left Shift.
    pub fn lsft_t(kc: u8) -> Self {
        Self::mod_tap(0x02, kc)
    }

    /// LALT_T(kc) — tap produces `kc`, hold activates Left Alt.
    pub fn lalt_t(kc: u8) -> Self {
        Self::mod_tap(0x04, kc)
    }

    /// LGUI_T(kc) — tap produces `kc`, hold activates Left GUI.
    pub fn lgui_t(kc: u8) -> Self {
        Self::mod_tap(0x08, kc)
    }

    /// RCTL_T(kc) — tap produces `kc`, hold activates Right Control.
    pub fn rctl_t(kc: u8) -> Self {
        Self::mod_tap(0x11, kc)
    }

    /// RSFT_T(kc) — tap produces `kc`, hold activates Right Shift.
    pub fn rsft_t(kc: u8) -> Self {
        Self::mod_tap(0x12, kc)
    }

    /// RALT_T(kc) — tap produces `kc`, hold activates Right Alt.
    pub fn ralt_t(kc: u8) -> Self {
        Self::mod_tap(0x14, kc)
    }

    /// RGUI_T(kc) — tap produces `kc`, hold activates Right GUI.
    pub fn rgui_t(kc: u8) -> Self {
        Self::mod_tap(0x18, kc)
    }

    /// C_S_T(kc) — tap produces `kc`, hold activates Ctrl+Shift.
    pub fn c_s_t(kc: u8) -> Self {
        Self::mod_tap(0x03, kc)
    }

    /// MEH_T(kc) — tap produces `kc`, hold activates Ctrl+Shift+Alt (Meh).
    pub fn meh_t(kc: u8) -> Self {
        Self::mod_tap(0x07, kc)
    }

    /// HYPR_T(kc) — tap produces `kc`, hold activates Ctrl+Shift+Alt+GUI (Hyper).
    pub fn hypr_t(kc: u8) -> Self {
        Self::mod_tap(0x0F, kc)
    }

    /// ALL_T(kc) — alias for HYPR_T.
    pub fn all_t(kc: u8) -> Self {
        Self::hypr_t(kc)
    }

    /// Construct a One-Shot Layer keycode: OSL(layer)
    pub fn one_shot_layer(layer: u8) -> Self {
        Self(0x5280 | (layer as u16 & 0x1F))
    }

    /// Construct a Layer Momentary keycode: MO(layer)
    pub fn layer_momentary(layer: u8) -> Self {
        Self(0x5220 | (layer as u16 & 0x1F))
    }

    /// Construct a Layer Toggle keycode: TG(layer)
    pub fn layer_toggle(layer: u8) -> Self {
        Self(0x5260 | (layer as u16 & 0x1F))
    }

    /// Construct a Layer On keycode: TO(layer)
    pub fn layer_on(layer: u8) -> Self {
        Self(0x5200 | (layer as u16 & 0x1F))
    }

    /// Construct a Tap-Toggle Layer keycode: TT(layer)
    pub fn layer_tap_toggle(layer: u8) -> Self {
        Self(0x52C0 | (layer as u16 & 0x1F))
    }

    /// Construct a Default Layer keycode: DF(layer)
    pub fn layer_default(layer: u8) -> Self {
        Self(0x5240 | (layer as u16 & 0x1F))
    }

    /// Extract the modifier mask for ModTap, Mod, or OSM keycodes.
    pub fn mod_mask(self) -> u8 {
        match self.category() {
            KeycodeCategory::ModTap => ((self.0 >> 8) & 0x1F) as u8,
            KeycodeCategory::Mod => ((self.0 >> 8) & 0x1F) as u8,
            KeycodeCategory::LayerOneShotMod => (self.0 & 0x1F) as u8,
            KeycodeCategory::LayerMod => (self.0 & 0xF) as u8,
            _ => 0,
        }
    }

    /// Extract the basic keycode for ModTap, Mod, or LayerTap keycodes.
    pub fn base_keycode(self) -> u8 {
        match self.category() {
            KeycodeCategory::ModTap | KeycodeCategory::Mod | KeycodeCategory::LayerTap => {
                (self.0 & 0xFF) as u8
            }
            _ => 0,
        }
    }

    /// Extract the layer number for layer keycodes.
    pub fn layer(self) -> u8 {
        match self.category() {
            KeycodeCategory::LayerTap => ((self.0 >> 8) & 0x0F) as u8,
            KeycodeCategory::LayerMod => ((self.0 >> 4) & 0xF) as u8,
            KeycodeCategory::LayerMomentary
            | KeycodeCategory::LayerToggle
            | KeycodeCategory::LayerOn
            | KeycodeCategory::LayerDefault
            | KeycodeCategory::LayerOneShotLayer
            | KeycodeCategory::LayerTapToggle
            | KeycodeCategory::PersistentDefLayer => (self.0 & 0x1F) as u8,
            _ => 0,
        }
    }

    /// Determine the category of this keycode.
    pub fn category(self) -> KeycodeCategory {
        match self.0 {
            0x0000 => KeycodeCategory::None,
            0x0001 => KeycodeCategory::Transparent,
            // Mouse keys: QMK uses 0x00CD-0x00D9 range
            0x00CD..=0x00D9 => KeycodeCategory::Mouse,
            0x0004..=0x00FF => KeycodeCategory::Basic,
            0x0100..=0x1FFF => KeycodeCategory::Mod,
            0x2000..=0x3FFF => KeycodeCategory::ModTap,
            0x4000..=0x4FFF => KeycodeCategory::LayerTap,
            0x5000..=0x51FF => KeycodeCategory::LayerMod,
            0x5200..=0x521F => KeycodeCategory::LayerOn, // TO(layer)
            0x5220..=0x523F => KeycodeCategory::LayerMomentary, // MO(layer)
            0x5240..=0x525F => KeycodeCategory::LayerDefault, // DF(layer)
            0x5260..=0x527F => KeycodeCategory::LayerToggle, // TG(layer)
            0x5280..=0x529F => KeycodeCategory::LayerOneShotLayer, // OSL(layer)
            0x52A0..=0x52BF => KeycodeCategory::LayerOneShotMod, // OSM(mod)
            0x52C0..=0x52DF => KeycodeCategory::LayerTapToggle, // TT(layer)
            0x52E0..=0x52FF => KeycodeCategory::PersistentDefLayer,
            0x5600..=0x56FF => KeycodeCategory::SwapHands,
            0x5700..=0x57FF => KeycodeCategory::TapDance,
            0x7C77 => KeycodeCategory::TriLayer,
            0x7C78 => KeycodeCategory::TriLayer,
            0x7000..=0x70FF => KeycodeCategory::Magic,
            0x7800..=0x78FF => KeycodeCategory::Lighting,
            0x7C00..=0x7DFF => KeycodeCategory::Quantum,
            _ => KeycodeCategory::Unknown,
        }
    }

    /// Get a human-readable name for this keycode.
    /// Returns an owned String since complex keycodes need formatting.
    pub fn name(self) -> String {
        if self.0 <= 0xFF {
            return BasicKey(self.0 as u8).name();
        }
        if let Some(name) = magic_keycode_name(self.0) {
            return name.to_string();
        }
        if (RgbKey::BLOCK..=(RgbKey::BLOCK | 0xFF)).contains(&self.0) {
            return RgbKey((self.0 & 0xFF) as u8).name();
        }
        if (QuantumKey::BLOCK..=(QuantumKey::BLOCK | 0xFF)).contains(&self.0) {
            return QuantumKey((self.0 & 0xFF) as u8).name();
        }
        self.decode_complex()
    }

    /// Canonical QMK keycode name (e.g. `KC_A`, `KC_SEMICOLON`), from QMK's
    /// `quantum/keycodes.h`. `None` for parametric keycodes (mod-tap, layer-tap,
    /// …) whose names are composed at runtime — use [`Self::name`] for those.
    pub fn qmk_name(self) -> Option<&'static str> {
        crate::qmk_names::qmk_keycode_name(self.0)
    }

    /// Get a short label (for rendering on small key caps).
    /// Truncates to fit in tight spaces.
    pub fn short_name(self) -> String {
        let full = self.name();
        if full.len() <= 5 {
            full
        } else {
            // For complex names, abbreviate
            full
        }
    }

    /// For dual-function keys (mod-tap, layer-tap, etc.), return separate
    /// tap and hold labels for split rendering on keycaps.
    /// Returns `Some((tap_label, hold_label))` or `None` for simple keys.
    pub fn dual_labels(self) -> Option<(String, String)> {
        let raw = self.0;
        match self.category() {
            KeycodeCategory::ModTap => {
                let mods = (raw >> 8) & 0x1F;
                let kc = raw & 0xFF;
                let tap = BasicKey(kc as u8).name();
                let hold = mod_tap_prefix(mods as u8).to_string();
                Some((tap, hold))
            }
            KeycodeCategory::LayerTap => {
                let layer = (raw >> 8) & 0x0F;
                let kc = raw & 0xFF;
                let tap = BasicKey(kc as u8).name();
                let hold = format!("LT{layer}");
                Some((tap, hold))
            }
            KeycodeCategory::LayerMod => {
                let layer = (raw >> 4) & 0xF;
                let mods = raw & 0xF;
                let hold = mod_mask_to_string(mods as u8);
                let tap = format!("LM{layer}");
                Some((tap, hold))
            }
            KeycodeCategory::LayerTapToggle => {
                let layer = raw & 0x1F;
                Some((format!("TT{layer}"), format!("L{layer}")))
            }
            KeycodeCategory::LayerOneShotLayer => {
                let layer = raw & 0x1F;
                Some(("OSL".to_string(), format!("L{layer}")))
            }
            KeycodeCategory::LayerOneShotMod => {
                let mods = (raw & 0x1F) as u8;
                let hold = mod_mask_to_string(mods);
                Some(("OSM".to_string(), hold))
            }
            _ => None,
        }
    }

    /// Decode a complex (non-basic) keycode into a descriptive string.
    fn decode_complex(self) -> String {
        let raw = self.0;
        match self.category() {
            KeycodeCategory::LayerTap => {
                // LT(layer, kc): bits [11:8] = layer (4 bits), bits [7:0] = keycode
                let layer = (raw >> 8) & 0x0F;
                let kc = raw & 0xFF;
                let kc_name = BasicKey(kc as u8).name();
                format!("LT({layer},{kc_name})")
            }
            KeycodeCategory::LayerMod => {
                // LM(layer, mod): bits [12:8] = mod, bits [7:4] = layer...
                // Actually: QK_LAYER_MOD = 0x5000, layer in bits [8:4], mods in bits [3:0] shifted
                // QMK: #define LM(layer, mod) (QK_LAYER_MOD | (((layer) & 0xF) << 4) | ((mod) &
                // 0xF))
                let layer = (raw >> 4) & 0xF;
                let mods = raw & 0xF;
                let mod_str = mod_mask_to_string(mods as u8);
                format!("LM({layer},{mod_str})")
            }
            KeycodeCategory::LayerMomentary => {
                let layer = raw & 0x1F;
                format!("MO({layer})")
            }
            KeycodeCategory::LayerDefault => {
                let layer = raw & 0x1F;
                format!("DF({layer})")
            }
            KeycodeCategory::LayerToggle => {
                let layer = raw & 0x1F;
                format!("TG({layer})")
            }
            KeycodeCategory::LayerOneShotLayer => {
                let layer = raw & 0x1F;
                format!("OSL({layer})")
            }
            KeycodeCategory::LayerOneShotMod => {
                let mods = raw & 0x1F;
                format!("OSM({mods:#04x})")
            }
            KeycodeCategory::LayerTapToggle => {
                let layer = raw & 0x1F;
                format!("TT({layer})")
            }
            KeycodeCategory::PersistentDefLayer => {
                let layer = raw & 0x1F;
                format!("PDF({layer})")
            }
            KeycodeCategory::LayerOn => {
                // TO(layer): QK_TO | layer
                let layer = raw & 0x1F;
                format!("TO({layer})")
            }
            KeycodeCategory::ModTap => {
                // MT(mod, kc): bits [12:8] = mod mask, bits [7:0] = keycode
                let mods = (raw >> 8) & 0x1F;
                let kc = raw & 0xFF;
                let kc_name = BasicKey(kc as u8).name();
                // Use QMK-style alias if it's a single modifier
                let prefix = mod_tap_prefix(mods as u8);
                format!("{prefix}({kc_name})")
            }
            KeycodeCategory::Mod => {
                // Modifier + basic key: bits [12:8] = mod, bits [7:0] = key
                let mods = ((raw >> 8) & 0x1F) as u8;
                let kc = raw & 0xFF;
                if kc == 0 {
                    mod_mask_to_string(mods)
                } else if mods == 0x02 {
                    // Shift-only: show the actual shifted symbol if possible
                    if let Some(sym) = shifted_symbol(kc) {
                        return sym.to_string();
                    }
                    let kc_name = BasicKey(kc as u8).name();
                    format!("S({kc_name})")
                } else {
                    let kc_name = BasicKey(kc as u8).name();
                    let mod_str = mod_mask_to_string(mods);
                    format!("{mod_str}({kc_name})")
                }
            }
            KeycodeCategory::TapDance => {
                let idx = raw & 0xFF;
                format!("TD({idx})")
            }
            KeycodeCategory::SwapHands => {
                let kc = raw & 0xFF;
                if kc == 0xF0 {
                    "SH_TG".to_string()
                } else if kc == 0xF1 {
                    "SH_TT".to_string()
                } else if kc == 0xF2 {
                    "SH_MON".to_string()
                } else if kc == 0xF3 {
                    "SH_MOFF".to_string()
                } else if kc == 0xF4 {
                    "SH_OFF".to_string()
                } else if kc == 0xF5 {
                    "SH_ON".to_string()
                } else if kc == 0xF6 {
                    "SH_OS".to_string()
                } else {
                    let kc_name = BasicKey(kc as u8).name();
                    format!("SH({kc_name})")
                }
            }
            KeycodeCategory::Mouse => BasicKey(raw as u8).name(),
            _ => format!("{raw:#06x}"),
        }
    }

    /// Get a human-friendly description for tooltips.
    pub fn description(self) -> String {
        if self.0 <= 0xFF {
            return BasicKey(self.0 as u8).description();
        }
        if (RgbKey::BLOCK..=(RgbKey::BLOCK | 0xFF)).contains(&self.0) {
            return RgbKey((self.0 & 0xFF) as u8).description();
        }
        if (QuantumKey::BLOCK..=(QuantumKey::BLOCK | 0xFF)).contains(&self.0) {
            return QuantumKey((self.0 & 0xFF) as u8).description();
        }
        // Parametric / magic / swap-hands keycodes, by category.
        match self.category() {
            KeycodeCategory::Mod => {
                let mods = ((self.0 >> 8) & 0x1F) as u8;
                let kc = self.0 & 0xFF;
                let mod_str = mod_mask_to_string(mods);
                if kc == 0 {
                    format!("{mod_str} modifier")
                } else {
                    let kc_name = BasicKey(kc as u8).name();
                    format!("{kc_name} with {mod_str} held")
                }
            }
            KeycodeCategory::ModTap => {
                let mods = ((self.0 >> 8) & 0x1F) as u8;
                let kc = self.0 & 0xFF;
                let kc_name = BasicKey(kc as u8).name();
                let mod_str = mod_mask_to_string(mods);
                format!("{kc_name} on tap, {mod_str} on hold")
            }
            KeycodeCategory::LayerTap => {
                let layer = (self.0 >> 8) & 0x0F;
                let kc = self.0 & 0xFF;
                let kc_name = BasicKey(kc as u8).name();
                format!("{kc_name} on tap, Layer {layer} on hold")
            }
            KeycodeCategory::LayerMod => {
                let layer = (self.0 >> 4) & 0xF;
                let mods = (self.0 & 0xF) as u8;
                let mod_str = mod_mask_to_string(mods);
                format!("Activate Layer {layer} with {mod_str}")
            }
            KeycodeCategory::LayerMomentary => {
                let layer = self.0 & 0x1F;
                format!("Momentary Layer {layer} — active while held")
            }
            KeycodeCategory::LayerToggle => {
                let layer = self.0 & 0x1F;
                format!("Toggle Layer {layer} on/off")
            }
            KeycodeCategory::LayerOn => {
                let layer = self.0 & 0x1F;
                format!("Turn on Layer {layer} (deactivate all others)")
            }
            KeycodeCategory::LayerDefault => {
                let layer = self.0 & 0x1F;
                format!("Set Layer {layer} as the default base layer")
            }
            KeycodeCategory::LayerOneShotLayer => {
                let layer = self.0 & 0x1F;
                format!("One-Shot Layer {layer} — active for the next keypress only")
            }
            KeycodeCategory::LayerOneShotMod => {
                let mods = (self.0 & 0x1F) as u8;
                let mod_str = mod_mask_to_string(mods);
                format!("One-Shot {mod_str} — applies to the next keypress only")
            }
            KeycodeCategory::LayerTapToggle => {
                let layer = self.0 & 0x1F;
                format!("Layer {layer} on hold, toggle on tap")
            }
            KeycodeCategory::PersistentDefLayer => {
                let layer = self.0 & 0x1F;
                format!("Persistently set Layer {layer} as default (survives reboot)")
            }
            KeycodeCategory::TapDance => {
                let idx = self.0 & 0xFF;
                format!("Tap Dance {idx} — different actions for tap/hold/double-tap")
            }
            KeycodeCategory::SwapHands => {
                let kc = self.0 & 0xFF;
                match kc {
                    0xF0 => "Toggle swap hands".into(),
                    0xF1 => "Tap-toggle swap hands".into(),
                    0xF2 => "Momentary swap on".into(),
                    0xF3 => "Momentary swap off".into(),
                    0xF4 => "Turn off swap hands".into(),
                    0xF5 => "Turn on swap hands".into(),
                    0xF6 => "One-shot swap hands".into(),
                    _ => {
                        let kc_name = BasicKey(kc as u8).name();
                        format!("{kc_name} on tap, swap hands on hold")
                    }
                }
            }
            KeycodeCategory::Magic => magic_keycode_name(self.0)
                .map(|n| format!("Magic: {n}"))
                .unwrap_or_else(|| format!("Magic keycode (0x{:04X})", self.0)),
            _ => {
                let name = self.name();
                format!("{name} (0x{:04X})", self.0)
            }
        }
    }
}

impl std::fmt::Display for Keycode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeycodeCategory {
    None,
    Transparent,
    Basic,
    Mouse,
    Mod,
    LayerTap,
    LayerOn,
    LayerMomentary,
    LayerDefault,
    LayerToggle,
    LayerOneShotLayer,
    LayerOneShotMod,
    TapDance,
    ModTap,
    TriLayer,
    LayerMod,
    LayerTapToggle,
    PersistentDefLayer,
    SwapHands,
    Magic,
    Lighting,
    Quantum,
    Unicode,
    Unknown,
}

/// Convert a QMK modifier bitmask to a human-readable string.
pub fn mod_mask_to_string(mods: u8) -> String {
    let mut parts = Vec::new();
    // Left modifiers
    if mods & 0x01 != 0 {
        parts.push("C");
    } // Ctrl
    if mods & 0x02 != 0 {
        parts.push("S");
    } // Shift
    if mods & 0x04 != 0 {
        parts.push("A");
    } // Alt
    if mods & 0x08 != 0 {
        parts.push("G");
    } // GUI
    // Right modifiers (bit 4 = use right side)
    if mods & 0x10 != 0 {
        // Right-side flag — modify the labels
        parts.iter_mut().for_each(|p| {
            *p = match *p {
                "C" => "RC",
                "S" => "RS",
                "A" => "RA",
                "G" => "RG",
                _ => *p,
            }
        });
    }
    if parts.is_empty() {
        "MOD".to_string()
    } else {
        parts.join("+")
    }
}

/// Get the QMK-style Mod-Tap prefix for a given modifier mask.
/// Returns e.g. "LSFT_T", "LCTL_T", "MEH_T", or falls back to "MT(mods,".
pub fn mod_tap_prefix(mods: u8) -> &'static str {
    match mods {
        0x01 => "LCTL_T",
        0x02 => "LSFT_T",
        0x04 => "LALT_T",
        0x08 => "LGUI_T",
        0x11 => "RCTL_T",
        0x12 => "RSFT_T",
        0x14 => "RALT_T",
        0x18 => "RGUI_T",
        0x03 => "C_S_T",
        0x05 => "LCA_T",
        0x09 => "LCG_T",
        0x06 => "LSA_T",
        0x0A => "LSG_T",
        0x0C => "LAG_T",
        0x07 => "MEH_T",
        0x0F => "HYPR_T",
        0x13 => "RCS_T",
        0x15 => "RCA_T",
        0x19 => "RCG_T",
        0x16 => "RSA_T",
        0x1A => "RSG_T",
        0x1C => "RAG_T",
        0x17 => "RMEH_T",
        0x1F => "RHYP_T",
        _ => "MT",
    }
}

/// Map a basic HID keycode to its US-ANSI shifted symbol.
/// Returns None if there's no common symbol (e.g. Shift+A is just 'A').
fn shifted_symbol(kc: u16) -> Option<&'static str> {
    Some(match kc {
        0x1E => "!",  // 1
        0x1F => "@",  // 2
        0x20 => "#",  // 3
        0x21 => "$",  // 4
        0x22 => "%",  // 5
        0x23 => "^",  // 6
        0x24 => "&",  // 7
        0x25 => "*",  // 8
        0x26 => "(",  // 9
        0x27 => ")",  // 0
        0x2D => "_",  // -
        0x2E => "+",  // =
        0x2F => "{",  // [
        0x30 => "}",  // ]
        0x31 => "|",  // backslash
        0x33 => ":",  // ;
        0x34 => "\"", // '
        0x35 => "~",  // `
        0x36 => "<",  // ,
        0x37 => ">",  // .
        0x38 => "?",  // /
        _ => return None,
    })
}

/// Look up the name of a Magic keycode (0x7000-0x70FF range).
fn magic_keycode_name(kc: u16) -> Option<&'static str> {
    Some(match kc {
        0x7000 => "MG_SWNU", // Swap Control and GUI (on)
        0x7001 => "MG_SWCU", // Swap Control and Caps Lock (on)
        0x7002 => "MG_SWLA", // Swap Left Alt and GUI (on)
        0x7003 => "MG_SWRA", // Swap Right Alt and GUI (on)
        0x7004 => "MG_NKRO", // N-Key Rollover (on)
        0x7005 => "MG_GESC", // Grave Escape (on)
        0x7006 => "MG_BSPC", // Swap Backspace and Backslash (on)
        0x7007 => "CG_NORM", // Unswap Control and GUI
        0x7008 => "MG_UNCC", // Unswap Caps Lock
        0x7009 => "MG_UNLA", // Unswap Left Alt and GUI
        0x700A => "MG_UNRA", // Unswap Right Alt and GUI
        0x700B => "MG_UNNK", // N-Key Rollover (off)
        0x700C => "MG_UNGE", // Grave Escape (off)
        0x700D => "MG_UNBS", // Unswap Backspace
        0x700E => "CG_TOGG", // Toggle Control and GUI swap
        0x700F => "MG_TOGN", // Toggle NKRO
        _ => return None,
    })
}

/// Describes a type of quantum key that can be configured.
#[derive(Debug, Clone)]
pub struct QuantumKeyType {
    /// Display name, e.g. "LSFT_T"
    pub name:        &'static str,
    /// Human-readable description
    pub description: &'static str,
    /// The modifier mask used (for mod-tap types)
    pub mod_mask:    Option<u8>,
    /// The category this belongs to
    pub category:    &'static str,
}

/// Get all available quantum key types that can be configured.
/// This is the public API for discovering what quantum keys are available.
pub fn quantum_key_types() -> Vec<QuantumKeyType> {
    vec![
        // Mod-Tap keys
        QuantumKeyType {
            name:        "LCTL_T",
            description: "Tap: key, Hold: Left Control",
            mod_mask:    Some(0x01),
            category:    "Mod-Tap",
        },
        QuantumKeyType {
            name:        "LSFT_T",
            description: "Tap: key, Hold: Left Shift",
            mod_mask:    Some(0x02),
            category:    "Mod-Tap",
        },
        QuantumKeyType {
            name:        "LALT_T",
            description: "Tap: key, Hold: Left Alt",
            mod_mask:    Some(0x04),
            category:    "Mod-Tap",
        },
        QuantumKeyType {
            name:        "LGUI_T",
            description: "Tap: key, Hold: Left GUI/Super",
            mod_mask:    Some(0x08),
            category:    "Mod-Tap",
        },
        QuantumKeyType {
            name:        "RCTL_T",
            description: "Tap: key, Hold: Right Control",
            mod_mask:    Some(0x11),
            category:    "Mod-Tap",
        },
        QuantumKeyType {
            name:        "RSFT_T",
            description: "Tap: key, Hold: Right Shift",
            mod_mask:    Some(0x12),
            category:    "Mod-Tap",
        },
        QuantumKeyType {
            name:        "RALT_T",
            description: "Tap: key, Hold: Right Alt",
            mod_mask:    Some(0x14),
            category:    "Mod-Tap",
        },
        QuantumKeyType {
            name:        "RGUI_T",
            description: "Tap: key, Hold: Right GUI/Super",
            mod_mask:    Some(0x18),
            category:    "Mod-Tap",
        },
        QuantumKeyType {
            name:        "C_S_T",
            description: "Tap: key, Hold: Ctrl+Shift",
            mod_mask:    Some(0x03),
            category:    "Mod-Tap",
        },
        QuantumKeyType {
            name:        "LCA_T",
            description: "Tap: key, Hold: Ctrl+Alt",
            mod_mask:    Some(0x05),
            category:    "Mod-Tap",
        },
        QuantumKeyType {
            name:        "LCG_T",
            description: "Tap: key, Hold: Ctrl+GUI",
            mod_mask:    Some(0x09),
            category:    "Mod-Tap",
        },
        QuantumKeyType {
            name:        "LSA_T",
            description: "Tap: key, Hold: Shift+Alt",
            mod_mask:    Some(0x06),
            category:    "Mod-Tap",
        },
        QuantumKeyType {
            name:        "LSG_T",
            description: "Tap: key, Hold: Shift+GUI",
            mod_mask:    Some(0x0A),
            category:    "Mod-Tap",
        },
        QuantumKeyType {
            name:        "LAG_T",
            description: "Tap: key, Hold: Alt+GUI",
            mod_mask:    Some(0x0C),
            category:    "Mod-Tap",
        },
        QuantumKeyType {
            name:        "MEH_T",
            description: "Tap: key, Hold: Ctrl+Shift+Alt (Meh)",
            mod_mask:    Some(0x07),
            category:    "Mod-Tap",
        },
        QuantumKeyType {
            name:        "HYPR_T",
            description: "Tap: key, Hold: Ctrl+Shift+Alt+GUI (Hyper)",
            mod_mask:    Some(0x0F),
            category:    "Mod-Tap",
        },
        // One-Shot Modifiers
        QuantumKeyType {
            name:        "OSM(Ctrl)",
            description: "One-Shot Left Control — applies to next keypress only",
            mod_mask:    Some(0x01),
            category:    "One-Shot Mod",
        },
        QuantumKeyType {
            name:        "OSM(Shift)",
            description: "One-Shot Left Shift — applies to next keypress only",
            mod_mask:    Some(0x02),
            category:    "One-Shot Mod",
        },
        QuantumKeyType {
            name:        "OSM(Alt)",
            description: "One-Shot Left Alt — applies to next keypress only",
            mod_mask:    Some(0x04),
            category:    "One-Shot Mod",
        },
        QuantumKeyType {
            name:        "OSM(GUI)",
            description: "One-Shot Left GUI — applies to next keypress only",
            mod_mask:    Some(0x08),
            category:    "One-Shot Mod",
        },
        QuantumKeyType {
            name:        "OSM(Meh)",
            description: "One-Shot Ctrl+Shift+Alt — applies to next keypress only",
            mod_mask:    Some(0x07),
            category:    "One-Shot Mod",
        },
        QuantumKeyType {
            name:        "OSM(Hyper)",
            description: "One-Shot Ctrl+Shift+Alt+GUI — applies to next keypress only",
            mod_mask:    Some(0x0F),
            category:    "One-Shot Mod",
        },
        // Layer functions
        QuantumKeyType {
            name:        "LT",
            description: "Layer-Tap: tap produces key, hold activates layer",
            mod_mask:    None,
            category:    "Layer-Tap",
        },
        QuantumKeyType {
            name:        "MO",
            description: "Momentary layer — active while held",
            mod_mask:    None,
            category:    "Layer",
        },
        QuantumKeyType {
            name:        "TG",
            description: "Toggle layer on/off",
            mod_mask:    None,
            category:    "Layer",
        },
        QuantumKeyType {
            name:        "TO",
            description: "Turn on layer (deactivates all others)",
            mod_mask:    None,
            category:    "Layer",
        },
        QuantumKeyType {
            name:        "TT",
            description: "Layer on hold, toggle on tap",
            mod_mask:    None,
            category:    "Layer",
        },
        QuantumKeyType {
            name:        "DF",
            description: "Set default base layer",
            mod_mask:    None,
            category:    "Layer",
        },
        QuantumKeyType {
            name:        "OSL",
            description: "One-Shot Layer — active for next keypress only",
            mod_mask:    None,
            category:    "Layer",
        },
        QuantumKeyType {
            name:        "LM",
            description: "Activate layer with modifier(s) applied",
            mod_mask:    None,
            category:    "Layer",
        },
    ]
}

/// Default quantum keycode configurations — returns commonly used presets.
/// Each entry is (display_name, keycode_raw_value).
pub fn quantum_keycode_defaults() -> Vec<(&'static str, u16)> {
    let mut defaults = Vec::new();

    // Home-row mod presets (LSFT_T on common keys)
    let home_row_keys: &[(u8, &str)] = &[
        (0x04, "A"),
        (0x16, "S"),
        (0x07, "D"),
        (0x09, "F"),
        (0x0D, "J"),
        (0x0E, "K"),
        (0x0F, "L"),
    ];

    for &(kc, _name) in home_row_keys {
        defaults.push(("LSFT_T", Keycode::lsft_t(kc).raw()));
        defaults.push(("LCTL_T", Keycode::lctl_t(kc).raw()));
        defaults.push(("LALT_T", Keycode::lalt_t(kc).raw()));
        defaults.push(("LGUI_T", Keycode::lgui_t(kc).raw()));
    }

    // One-shot modifiers
    defaults.push(("OSM(Ctrl)", Keycode::one_shot_mod(0x01).raw()));
    defaults.push(("OSM(Shift)", Keycode::one_shot_mod(0x02).raw()));
    defaults.push(("OSM(Alt)", Keycode::one_shot_mod(0x04).raw()));
    defaults.push(("OSM(GUI)", Keycode::one_shot_mod(0x08).raw()));
    defaults.push(("OSM(Meh)", Keycode::one_shot_mod(0x07).raw()));
    defaults.push(("OSM(Hyper)", Keycode::one_shot_mod(0x0F).raw()));

    defaults
}
