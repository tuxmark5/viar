//! The keymap picker's "Layers" tab: a two-step selector — pick a layer-op
//! kind (MO/TG/TO/DF/OSL/TT/PDF) or OSM, then a destination layer / modifier —
//! instead of a flat grid of every layer-op × every layer.

use eframe::egui;
use via_protocol::{
    KeyAction,
    KeycodeGroup,
    LayerId,
    ModMask,
};

/// Number of destination layers offered by the [Layers picker](render_layer_picker).
const LAYER_PICKER_COUNT: u8 = 16;

/// Layer-op kinds that take a destination layer: display label, tooltip
/// description, and constructor. Their order is the "kind index" used throughout
/// the layer picker.
const LAYER_KINDS: [(&str, &str, fn(LayerId) -> KeyAction); 7] = [
    ("MO", "Momentary — active while the key is held", KeyAction::Momentary),
    ("TG", "Toggle the layer on/off", KeyAction::ToggleLayer),
    ("TO", "Activate the layer, turning off all others", KeyAction::ToLayer),
    ("DF", "Set the default base layer", KeyAction::DefLayer),
    ("OSL", "One-shot layer — active for the next keypress only", KeyAction::OneShotLayer),
    ("TT", "Tap-toggle — tap to toggle, hold for momentary", KeyAction::TapToggleLayer),
    ("PDF", "Persistent default layer — survives reboot", KeyAction::PersistentDefLayer),
];

/// Kind index of the one-shot-modifier option (past the layer-op kinds).
const OSM_KIND: usize = LAYER_KINDS.len();

/// Tooltip for the OSM kind button.
const OSM_DESC: &str = "One-shot modifier — applies to the next keypress only";

/// One-shot-modifier presets shown when the OSM kind is selected.
const OSM_MASKS: [ModMask; 14] = [
    ModMask::CTRL,
    ModMask::SHIFT,
    ModMask::ALT,
    ModMask::GUI,
    ModMask::RCTRL,
    ModMask::RSHIFT,
    ModMask::RALT,
    ModMask::RGUI,
    ModMask::CTRL.and(ModMask::SHIFT),
    ModMask::CTRL.and(ModMask::ALT),
    ModMask::CTRL.and(ModMask::GUI),
    ModMask::SHIFT.and(ModMask::ALT),
    ModMask::SHIFT.and(ModMask::GUI),
    ModMask::ALT.and(ModMask::GUI),
];

/// Whether a picker group is the special "Layers" group (rendered as a kind +
/// destination-layer selector rather than a flat grid).
pub fn is_layers_group(group: &KeycodeGroup) -> bool {
    group.name == "Layers"
}

/// The layer-picker kind index a current action already is, so re-opening the
/// picker lands on the right kind. `None` for non-layer actions.
fn layer_kind_index(action: KeyAction) -> Option<usize> {
    Some(match action {
        KeyAction::Momentary(_) => 0,
        KeyAction::ToggleLayer(_) => 1,
        KeyAction::ToLayer(_) => 2,
        KeyAction::DefLayer(_) => 3,
        KeyAction::OneShotLayer(_) => 4,
        KeyAction::TapToggleLayer(_) => 5,
        KeyAction::PersistentDefLayer(_) => 6,
        KeyAction::OneShotMod(_) => OSM_KIND,
        _ => return None,
    })
}

/// A single picker cell (rounded button, highlighted when `selected`). Returns
/// its click response.
fn picker_cell(ui: &mut egui::Ui, label: &str, selected: bool, size: egui::Vec2) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    let bg = if selected {
        egui::Color32::from_rgb(70, 130, 180)
    } else if response.hovered() {
        egui::Color32::from_rgb(70, 70, 80)
    } else {
        egui::Color32::from_rgb(42, 42, 48)
    };
    let text_col = if selected {
        egui::Color32::WHITE
    } else {
        egui::Color32::from_rgb(190, 190, 200)
    };
    let rounding = egui::CornerRadius::same(3);
    ui.painter().rect_filled(rect, rounding, bg);
    if selected {
        ui.painter().rect_stroke(
            rect,
            rounding,
            egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(100, 180, 255)),
            egui::StrokeKind::Outside,
        );
    }
    let font_size = if label.len() <= 2 {
        11.0
    } else if label.len() <= 5 {
        9.5
    } else {
        8.0
    };
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(font_size),
        text_col,
    );
    response
}

/// Section heading inside the layer picker.
fn layer_picker_heading(ui: &mut egui::Ui, text: &str) {
    ui.label(
        egui::RichText::new(text)
            .size(13.0)
            .color(egui::Color32::from_rgb(140, 140, 155)),
    );
}

/// The Layers picker: choose a layer-op *kind* (MO/TG/TO/DF/OSL/TT/PDF or OSM),
/// then a *destination layer* (or, for OSM, a modifier). Returns the built action
/// when the user clicks a destination. The selected kind persists in egui memory.
pub fn render_layer_picker(ui: &mut egui::Ui, current: KeyAction) -> Option<KeyAction> {
    let kind_id = ui.id().with("layer_picker_kind");
    let mut kind: usize = ui
        .memory(|m| m.data.get_temp(kind_id))
        .or_else(|| layer_kind_index(current))
        .unwrap_or(0);

    let mut result = None;

    // Step 1: pick the kind.
    layer_picker_heading(ui, "Layer action");
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(3.0, 3.0);
        for (i, (label, desc, _)) in LAYER_KINDS.iter().enumerate() {
            if picker_cell(ui, label, kind == i, egui::vec2(42.0, 24.0))
                .on_hover_text(*desc)
                .clicked()
            {
                kind = i;
            }
        }
        if picker_cell(ui, "OSM", kind == OSM_KIND, egui::vec2(42.0, 24.0))
            .on_hover_text(OSM_DESC)
            .clicked()
        {
            kind = OSM_KIND;
        }
    });
    ui.memory_mut(|m| m.data.insert_temp(kind_id, kind));

    ui.add_space(8.0);

    // Step 2: pick the destination (a layer, or a modifier for OSM).
    if kind == OSM_KIND {
        layer_picker_heading(ui, "One-shot modifier");
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(3.0, 3.0);
            for mask in OSM_MASKS {
                let action = KeyAction::OneShotMod(mask);
                let resp = picker_cell(
                    ui,
                    &mask.to_string(),
                    current == action,
                    egui::vec2(44.0, 26.0),
                );
                if resp.clicked() {
                    result = Some(action);
                }
                resp.on_hover_text(action.to_string());
            }
        });
    } else {
        let (label, _, ctor) = LAYER_KINDS[kind];
        layer_picker_heading(ui, &format!("{label} — destination layer"));
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(3.0, 3.0);
            for n in 0..LAYER_PICKER_COUNT {
                let action = ctor(LayerId(n));
                let resp = picker_cell(
                    ui,
                    &n.to_string(),
                    current == action,
                    egui::vec2(28.0, 26.0),
                );
                if resp.clicked() {
                    result = Some(action);
                }
                resp.on_hover_text(action.to_string());
            }
        });
    }

    result
}
