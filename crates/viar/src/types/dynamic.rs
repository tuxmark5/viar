use std::collections::HashMap;

use via_protocol::{
    ComboEntry,
    DynamicEntryCounts,
    KeyAction,
    KeyOverrideEntry,
    TapDanceEntry,
};

/// The kinds of dynamic entry a user can give a custom name to.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum DynamicKind {
    TapDance,
    Combo,
    KeyOverride,
}

impl DynamicKind {
    /// Stable persistence tag (`td` / `combo` / `ko`).
    fn tag(self) -> &'static str {
        match self {
            Self::TapDance => "td",
            Self::Combo => "combo",
            Self::KeyOverride => "ko",
        }
    }

    fn from_tag(tag: &str) -> Option<Self> {
        match tag {
            "td" => Some(Self::TapDance),
            "combo" => Some(Self::Combo),
            "ko" => Some(Self::KeyOverride),
            _ => None,
        }
    }
}

/// Identifies a nameable dynamic-entry slot — a `(kind, index)` pair. Used as the
/// key of the alias map; persisted as its `"td:0"` / `"combo:3"` string form.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct AliasKey {
    pub kind:  DynamicKind,
    pub index: usize,
}

impl AliasKey {
    pub fn tap_dance(index: usize) -> Self {
        Self { kind: DynamicKind::TapDance, index }
    }

    pub fn combo(index: usize) -> Self {
        Self { kind: DynamicKind::Combo, index }
    }

    pub fn key_override(index: usize) -> Self {
        Self { kind: DynamicKind::KeyOverride, index }
    }

    /// The default (un-aliased) display name, e.g. `TD(0)`, `C3`, `KO1`.
    pub fn default_name(self) -> String {
        match self.kind {
            DynamicKind::TapDance => format!("TD({})", self.index),
            DynamicKind::Combo => format!("C{}", self.index),
            DynamicKind::KeyOverride => format!("KO{}", self.index),
        }
    }
}

impl std::fmt::Display for AliasKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.kind.tag(), self.index)
    }
}

impl std::str::FromStr for AliasKey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let (tag, index) = s.split_once(':').ok_or(())?;
        Ok(Self {
            kind:  DynamicKind::from_tag(tag).ok_or(())?,
            index: index.parse().map_err(|_| ())?,
        })
    }
}

// Persisted (in the TOML config) as its string form, so config keys stay
// human-readable and backward-compatible with the old `HashMap<String, String>`.
impl serde::Serialize for AliasKey {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> serde::Deserialize<'de> for AliasKey {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = <&str>::deserialize(deserializer)?;
        s.parse()
            .map_err(|_| serde::de::Error::custom(format!("invalid alias key: {s:?}")))
    }
}

/// Identifies which keycode field is currently selected for the shared picker.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActiveKeycodeField {
    /// Tap dance field: (entry_index, field_name)
    TapDance(usize, TapDanceField),
    /// Combo field: (entry_index, field_variant)
    Combo(usize, ComboField),
    /// Key override field: (entry_index, field_variant)
    KeyOverride(usize, KeyOverrideField),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TapDanceField {
    OnTap,
    OnHold,
    OnDoubleTap,
    OnTapHold,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ComboField {
    Input(usize), // 0..3
    Output,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeyOverrideField {
    Trigger,
    Replacement,
}

/// Dynamic entry data loaded from the device.
pub struct DynamicEntryData {
    pub counts: DynamicEntryCounts,
    pub tap_dances: Vec<TapDanceEntry>,
    pub combos: Vec<ComboEntry>,
    pub key_overrides: Vec<KeyOverrideEntry>,
    /// Index of the entry currently being edited (per type)
    pub editing_tap_dance: Option<usize>,
    pub editing_combo: Option<usize>,
    pub editing_key_override: Option<usize>,
    /// Which keycode field is active for the shared picker
    pub active_field: Option<ActiveKeycodeField>,
    /// Which picker group tab is selected
    pub picker_group_idx: usize,
    /// User-defined names for dynamic entries, keyed by the slot they rename.
    pub aliases: HashMap<AliasKey, String>,
    /// Which alias is currently being edited inline.
    pub editing_alias: Option<AliasKey>,
}

impl DynamicEntryData {
    pub fn new(
        counts: DynamicEntryCounts,
        tap_dances: Vec<TapDanceEntry>,
        combos: Vec<ComboEntry>,
        key_overrides: Vec<KeyOverrideEntry>,
    ) -> Self {
        Self {
            counts,
            tap_dances,
            combos,
            key_overrides,
            editing_tap_dance: None,
            editing_combo: None,
            editing_key_override: None,
            active_field: None,
            picker_group_idx: 0,
            aliases: HashMap::new(),
            editing_alias: None,
        }
    }

    /// Display name for a slot: the user's alias if set (and non-empty),
    /// otherwise the slot's default name.
    pub fn slot_name(&self, key: AliasKey) -> String {
        self.aliases
            .get(&key)
            .filter(|s| !s.is_empty())
            .cloned()
            .unwrap_or_else(|| key.default_name())
    }

    /// Get the tap dance display name.
    pub fn td_name(&self, idx: usize) -> String {
        self.slot_name(AliasKey::tap_dance(idx))
    }

    /// Get the combo display name.
    pub fn combo_name(&self, idx: usize) -> String {
        self.slot_name(AliasKey::combo(idx))
    }

    /// Get the key override display name.
    pub fn ko_name(&self, idx: usize) -> String {
        self.slot_name(AliasKey::key_override(idx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alias_key_string_round_trip() {
        for key in [
            AliasKey::tap_dance(0),
            AliasKey::combo(3),
            AliasKey::key_override(12),
        ] {
            assert_eq!(key.to_string().parse::<AliasKey>(), Ok(key));
        }
        assert_eq!(AliasKey::tap_dance(0).to_string(), "td:0");
        assert_eq!(AliasKey::combo(3).to_string(), "combo:3");
        assert_eq!("ko:1".parse(), Ok(AliasKey::key_override(1)));
        assert!("bogus".parse::<AliasKey>().is_err());
    }

    #[test]
    fn alias_map_survives_toml_round_trip() {
        // The alias map is persisted in the TOML config, so `AliasKey` must
        // (de)serialize through TOML's string-keyed tables.
        let mut aliases: HashMap<AliasKey, String> = HashMap::new();
        aliases.insert(AliasKey::tap_dance(0), "Copy".to_string());
        aliases.insert(AliasKey::combo(3), "Paste".to_string());

        let toml = toml::to_string(&aliases).unwrap();
        let back: HashMap<AliasKey, String> = toml::from_str(&toml).unwrap();
        assert_eq!(aliases, back);
    }
}

/// Display label for a keycode action — the user's custom tap-dance name when the
/// action is a tap-dance and one is set, otherwise the action's own name. A free
/// function (not a [`DynamicEntryData`] method) so callers can pass a cloned alias
/// map and not hold a borrow on the whole entry data.
pub fn action_label(action: KeyAction, aliases: Option<&HashMap<AliasKey, String>>) -> String {
    if let (KeyAction::TapDance(td), Some(aliases)) = (action, aliases)
        && let Some(name) = aliases
            .get(&AliasKey::tap_dance(td.0 as usize))
            .filter(|s| !s.is_empty())
    {
        return name.clone();
    }
    action.to_string()
}
