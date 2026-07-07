//! The keycode palette shown in the picker, as encoding-independent
//! [`KeyAction`]s. Layer/mod-tap actions are semantic variants (so they encode
//! correctly per board); magic/lighting/quantum/tap-dance keycodes have no
//! modelled variant yet and are carried as `Raw` canonical values.

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

/// How many layers the picker offers for layer keycodes (MO/TG/TO/DF/OSL/TT):
/// layers `0..LAYER_KEYCODE_COUNT`.
const LAYER_KEYCODE_COUNT: u8 = 16;

/// The eleven punctuation keys, shared by the "Symbols" and "Shifted" groups.
const SYMBOLS: &[BasicKey] = &[
    BasicKey::KC_MINUS,
    BasicKey::KC_EQUAL,
    BasicKey::KC_LEFT_BRACKET,
    BasicKey::KC_RIGHT_BRACKET,
    BasicKey::KC_BACKSLASH,
    BasicKey::KC_SEMICOLON,
    BasicKey::KC_QUOTE,
    BasicKey::KC_GRAVE,
    BasicKey::KC_COMMA,
    BasicKey::KC_DOT,
    BasicKey::KC_SLASH,
];

const fn basic(k: u8) -> KeyAction {
    KeyAction::Basic(BasicKey(k))
}

/// Inclusive run of basic keycodes `from..=to`, as `KeyAction::Basic`.
fn basic_range(from: BasicKey, to: BasicKey) -> impl Iterator<Item = KeyAction> {
    (from.0..=to.0).map(basic)
}

/// A Shift-modified basic key (`S(kc)`).
const fn shifted(key: BasicKey) -> KeyAction {
    KeyAction::Modified {
        mods: ModMask(0x02),
        key,
    }
}

/// Every basic HID keycode (letters, numbers, common keys, F-keys, nav, numpad,
/// modifiers), for callers that want a flat list.
pub fn all_basic_keycodes() -> Vec<KeyAction> {
    use BasicKey as K;
    basic_range(K::KC_A, K::KC_Z) // Letters
        .chain(basic_range(K::KC_1, K::KC_0)) // Numbers
        .chain(basic_range(K::KC_ENTER, K::KC_SLASH)) // Common keys
        .chain(basic_range(K::KC_F1, K::KC_F12)) // F-keys
        .chain(basic_range(K::KC_PRINT_SCREEN, K::KC_UP)) // Nav cluster
        .chain(basic_range(K::KC_NUM_LOCK, K::KC_KP_DOT)) // Numpad
        .chain(basic_range(K::KC_LEFT_CTRL, K::KC_RIGHT_GUI)) // Modifiers
        .collect()
}

/// A named group of keycodes for the picker UI.
pub struct KeycodeGroup {
    pub name:  &'static str,
    pub codes: Vec<KeyAction>,
}

/// Keycodes organised into groups for the picker.
pub fn keycode_groups() -> Vec<KeycodeGroup> {
    use BasicKey as K;
    vec![
        KeycodeGroup {
            name:  "Letters",
            codes: basic_range(K::KC_A, K::KC_Z).collect(),
        },
        KeycodeGroup {
            name:  "Numbers",
            codes: basic_range(K::KC_1, K::KC_0).collect(),
        },
        KeycodeGroup {
            name:  "Symbols",
            codes: SYMBOLS.iter().copied().map(KeyAction::Basic).collect(),
        },
        KeycodeGroup {
            name:  "Shifted",
            codes: (K::KC_1.0..=K::KC_0.0)
                .map(BasicKey)
                .chain(SYMBOLS.iter().copied())
                .map(shifted)
                .collect(),
        },
        KeycodeGroup {
            name:  "Editing",
            codes: [
                K::KC_ENTER,
                K::KC_ESCAPE,
                K::KC_BACKSPACE,
                K::KC_TAB,
                K::KC_SPACE,
                K::KC_CAPS_LOCK,
                K::KC_INSERT,
                K::KC_DELETE,
            ]
            .map(KeyAction::Basic)
            .to_vec(),
        },
        KeycodeGroup {
            name:  "Navigation",
            codes: [
                K::KC_HOME,
                K::KC_END,
                K::KC_PAGE_UP,
                K::KC_PAGE_DOWN,
                K::KC_LEFT,
                K::KC_DOWN,
                K::KC_UP,
                K::KC_RIGHT,
            ]
            .map(KeyAction::Basic)
            .to_vec(),
        },
        KeycodeGroup {
            name:  "F-Keys",
            codes: basic_range(K::KC_F1, K::KC_F12)
                .chain(basic_range(K::KC_F13, K::KC_F24))
                .collect(),
        },
        KeycodeGroup {
            name:  "Modifiers",
            codes: basic_range(K::KC_LEFT_CTRL, K::KC_RIGHT_GUI).collect(),
        },
        KeycodeGroup {
            name:  "Media",
            codes: basic_range(K::KC_SYSTEM_POWER, K::KC_MEDIA_FAST_FORWARD).collect(),
        },
        KeycodeGroup {
            name:  "Mouse",
            codes: basic_range(K::QK_MOUSE_CURSOR_UP, K::QK_MOUSE_WHEEL_UP).collect(),
        },
        KeycodeGroup {
            name:  "Layers",
            codes: {
                let layers = |f: fn(LayerId) -> KeyAction| {
                    (0..LAYER_KEYCODE_COUNT).map(move |i| f(LayerId(i)))
                };
                let mut v: Vec<KeyAction> = layers(KeyAction::Momentary)
                    .chain(layers(KeyAction::ToggleLayer))
                    .chain(layers(KeyAction::ToLayer))
                    .chain(layers(KeyAction::DefLayer))
                    .chain(layers(KeyAction::OneShotLayer))
                    .chain(layers(KeyAction::TapToggleLayer))
                    .chain(layers(KeyAction::PersistentDefLayer))
                    .collect();
                // One-shot modifiers: L/R Ctrl/Shift/Alt/GUI and common combos.
                use ModMask as M;
                v.extend(
                    [
                        M::CTRL,
                        M::SHIFT,
                        M::ALT,
                        M::GUI,
                        M::RCTRL,
                        M::RSHIFT,
                        M::RALT,
                        M::RGUI,
                        M::CTRL | M::SHIFT,
                        M::CTRL | M::ALT,
                        M::CTRL | M::GUI,
                        M::SHIFT | M::ALT,
                        M::SHIFT | M::GUI,
                        M::ALT | M::GUI,
                    ]
                    .map(KeyAction::OneShotMod),
                );
                v
            },
        },
        KeycodeGroup {
            name:  "Numpad",
            codes: basic_range(K::KC_NUM_LOCK, K::KC_KP_DOT).collect(),
        },
        KeycodeGroup {
            name:  "Intl",
            // RO, Kana, Yen, Henkan, Muhenkan, Hangul, Hanja
            codes: [
                K::KC_INTERNATIONAL_1,
                K::KC_INTERNATIONAL_2,
                K::KC_INTERNATIONAL_3,
                K::KC_INTERNATIONAL_4,
                K::KC_INTERNATIONAL_5,
                K::KC_LANGUAGE_1,
                K::KC_LANGUAGE_2,
            ]
            .map(KeyAction::Basic)
            .to_vec(),
        },
        KeycodeGroup {
            name:  "Tap Dance",
            codes: (0..32u8)
                .map(|i| KeyAction::TapDance(TapDanceKey(i)))
                .collect(),
        },
        KeycodeGroup {
            name:  "Lighting",
            codes: {
                use RgbKey as R;
                (R::QK_BACKLIGHT_ON.0..=R::QK_BACKLIGHT_TOGGLE_BREATHING.0)
                    .chain(R::QK_LED_MATRIX_ON.0..=R::QK_LED_MATRIX_SPEED_DOWN.0)
                    .chain(R::QK_UNDERGLOW_TOGGLE.0..=R::RGB_MODE_TWINKLE.0)
                    .chain(R::QK_RGB_MATRIX_ON.0..=R::QK_RGB_MATRIX_SPEED_DOWN.0)
                    .map(|o| KeyAction::Rgb(RgbKey(o)))
                    .collect()
            },
        },
        KeycodeGroup {
            name:  "Quantum",
            codes: {
                use QuantumKey as Q;
                [
                    Q::QK_BOOTLOADER,
                    Q::QK_REBOOT,
                    Q::QK_DEBUG_TOGGLE,
                    Q::QK_CLEAR_EEPROM,
                    Q::QK_MAKE,
                    Q::QK_AUTO_SHIFT_TOGGLE,
                    Q::QK_GRAVE_ESCAPE,
                    Q::QK_CAPS_WORD_TOGGLE,
                    Q::QK_AUTOCORRECT_TOGGLE,
                    Q::QK_REPEAT_KEY,
                    Q::QK_ALT_REPEAT_KEY,
                    Q::QK_LAYER_LOCK,
                    Q::QK_LEADER,
                    Q::QK_LOCK,
                    Q::QK_COMBO_TOGGLE,
                    Q::QK_ONE_SHOT_TOGGLE,
                    Q::QK_KEY_OVERRIDE_TOGGLE,
                    Q::QK_DYNAMIC_MACRO_RECORD_START_1,
                    Q::QK_DYNAMIC_MACRO_RECORD_STOP,
                    Q::QK_DYNAMIC_MACRO_PLAY_1,
                ]
                .map(KeyAction::Quantum)
                .to_vec()
            },
        },
        KeycodeGroup {
            name:  "Special",
            codes: vec![
                KeyAction::Basic(K::KC_NO),
                KeyAction::Basic(K::KC_TRANSPARENT),
                KeyAction::Basic(K::KC_PRINT_SCREEN),
                KeyAction::Basic(K::KC_SCROLL_LOCK),
                KeyAction::Basic(K::KC_PAUSE),
                KeyAction::Basic(K::KC_APPLICATION),
                KeyAction::Basic(K::KC_KB_POWER),
                KeyAction::Quantum(QuantumKey::QK_TRI_LAYER_LOWER),
                KeyAction::Quantum(QuantumKey::QK_TRI_LAYER_UPPER),
            ],
        },
    ]
}
