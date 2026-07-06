//! A basic HID keycode (`0x00–0xFF`) and named constants for all of them.

/// A basic HID keycode (`0x00–0xFF`): letters, mods, media, mouse, NONE/TRNS.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BasicKey(pub u8);

impl BasicKey {
    /// The canonical QMK name, e.g. `KC_A`, `KC_SEMICOLON`.
    pub fn qmk_name(self) -> Option<&'static str> {
        crate::qmk_names::qmk_keycode_name(self.0 as u16)
    }

    /// Short display name, e.g. `A`, `Bksp`, `;`.
    pub fn name(self) -> String {
        match self.0 {
            0x00 => "NONE",
            0x01 => "TRNS",
            0x04 => "A",
            0x05 => "B",
            0x06 => "C",
            0x07 => "D",
            0x08 => "E",
            0x09 => "F",
            0x0A => "G",
            0x0B => "H",
            0x0C => "I",
            0x0D => "J",
            0x0E => "K",
            0x0F => "L",
            0x10 => "M",
            0x11 => "N",
            0x12 => "O",
            0x13 => "P",
            0x14 => "Q",
            0x15 => "R",
            0x16 => "S",
            0x17 => "T",
            0x18 => "U",
            0x19 => "V",
            0x1A => "W",
            0x1B => "X",
            0x1C => "Y",
            0x1D => "Z",
            0x1E => "1",
            0x1F => "2",
            0x20 => "3",
            0x21 => "4",
            0x22 => "5",
            0x23 => "6",
            0x24 => "7",
            0x25 => "8",
            0x26 => "9",
            0x27 => "0",
            0x28 => "Enter",
            0x29 => "Esc",
            0x2A => "Bksp",
            0x2B => "Tab",
            0x2C => "Space",
            0x2D => "-",
            0x2E => "=",
            0x2F => "[",
            0x30 => "]",
            0x31 => "\\",
            0x33 => ";",
            0x34 => "'",
            0x35 => "`",
            0x36 => ",",
            0x37 => ".",
            0x38 => "/",
            0x39 => "CapsLk",
            0x3A => "F1",
            0x3B => "F2",
            0x3C => "F3",
            0x3D => "F4",
            0x3E => "F5",
            0x3F => "F6",
            0x40 => "F7",
            0x41 => "F8",
            0x42 => "F9",
            0x43 => "F10",
            0x44 => "F11",
            0x45 => "F12",
            0x46 => "PrtSc",
            0x47 => "ScrLk",
            0x48 => "Pause",
            0x49 => "Ins",
            0x4A => "Home",
            0x4B => "PgUp",
            0x4C => "Del",
            0x4D => "End",
            0x4E => "PgDn",
            0x4F => "→",
            0x50 => "←",
            0x51 => "↓",
            0x52 => "↑",
            0x53 => "NLck",
            0x54 => "N/",
            0x55 => "N*",
            0x56 => "N-",
            0x57 => "N+",
            0x58 => "NEnt",
            0x59 => "N1",
            0x5A => "N2",
            0x5B => "N3",
            0x5C => "N4",
            0x5D => "N5",
            0x5E => "N6",
            0x5F => "N7",
            0x60 => "N8",
            0x61 => "N9",
            0x62 => "N0",
            0x63 => "N.",
            0xE0 => "LCtrl",
            0xE1 => "LShft",
            0xE2 => "LAlt",
            0xE3 => "LGui",
            0xE4 => "RCtrl",
            0xE5 => "RShft",
            0xE6 => "RAlt",
            0xE7 => "RGui",
            0xA5 => "MPlay",
            0xA6 => "MStop",
            0xA7 => "MPrev",
            0xA8 => "Mute",
            0xA9 => "VolUp",
            0xAA => "VolDn",
            0xAB => "MNext",
            0xAC => "MEjct",
            0xAD => "MFfwd",
            0xAE => "MRwnd",
            0xAF => "BriUp",
            0xB0 => "BriDn",
            0xB1 => "MSlct",
            0xB2 => "Mail",
            0xB3 => "Calc",
            0xB4 => "MyCmp",
            0xB5 => "WwwSr",
            0xB6 => "WwwHm",
            0xB7 => "WwwBk",
            0xB8 => "WwwFw",
            0xB9 => "WwwSp",
            0xBA => "WwwRf",
            0xBB => "WwwFv",
            0x65 => "App",
            0x66 => "Power",
            0x68 => "F13",
            0x69 => "F14",
            0x6A => "F15",
            0x6B => "F16",
            0x6C => "F17",
            0x6D => "F18",
            0x6E => "F19",
            0x6F => "F20",
            0x70 => "F21",
            0x71 => "F22",
            0x72 => "F23",
            0x73 => "F24",
            0x87 => "RO",
            0x88 => "Kana",
            0x89 => "Yen",
            0x8A => "Henk",
            0x8B => "Mhen",
            0x90 => "HGL",
            0x91 => "Hanja",
            0xCD => "MsUp",
            0xCE => "MsDn",
            0xCF => "MsLt",
            0xD0 => "MsRt",
            0xD1 => "Btn1",
            0xD2 => "Btn2",
            0xD3 => "Btn3",
            0xD4 => "Btn4",
            0xD5 => "Btn5",
            0xD6 => "WhUp",
            0xD7 => "WhDn",
            0xD8 => "WhLt",
            0xD9 => "WhRt",
            v => return format!("0x{v:02X}"),
        }
        .to_string()
    }

    /// Longer human description for tooltips.
    pub fn description(self) -> String {
        let v = self.0;
        match v {
            0x00 => return "No action (transparent to layers below)".to_string(),
            0x01 => return "Transparent — falls through to the layer below".to_string(),
            0x04..=0x1D => return format!("Character {}", (b'A' + (v - 0x04)) as char),
            0x3A..=0x45 => return format!("Function key F{}", v - 0x3A + 1),
            0x59..=0x61 => return format!("Numpad {}", v - 0x59 + 1),
            _ => {}
        }
        let text: Option<&str> = match v {
            0x1E => Some("Number 1"),
            0x1F => Some("Number 2"),
            0x20 => Some("Number 3"),
            0x21 => Some("Number 4"),
            0x22 => Some("Number 5"),
            0x23 => Some("Number 6"),
            0x24 => Some("Number 7"),
            0x25 => Some("Number 8"),
            0x26 => Some("Number 9"),
            0x27 => Some("Number 0"),
            0x28 => Some("Enter / Return"),
            0x29 => Some("Escape"),
            0x2A => Some("Backspace"),
            0x2B => Some("Tab"),
            0x2C => Some("Spacebar"),
            0x2D => Some("Minus / Hyphen"),
            0x2E => Some("Equals"),
            0x2F => Some("Left Bracket"),
            0x30 => Some("Right Bracket"),
            0x31 => Some("Backslash"),
            0x33 => Some("Semicolon"),
            0x34 => Some("Apostrophe / Quote"),
            0x35 => Some("Grave / Backtick"),
            0x36 => Some("Comma"),
            0x37 => Some("Period / Dot"),
            0x38 => Some("Forward Slash"),
            0x39 => Some("Caps Lock"),
            0x46 => Some("Print Screen"),
            0x47 => Some("Scroll Lock"),
            0x48 => Some("Pause / Break"),
            0x49 => Some("Insert"),
            0x4A => Some("Home"),
            0x4B => Some("Page Up"),
            0x4C => Some("Delete"),
            0x4D => Some("End"),
            0x4E => Some("Page Down"),
            0x4F => Some("Right Arrow"),
            0x50 => Some("Left Arrow"),
            0x51 => Some("Down Arrow"),
            0x52 => Some("Up Arrow"),
            0x53 => Some("Num Lock"),
            0x54 => Some("Numpad Divide"),
            0x55 => Some("Numpad Multiply"),
            0x56 => Some("Numpad Minus"),
            0x57 => Some("Numpad Plus"),
            0x58 => Some("Numpad Enter"),
            0x62 => Some("Numpad 0"),
            0x63 => Some("Numpad Decimal"),
            0x65 => Some("Application / Menu key"),
            0x66 => Some("System Power"),
            0xA5 => Some("Media Play/Pause"),
            0xA6 => Some("Media Stop"),
            0xA7 => Some("Previous Track"),
            0xA8 => Some("Mute audio"),
            0xA9 => Some("Volume Up"),
            0xAA => Some("Volume Down"),
            0xAB => Some("Next Track"),
            0xAC => Some("Media Eject"),
            0xAD => Some("Fast Forward"),
            0xAE => Some("Rewind"),
            0xAF => Some("Screen Brightness Up"),
            0xB0 => Some("Screen Brightness Down"),
            0xB1 => Some("Media Select"),
            0xB2 => Some("Launch Mail"),
            0xB3 => Some("Launch Calculator"),
            0xB4 => Some("Launch My Computer"),
            0xB5 => Some("Browser Search"),
            0xB6 => Some("Browser Home"),
            0xB7 => Some("Browser Back"),
            0xB8 => Some("Browser Forward"),
            0xB9 => Some("Browser Stop"),
            0xBA => Some("Browser Refresh"),
            0xBB => Some("Browser Favorites"),
            0xE0 => Some("Left Control"),
            0xE1 => Some("Left Shift"),
            0xE2 => Some("Left Alt / Option"),
            0xE3 => Some("Left GUI / Super / Command"),
            0xE4 => Some("Right Control"),
            0xE5 => Some("Right Shift"),
            0xE6 => Some("Right Alt / AltGr"),
            0xE7 => Some("Right GUI / Super / Command"),
            0xCD => Some("Mouse Cursor Up"),
            0xCE => Some("Mouse Cursor Down"),
            0xCF => Some("Mouse Cursor Left"),
            0xD0 => Some("Mouse Cursor Right"),
            0xD1 => Some("Mouse Button 1 (Left Click)"),
            0xD2 => Some("Mouse Button 2 (Right Click)"),
            0xD3 => Some("Mouse Button 3 (Middle Click)"),
            0xD4 => Some("Mouse Button 4 (Back)"),
            0xD5 => Some("Mouse Button 5 (Forward)"),
            0xD6 => Some("Mouse Wheel Up (Scroll Up)"),
            0xD7 => Some("Mouse Wheel Down (Scroll Down)"),
            0xD8 => Some("Mouse Wheel Left (Scroll Left)"),
            0xD9 => Some("Mouse Wheel Right (Scroll Right)"),
            _ => None,
        };
        text.map(str::to_string)
            .unwrap_or_else(|| format!("{} (0x{:04X})", self.name(), v))
    }
}

impl std::fmt::Display for BasicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The single-byte keycodes QMK defines (`0x00–0xFF`), with the canonical
/// names and values from `quantum/keycodes.h` (mouse keys are `QK_MOUSE_*`).
impl BasicKey {
    // Empty / transparent
    pub const KC_NO: Self = Self(0x00);
    pub const KC_TRANSPARENT: Self = Self(0x01);

    // Letters
    pub const KC_A: Self = Self(0x04);
    pub const KC_B: Self = Self(0x05);
    pub const KC_C: Self = Self(0x06);
    pub const KC_D: Self = Self(0x07);
    pub const KC_E: Self = Self(0x08);
    pub const KC_F: Self = Self(0x09);
    pub const KC_G: Self = Self(0x0A);
    pub const KC_H: Self = Self(0x0B);
    pub const KC_I: Self = Self(0x0C);
    pub const KC_J: Self = Self(0x0D);
    pub const KC_K: Self = Self(0x0E);
    pub const KC_L: Self = Self(0x0F);
    pub const KC_M: Self = Self(0x10);
    pub const KC_N: Self = Self(0x11);
    pub const KC_O: Self = Self(0x12);
    pub const KC_P: Self = Self(0x13);
    pub const KC_Q: Self = Self(0x14);
    pub const KC_R: Self = Self(0x15);
    pub const KC_S: Self = Self(0x16);
    pub const KC_T: Self = Self(0x17);
    pub const KC_U: Self = Self(0x18);
    pub const KC_V: Self = Self(0x19);
    pub const KC_W: Self = Self(0x1A);
    pub const KC_X: Self = Self(0x1B);
    pub const KC_Y: Self = Self(0x1C);
    pub const KC_Z: Self = Self(0x1D);

    // Numbers
    pub const KC_1: Self = Self(0x1E);
    pub const KC_2: Self = Self(0x1F);
    pub const KC_3: Self = Self(0x20);
    pub const KC_4: Self = Self(0x21);
    pub const KC_5: Self = Self(0x22);
    pub const KC_6: Self = Self(0x23);
    pub const KC_7: Self = Self(0x24);
    pub const KC_8: Self = Self(0x25);
    pub const KC_9: Self = Self(0x26);
    pub const KC_0: Self = Self(0x27);

    // Enter, escape & whitespace
    pub const KC_ENTER: Self = Self(0x28);
    pub const KC_ESCAPE: Self = Self(0x29);
    pub const KC_BACKSPACE: Self = Self(0x2A);
    pub const KC_TAB: Self = Self(0x2B);
    pub const KC_SPACE: Self = Self(0x2C);

    // Punctuation
    pub const KC_MINUS: Self = Self(0x2D);
    pub const KC_EQUAL: Self = Self(0x2E);
    pub const KC_LEFT_BRACKET: Self = Self(0x2F);
    pub const KC_RIGHT_BRACKET: Self = Self(0x30);
    pub const KC_BACKSLASH: Self = Self(0x31);
    pub const KC_NONUS_HASH: Self = Self(0x32);
    pub const KC_SEMICOLON: Self = Self(0x33);
    pub const KC_QUOTE: Self = Self(0x34);
    pub const KC_GRAVE: Self = Self(0x35);
    pub const KC_COMMA: Self = Self(0x36);
    pub const KC_DOT: Self = Self(0x37);
    pub const KC_SLASH: Self = Self(0x38);

    // Caps lock & F1–F12
    pub const KC_CAPS_LOCK: Self = Self(0x39);
    pub const KC_F1: Self = Self(0x3A);
    pub const KC_F2: Self = Self(0x3B);
    pub const KC_F3: Self = Self(0x3C);
    pub const KC_F4: Self = Self(0x3D);
    pub const KC_F5: Self = Self(0x3E);
    pub const KC_F6: Self = Self(0x3F);
    pub const KC_F7: Self = Self(0x40);
    pub const KC_F8: Self = Self(0x41);
    pub const KC_F9: Self = Self(0x42);
    pub const KC_F10: Self = Self(0x43);
    pub const KC_F11: Self = Self(0x44);
    pub const KC_F12: Self = Self(0x45);

    // Navigation & system
    pub const KC_PRINT_SCREEN: Self = Self(0x46);
    pub const KC_SCROLL_LOCK: Self = Self(0x47);
    pub const KC_PAUSE: Self = Self(0x48);
    pub const KC_INSERT: Self = Self(0x49);
    pub const KC_HOME: Self = Self(0x4A);
    pub const KC_PAGE_UP: Self = Self(0x4B);
    pub const KC_DELETE: Self = Self(0x4C);
    pub const KC_END: Self = Self(0x4D);
    pub const KC_PAGE_DOWN: Self = Self(0x4E);
    pub const KC_RIGHT: Self = Self(0x4F);
    pub const KC_LEFT: Self = Self(0x50);
    pub const KC_DOWN: Self = Self(0x51);
    pub const KC_UP: Self = Self(0x52);

    // Numpad
    pub const KC_NUM_LOCK: Self = Self(0x53);
    pub const KC_KP_SLASH: Self = Self(0x54);
    pub const KC_KP_ASTERISK: Self = Self(0x55);
    pub const KC_KP_MINUS: Self = Self(0x56);
    pub const KC_KP_PLUS: Self = Self(0x57);
    pub const KC_KP_ENTER: Self = Self(0x58);
    pub const KC_KP_1: Self = Self(0x59);
    pub const KC_KP_2: Self = Self(0x5A);
    pub const KC_KP_3: Self = Self(0x5B);
    pub const KC_KP_4: Self = Self(0x5C);
    pub const KC_KP_5: Self = Self(0x5D);
    pub const KC_KP_6: Self = Self(0x5E);
    pub const KC_KP_7: Self = Self(0x5F);
    pub const KC_KP_8: Self = Self(0x60);
    pub const KC_KP_9: Self = Self(0x61);
    pub const KC_KP_0: Self = Self(0x62);
    pub const KC_KP_DOT: Self = Self(0x63);

    // Misc
    pub const KC_NONUS_BACKSLASH: Self = Self(0x64);
    pub const KC_APPLICATION: Self = Self(0x65);
    pub const KC_KB_POWER: Self = Self(0x66);
    pub const KC_KP_EQUAL: Self = Self(0x67);

    // F13–F24
    pub const KC_F13: Self = Self(0x68);
    pub const KC_F14: Self = Self(0x69);
    pub const KC_F15: Self = Self(0x6A);
    pub const KC_F16: Self = Self(0x6B);
    pub const KC_F17: Self = Self(0x6C);
    pub const KC_F18: Self = Self(0x6D);
    pub const KC_F19: Self = Self(0x6E);
    pub const KC_F20: Self = Self(0x6F);
    pub const KC_F21: Self = Self(0x70);
    pub const KC_F22: Self = Self(0x71);
    pub const KC_F23: Self = Self(0x72);
    pub const KC_F24: Self = Self(0x73);

    // Editing & system control
    pub const KC_EXECUTE: Self = Self(0x74);
    pub const KC_HELP: Self = Self(0x75);
    pub const KC_MENU: Self = Self(0x76);
    pub const KC_SELECT: Self = Self(0x77);
    pub const KC_STOP: Self = Self(0x78);
    pub const KC_AGAIN: Self = Self(0x79);
    pub const KC_UNDO: Self = Self(0x7A);
    pub const KC_CUT: Self = Self(0x7B);
    pub const KC_COPY: Self = Self(0x7C);
    pub const KC_PASTE: Self = Self(0x7D);
    pub const KC_FIND: Self = Self(0x7E);
    pub const KC_KB_MUTE: Self = Self(0x7F);
    pub const KC_KB_VOLUME_UP: Self = Self(0x80);
    pub const KC_KB_VOLUME_DOWN: Self = Self(0x81);
    pub const KC_LOCKING_CAPS_LOCK: Self = Self(0x82);
    pub const KC_LOCKING_NUM_LOCK: Self = Self(0x83);
    pub const KC_LOCKING_SCROLL_LOCK: Self = Self(0x84);
    pub const KC_KP_COMMA: Self = Self(0x85);
    pub const KC_KP_EQUAL_AS400: Self = Self(0x86);

    // International
    pub const KC_INTERNATIONAL_1: Self = Self(0x87);
    pub const KC_INTERNATIONAL_2: Self = Self(0x88);
    pub const KC_INTERNATIONAL_3: Self = Self(0x89);
    pub const KC_INTERNATIONAL_4: Self = Self(0x8A);
    pub const KC_INTERNATIONAL_5: Self = Self(0x8B);
    pub const KC_INTERNATIONAL_6: Self = Self(0x8C);
    pub const KC_INTERNATIONAL_7: Self = Self(0x8D);
    pub const KC_INTERNATIONAL_8: Self = Self(0x8E);
    pub const KC_INTERNATIONAL_9: Self = Self(0x8F);

    // Language
    pub const KC_LANGUAGE_1: Self = Self(0x90);
    pub const KC_LANGUAGE_2: Self = Self(0x91);
    pub const KC_LANGUAGE_3: Self = Self(0x92);
    pub const KC_LANGUAGE_4: Self = Self(0x93);
    pub const KC_LANGUAGE_5: Self = Self(0x94);
    pub const KC_LANGUAGE_6: Self = Self(0x95);
    pub const KC_LANGUAGE_7: Self = Self(0x96);
    pub const KC_LANGUAGE_8: Self = Self(0x97);
    pub const KC_LANGUAGE_9: Self = Self(0x98);

    // Additional system
    pub const KC_ALTERNATE_ERASE: Self = Self(0x99);
    pub const KC_SYSTEM_REQUEST: Self = Self(0x9A);
    pub const KC_CANCEL: Self = Self(0x9B);
    pub const KC_CLEAR: Self = Self(0x9C);
    pub const KC_PRIOR: Self = Self(0x9D);
    pub const KC_RETURN: Self = Self(0x9E);
    pub const KC_SEPARATOR: Self = Self(0x9F);
    pub const KC_OUT: Self = Self(0xA0);
    pub const KC_OPER: Self = Self(0xA1);
    pub const KC_CLEAR_AGAIN: Self = Self(0xA2);
    pub const KC_CRSEL: Self = Self(0xA3);
    pub const KC_EXSEL: Self = Self(0xA4);

    // Media & consumer
    pub const KC_SYSTEM_POWER: Self = Self(0xA5);
    pub const KC_SYSTEM_SLEEP: Self = Self(0xA6);
    pub const KC_SYSTEM_WAKE: Self = Self(0xA7);
    pub const KC_AUDIO_MUTE: Self = Self(0xA8);
    pub const KC_AUDIO_VOL_UP: Self = Self(0xA9);
    pub const KC_AUDIO_VOL_DOWN: Self = Self(0xAA);
    pub const KC_MEDIA_NEXT_TRACK: Self = Self(0xAB);
    pub const KC_MEDIA_PREV_TRACK: Self = Self(0xAC);
    pub const KC_MEDIA_STOP: Self = Self(0xAD);
    pub const KC_MEDIA_PLAY_PAUSE: Self = Self(0xAE);
    pub const KC_MEDIA_SELECT: Self = Self(0xAF);
    pub const KC_MEDIA_EJECT: Self = Self(0xB0);
    pub const KC_MAIL: Self = Self(0xB1);
    pub const KC_CALCULATOR: Self = Self(0xB2);
    pub const KC_MY_COMPUTER: Self = Self(0xB3);
    pub const KC_WWW_SEARCH: Self = Self(0xB4);
    pub const KC_WWW_HOME: Self = Self(0xB5);
    pub const KC_WWW_BACK: Self = Self(0xB6);
    pub const KC_WWW_FORWARD: Self = Self(0xB7);
    pub const KC_WWW_STOP: Self = Self(0xB8);
    pub const KC_WWW_REFRESH: Self = Self(0xB9);
    pub const KC_WWW_FAVORITES: Self = Self(0xBA);
    pub const KC_MEDIA_FAST_FORWARD: Self = Self(0xBB);
    pub const KC_MEDIA_REWIND: Self = Self(0xBC);
    pub const KC_BRIGHTNESS_UP: Self = Self(0xBD);
    pub const KC_BRIGHTNESS_DOWN: Self = Self(0xBE);
    pub const KC_CONTROL_PANEL: Self = Self(0xBF);
    pub const KC_ASSISTANT: Self = Self(0xC0);
    pub const KC_MISSION_CONTROL: Self = Self(0xC1);
    pub const KC_LAUNCHPAD: Self = Self(0xC2);

    // Mouse
    pub const QK_MOUSE_CURSOR_UP: Self = Self(0xCD);
    pub const QK_MOUSE_CURSOR_DOWN: Self = Self(0xCE);
    pub const QK_MOUSE_CURSOR_LEFT: Self = Self(0xCF);
    pub const QK_MOUSE_CURSOR_RIGHT: Self = Self(0xD0);
    pub const QK_MOUSE_BUTTON_1: Self = Self(0xD1);
    pub const QK_MOUSE_BUTTON_2: Self = Self(0xD2);
    pub const QK_MOUSE_BUTTON_3: Self = Self(0xD3);
    pub const QK_MOUSE_BUTTON_4: Self = Self(0xD4);
    pub const QK_MOUSE_BUTTON_5: Self = Self(0xD5);
    pub const QK_MOUSE_BUTTON_6: Self = Self(0xD6);
    pub const QK_MOUSE_BUTTON_7: Self = Self(0xD7);
    pub const QK_MOUSE_BUTTON_8: Self = Self(0xD8);
    pub const QK_MOUSE_WHEEL_UP: Self = Self(0xD9);
    pub const QK_MOUSE_WHEEL_DOWN: Self = Self(0xDA);
    pub const QK_MOUSE_WHEEL_LEFT: Self = Self(0xDB);
    pub const QK_MOUSE_WHEEL_RIGHT: Self = Self(0xDC);
    pub const QK_MOUSE_ACCELERATION_0: Self = Self(0xDD);
    pub const QK_MOUSE_ACCELERATION_1: Self = Self(0xDE);
    pub const QK_MOUSE_ACCELERATION_2: Self = Self(0xDF);

    // Modifiers
    pub const KC_LEFT_CTRL: Self = Self(0xE0);
    pub const KC_LEFT_SHIFT: Self = Self(0xE1);
    pub const KC_LEFT_ALT: Self = Self(0xE2);
    pub const KC_LEFT_GUI: Self = Self(0xE3);
    pub const KC_RIGHT_CTRL: Self = Self(0xE4);
    pub const KC_RIGHT_SHIFT: Self = Self(0xE5);
    pub const KC_RIGHT_ALT: Self = Self(0xE6);
    pub const KC_RIGHT_GUI: Self = Self(0xE7);
}
