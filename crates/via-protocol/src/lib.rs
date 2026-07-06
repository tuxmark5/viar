//! VIA/Vial keyboard configuration protocol over USB HID.
//!
//! This crate provides device discovery and communication with keyboards
//! running QMK firmware with VIA or Vial enabled.

pub mod basic_key;
mod command;
pub mod device;
pub mod encoding;
mod error;
pub mod key_action;
pub mod keycode_groups;
mod keycode_macros;
mod keycodes;
pub mod layout;
pub mod mod_mask;
mod protocol;
mod qmk_names;
pub mod quantum_key;
pub mod rgb_key;
pub mod tap_dance_key;

pub use basic_key::BasicKey;
pub use command::{
    ComboEntry,
    DynamicEntryCounts,
    KeyOverrideEntry,
    LightingChannel,
    LightingProtocol,
    QmkSettingDescriptor,
    RgbValueId,
    TapDanceEntry,
    ViaCommand,
    ViaCommandId,
    VialRgbEffect,
    VialRgbValueId,
    pointing_settings,
};
pub use device::{
    HidAccessStatus,
    KeyboardDevice,
    KeyboardInfo,
};
pub use encoding::{
    KeycodeEncoding,
    KeycodeEncodingRef,
    NewEncoding,
    OldEncoding,
    encoding_for_protocol,
};
pub use error::{
    ViaError,
    ViaResult,
};
pub use key_action::{
    KeyAction,
    LayerId,
};
pub use keycode_groups::{
    KeycodeGroup,
    all_basic_keycodes,
    keycode_groups,
};
pub use keycodes::{
    Keycode,
    KeycodeCategory,
    QuantumKeyType,
    mod_mask_to_string,
    mod_tap_prefix,
    quantum_key_types,
    quantum_keycode_defaults,
};
pub use layout::{
    EncoderKey,
    EncoderPosition,
    KeyPosition,
    KeyboardLayout,
    parse_vial_definition,
};
pub use mod_mask::ModMask;
pub use protocol::{
    LightingValues,
    ViaProtocol,
    VialRgbInfo,
};
pub use quantum_key::QuantumKey;
pub use rgb_key::RgbKey;
pub use tap_dance_key::TapDanceKey;

/// VIA HID usage page used to identify VIA-enabled keyboards.
pub const VIA_USAGE_PAGE: u16 = 0xFF60;

/// VIA HID usage ID.
pub const VIA_USAGE: u16 = 0x61;

/// Standard VIA HID report size in bytes.
pub const VIA_REPORT_SIZE: usize = 32;
