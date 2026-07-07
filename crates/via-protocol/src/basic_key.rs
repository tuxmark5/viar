//! A basic HID keycode (`0x00–0xFF`) and named constants for the common ones.

use crate::{
    keycode_macros::basic_keys,
    keycodes::KeycodeCategory,
};

/// A basic HID keycode (`0x00–0xFF`): letters, mods, media, mouse, NONE/TRNS.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BasicKey(pub u8);

impl BasicKey {
    /// The canonical QMK name, e.g. `KC_A`, `KC_SEMICOLON`.
    pub fn qmk_name(self) -> Option<&'static str> {
        crate::qmk_names::qmk_keycode_name(self.0 as u16)
    }

    /// Broad category of this basic HID keycode, for picker grouping and keycap
    /// coloring.
    pub fn category(self) -> KeycodeCategory {
        match self.0 {
            0x00 => KeycodeCategory::None,
            0x01 => KeycodeCategory::Transparent,
            // Mouse keys occupy 0x00CD–0x00D9 in QMK.
            0xCD..=0xD9 => KeycodeCategory::Mouse,
            _ => KeycodeCategory::Basic,
        }
    }
}

// Constants (canonical `KC_*` names), `name()` (short display) and
// `description()`, generated from `0xVAL => "SHORT", CANONICAL, "DESC"`.
basic_keys! {
    0x00 => "NONE", KC_NO, "No action (transparent to layers below)",
    0x01 => "TRNS", KC_TRANSPARENT, "Transparent — falls through to the layer below",

    0x04 => "A", KC_A, "Character A",
    0x05 => "B", KC_B, "Character B",
    0x06 => "C", KC_C, "Character C",
    0x07 => "D", KC_D, "Character D",
    0x08 => "E", KC_E, "Character E",
    0x09 => "F", KC_F, "Character F",
    0x0A => "G", KC_G, "Character G",
    0x0B => "H", KC_H, "Character H",
    0x0C => "I", KC_I, "Character I",
    0x0D => "J", KC_J, "Character J",
    0x0E => "K", KC_K, "Character K",
    0x0F => "L", KC_L, "Character L",
    0x10 => "M", KC_M, "Character M",
    0x11 => "N", KC_N, "Character N",
    0x12 => "O", KC_O, "Character O",
    0x13 => "P", KC_P, "Character P",
    0x14 => "Q", KC_Q, "Character Q",
    0x15 => "R", KC_R, "Character R",
    0x16 => "S", KC_S, "Character S",
    0x17 => "T", KC_T, "Character T",
    0x18 => "U", KC_U, "Character U",
    0x19 => "V", KC_V, "Character V",
    0x1A => "W", KC_W, "Character W",
    0x1B => "X", KC_X, "Character X",
    0x1C => "Y", KC_Y, "Character Y",
    0x1D => "Z", KC_Z, "Character Z",

    0x1E => "1", KC_1, "Number 1",
    0x1F => "2", KC_2, "Number 2",
    0x20 => "3", KC_3, "Number 3",
    0x21 => "4", KC_4, "Number 4",
    0x22 => "5", KC_5, "Number 5",
    0x23 => "6", KC_6, "Number 6",
    0x24 => "7", KC_7, "Number 7",
    0x25 => "8", KC_8, "Number 8",
    0x26 => "9", KC_9, "Number 9",
    0x27 => "0", KC_0, "Number 0",

    0x28 => "Enter", KC_ENTER, "Enter / Return",
    0x29 => "Esc", KC_ESCAPE, "Escape",
    0x2A => "Bksp", KC_BACKSPACE, "Backspace",
    0x2B => "Tab", KC_TAB, "Tab",
    0x2C => "Space", KC_SPACE, "Spacebar",
    0x2D => "-", KC_MINUS, "Minus / Hyphen",
    0x2E => "=", KC_EQUAL, "Equals",
    0x2F => "[", KC_LEFT_BRACKET, "Left Bracket",
    0x30 => "]", KC_RIGHT_BRACKET, "Right Bracket",
    0x31 => "\\", KC_BACKSLASH, "Backslash",
    0x33 => ";", KC_SEMICOLON, "Semicolon",
    0x34 => "'", KC_QUOTE, "Apostrophe / Quote",
    0x35 => "`", KC_GRAVE, "Grave / Backtick",
    0x36 => ",", KC_COMMA, "Comma",
    0x37 => ".", KC_DOT, "Period / Dot",
    0x38 => "/", KC_SLASH, "Forward Slash",
    0x39 => "CapsLk", KC_CAPS_LOCK, "Caps Lock",

    0x3A => "F1", KC_F1, "Function key F1",
    0x3B => "F2", KC_F2, "Function key F2",
    0x3C => "F3", KC_F3, "Function key F3",
    0x3D => "F4", KC_F4, "Function key F4",
    0x3E => "F5", KC_F5, "Function key F5",
    0x3F => "F6", KC_F6, "Function key F6",
    0x40 => "F7", KC_F7, "Function key F7",
    0x41 => "F8", KC_F8, "Function key F8",
    0x42 => "F9", KC_F9, "Function key F9",
    0x43 => "F10", KC_F10, "Function key F10",
    0x44 => "F11", KC_F11, "Function key F11",
    0x45 => "F12", KC_F12, "Function key F12",

    0x46 => "PrtSc", KC_PRINT_SCREEN, "Print Screen",
    0x47 => "ScrLk", KC_SCROLL_LOCK, "Scroll Lock",
    0x48 => "Pause", KC_PAUSE, "Pause / Break",
    0x49 => "Ins", KC_INSERT, "Insert",
    0x4A => "Home", KC_HOME, "Home",
    0x4B => "PgUp", KC_PAGE_UP, "Page Up",
    0x4C => "Del", KC_DELETE, "Delete",
    0x4D => "End", KC_END, "End",
    0x4E => "PgDn", KC_PAGE_DOWN, "Page Down",
    0x4F => "→", KC_RIGHT, "Right Arrow",
    0x50 => "←", KC_LEFT, "Left Arrow",
    0x51 => "↓", KC_DOWN, "Down Arrow",
    0x52 => "↑", KC_UP, "Up Arrow",

    0x53 => "NLck", KC_NUM_LOCK, "Num Lock",
    0x54 => "N/", KC_KP_SLASH, "Numpad Divide",
    0x55 => "N*", KC_KP_ASTERISK, "Numpad Multiply",
    0x56 => "N-", KC_KP_MINUS, "Numpad Minus",
    0x57 => "N+", KC_KP_PLUS, "Numpad Plus",
    0x58 => "NEnt", KC_KP_ENTER, "Numpad Enter",
    0x59 => "N1", KC_KP_1, "Numpad 1",
    0x5A => "N2", KC_KP_2, "Numpad 2",
    0x5B => "N3", KC_KP_3, "Numpad 3",
    0x5C => "N4", KC_KP_4, "Numpad 4",
    0x5D => "N5", KC_KP_5, "Numpad 5",
    0x5E => "N6", KC_KP_6, "Numpad 6",
    0x5F => "N7", KC_KP_7, "Numpad 7",
    0x60 => "N8", KC_KP_8, "Numpad 8",
    0x61 => "N9", KC_KP_9, "Numpad 9",
    0x62 => "N0", KC_KP_0, "Numpad 0",
    0x63 => "N.", KC_KP_DOT, "Numpad Decimal",

    0x65 => "App", KC_APPLICATION, "Application / Menu key",
    0x66 => "Power", KC_KB_POWER, "System Power",

    0x68 => "F13", KC_F13, "Function key F13",
    0x69 => "F14", KC_F14, "Function key F14",
    0x6A => "F15", KC_F15, "Function key F15",
    0x6B => "F16", KC_F16, "Function key F16",
    0x6C => "F17", KC_F17, "Function key F17",
    0x6D => "F18", KC_F18, "Function key F18",
    0x6E => "F19", KC_F19, "Function key F19",
    0x6F => "F20", KC_F20, "Function key F20",
    0x70 => "F21", KC_F21, "Function key F21",
    0x71 => "F22", KC_F22, "Function key F22",
    0x72 => "F23", KC_F23, "Function key F23",
    0x73 => "F24", KC_F24, "Function key F24",

    0x87 => "RO", KC_INTERNATIONAL_1, "International 1 (Ro)",
    0x88 => "Kana", KC_INTERNATIONAL_2, "International 2 (Katakana)",
    0x89 => "Yen", KC_INTERNATIONAL_3, "International 3 (Yen)",
    0x8A => "Henk", KC_INTERNATIONAL_4, "International 4 (Henkan)",
    0x8B => "Mhen", KC_INTERNATIONAL_5, "International 5 (Muhenkan)",
    0x90 => "HGL", KC_LANGUAGE_1, "Language 1 (Hangul)",
    0x91 => "Hanja", KC_LANGUAGE_2, "Language 2 (Hanja)",

    0xA5 => "SysPwr", KC_SYSTEM_POWER, "System Power",
    0xA6 => "Sleep", KC_SYSTEM_SLEEP, "System Sleep",
    0xA7 => "Wake", KC_SYSTEM_WAKE, "System Wake",
    0xA8 => "Mute", KC_AUDIO_MUTE, "Mute audio",
    0xA9 => "VolUp", KC_AUDIO_VOL_UP, "Volume Up",
    0xAA => "VolDn", KC_AUDIO_VOL_DOWN, "Volume Down",
    0xAB => "MNext", KC_MEDIA_NEXT_TRACK, "Next Track",
    0xAC => "MPrev", KC_MEDIA_PREV_TRACK, "Previous Track",
    0xAD => "MStop", KC_MEDIA_STOP, "Media Stop",
    0xAE => "MPlay", KC_MEDIA_PLAY_PAUSE, "Media Play/Pause",
    0xAF => "MSel", KC_MEDIA_SELECT, "Media Select",
    0xB0 => "MEjct", KC_MEDIA_EJECT, "Media Eject",
    0xB1 => "Mail", KC_MAIL, "Launch Mail",
    0xB2 => "Calc", KC_CALCULATOR, "Launch Calculator",
    0xB3 => "MyCmp", KC_MY_COMPUTER, "Launch My Computer",
    0xB4 => "WSrch", KC_WWW_SEARCH, "Browser Search",
    0xB5 => "WHome", KC_WWW_HOME, "Browser Home",
    0xB6 => "WBack", KC_WWW_BACK, "Browser Back",
    0xB7 => "WFwd", KC_WWW_FORWARD, "Browser Forward",
    0xB8 => "WStop", KC_WWW_STOP, "Browser Stop",
    0xB9 => "WRfsh", KC_WWW_REFRESH, "Browser Refresh",
    0xBA => "WFav", KC_WWW_FAVORITES, "Browser Favorites",
    0xBB => "MFFwd", KC_MEDIA_FAST_FORWARD, "Fast Forward",

    0xCD => "MsUp", QK_MOUSE_CURSOR_UP, "Mouse Cursor Up",
    0xCE => "MsDn", QK_MOUSE_CURSOR_DOWN, "Mouse Cursor Down",
    0xCF => "MsLt", QK_MOUSE_CURSOR_LEFT, "Mouse Cursor Left",
    0xD0 => "MsRt", QK_MOUSE_CURSOR_RIGHT, "Mouse Cursor Right",
    0xD1 => "Btn1", QK_MOUSE_BUTTON_1, "Mouse Button 1 (Left Click)",
    0xD2 => "Btn2", QK_MOUSE_BUTTON_2, "Mouse Button 2 (Right Click)",
    0xD3 => "Btn3", QK_MOUSE_BUTTON_3, "Mouse Button 3 (Middle Click)",
    0xD4 => "Btn4", QK_MOUSE_BUTTON_4, "Mouse Button 4 (Back)",
    0xD5 => "Btn5", QK_MOUSE_BUTTON_5, "Mouse Button 5 (Forward)",
    0xD6 => "WhUp", QK_MOUSE_BUTTON_6, "Mouse Wheel Up (Scroll Up)",
    0xD7 => "WhDn", QK_MOUSE_BUTTON_7, "Mouse Wheel Down (Scroll Down)",
    0xD8 => "WhLt", QK_MOUSE_BUTTON_8, "Mouse Wheel Left (Scroll Left)",
    0xD9 => "WhRt", QK_MOUSE_WHEEL_UP, "Mouse Wheel Right (Scroll Right)",

    0xE0 => "LCtrl", KC_LEFT_CTRL, "Left Control",
    0xE1 => "LShft", KC_LEFT_SHIFT, "Left Shift",
    0xE2 => "LAlt", KC_LEFT_ALT, "Left Alt / Option",
    0xE3 => "LGui", KC_LEFT_GUI, "Left GUI / Super / Command",
    0xE4 => "RCtrl", KC_RIGHT_CTRL, "Right Control",
    0xE5 => "RShft", KC_RIGHT_SHIFT, "Right Shift",
    0xE6 => "RAlt", KC_RIGHT_ALT, "Right Alt / AltGr",
    0xE7 => "RGui", KC_RIGHT_GUI, "Right GUI / Super / Command",
}

