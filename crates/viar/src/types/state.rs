use std::sync::mpsc;

use via_protocol::{
    KeyAction,
    KeyboardDevice,
    KeyboardInfo,
    KeycodeEncodingRef,
    KeycodeGroup,
};

use super::{
    AppScreen,
    ConfirmDialog,
    ConnectedTab,
    DynamicEntryData,
    KeyFlash,
    KeymapData,
    KeymapLayer,
    LayerFlash,
    LightingData,
    PointingData,
    QmkSettingsData,
    StatusMessage,
};
use crate::theme::{
    Theme,
    ViarConfig,
};

/// Result of background HID detection.
pub enum DetectResult {
    Ok {
        api:       hidapi::HidApi,
        keyboards: Vec<KeyboardInfo>,
    },
    NoPermission,
    NoViaDevices,
    InitFailed(String),
}

/// The main application state.
pub struct ViarApp {
    pub hid_api: Option<hidapi::HidApi>,
    pub keyboards: Vec<KeyboardInfo>,
    pub connected_device: Option<KeyboardDevice>,
    pub protocol_version: Option<u16>,
    /// Keycode encoding scheme for the connected board (from its VIA protocol
    /// version); used to decode/encode raw keycodes at the device boundary.
    pub encoding: KeycodeEncodingRef,
    pub screen: AppScreen,
    pub keymap_data: Option<KeymapData>,
    /// Picker state
    pub picker_groups: Vec<KeycodeGroup>,
    pub picker_selected_group: usize,
    /// Status bar message
    pub status: Option<StatusMessage>,
    /// Pending confirmation dialog
    pub confirm_dialog: Option<ConfirmDialog>,
    /// Active tab in connected view
    pub active_tab: ConnectedTab,
    /// Lighting state
    pub lighting_data: Option<LightingData>,
    /// Dynamic entries (tap dance, combos, key overrides)
    pub dynamic_data: Option<DynamicEntryData>,
    /// Pointing device / trackpad settings
    pub pointing_data: Option<PointingData>,
    /// QMK Settings (core settings: tapping, autoshift, magic, etc.)
    pub qmk_settings_data: Option<QmkSettingsData>,
    /// Receiver for background HID detection result
    pub detect_rx: Option<mpsc::Receiver<DetectResult>>,
    /// Persistent config
    pub config: ViarConfig,
    /// Active theme
    pub theme: Theme,
    /// Vial firmware protocol version (separate from VIA protocol version)
    pub vial_protocol_version: Option<u32>,
    /// Vial keyboard UID
    pub vial_uid: Option<[u8; 8]>,
    /// QMK firmware version (from GetKeyboardValue)
    pub firmware_version: Option<u32>,
    /// Keyboard uptime at connection time (milliseconds)
    pub connect_uptime_ms: Option<u32>,
    /// Detected QMK features based on protocol probing
    pub detected_features: Vec<String>,
    /// User-generated quantum keycodes (mod-tap, OSM, etc.) for quick picker access
    pub quantum_favorites: Vec<KeyAction>,
    /// Action copied from a slot (shift+right-click) for pasting (shift+left-click)
    pub copied_keycode: Option<KeyAction>,
    /// A whole layer copied (shift+right-click a layer tab) for pasting into another.
    pub copied_layer: Option<KeymapLayer>,
    /// Copy/paste flash animation on a key slot (idle when its `start` is None).
    pub key_flash: KeyFlash,
    /// Copy/paste flash animation on a layer tab (idle when its `start` is None).
    pub layer_flash: LayerFlash,
}
