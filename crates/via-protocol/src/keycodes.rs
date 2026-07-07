//! Keycode categories and QMK naming/data helpers.
//!
//! Turning a raw keycode value into its meaning (and back) lives entirely in
//! [`crate::encoding`] / [`crate::KeycodeEncoding`]; this module holds no
//! decoding or bit-fiddling. What remains is the [`KeycodeCategory`] enum (for
//! picker grouping and keycap coloring) and the quantum-key catalog shown in the
//! picker UI. Modifier-mask formatting lives on [`crate::ModMask`].

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
