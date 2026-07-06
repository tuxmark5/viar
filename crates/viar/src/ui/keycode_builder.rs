//! Keycode builder UI for the keymap picker: constructs composite keycodes
//! (LT, MT, Mod+Key, OSM) from user-selected layers, modifiers, and base keys.

use eframe::egui;
use via_protocol::Keycode;

/// "Result:" preview text in the keycode builder.
const COL_RESULT: egui::Color32 = egui::Color32::from_rgb(140, 200, 140);
/// Hint / helper text in the keycode builder.
const COL_HINT: egui::Color32 = egui::Color32::from_rgb(110, 110, 125);

/// Render the keycode builder UI (LT, MT, Mod+Key, OSM).
/// Returns Some(keycode) if the user constructed and applied a keycode.
/// Free function to avoid borrow conflicts with self in keymap_tab closures.
pub(crate) fn render_keycode_builder(ui: &mut egui::Ui, current_kc: u16) -> Option<u16> {
    let builder_type_id = egui::Id::new("builder_type");
    let builder_layer_id = egui::Id::new("builder_layer");
    let builder_mods_id = egui::Id::new("builder_mods");
    let builder_key_id = egui::Id::new("builder_key");

    // Builder type selector
    let mut builder_type: usize = ui
        .memory(|mem| mem.data.get_temp(builder_type_id))
        .unwrap_or(0);

    ui.horizontal(|ui| {
        let types = ["LT(layer,key)", "MT(mod,key)", "Mod+Key", "OSM(mod)"];
        for (i, name) in types.iter().enumerate() {
            let sel = builder_type == i;
            let text = egui::RichText::new(*name).size(14.5);
            if ui.selectable_label(sel, text).clicked() {
                builder_type = i;
                ui.memory_mut(|mem| mem.data.insert_temp(builder_type_id, builder_type));
            }
        }
    });

    ui.add_space(4.0);

    // State
    let kc = Keycode(current_kc);
    let mut layer: u8 = ui
        .memory(|mem| mem.data.get_temp::<u8>(builder_layer_id))
        .unwrap_or_else(|| kc.layer());
    let mut mods: u8 = ui
        .memory(|mem| mem.data.get_temp::<u8>(builder_mods_id))
        .unwrap_or_else(|| kc.mod_mask());
    let mut base_key: u16 = ui
        .memory(|mem| mem.data.get_temp::<u16>(builder_key_id))
        .unwrap_or_else(|| kc.base_keycode() as u16);

    let mut result: Option<u16> = None;

    match builder_type {
        0 => {
            // LT(layer, key)
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Layer:").size(15.0));
                let mut l = layer as f32;
                if ui
                    .add(egui::Slider::new(&mut l, 0.0..=15.0).integer())
                    .changed()
                {
                    layer = l as u8;
                }
            });
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Tap Key:").size(15.0));
                let name = if base_key == 0 {
                    "---".to_string()
                } else {
                    Keycode(base_key).name()
                };
                ui.label(egui::RichText::new(&name).monospace().size(15.0));
            });
            // Quick key selector
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(2.0, 2.0);
                // Letters
                for k in 0x04u16..=0x1Du16 {
                    let n = Keycode(k).name();
                    if ui.small_button(&n).clicked() {
                        base_key = k;
                    }
                }
                // Numbers
                for k in 0x1Eu16..=0x27u16 {
                    let n = Keycode(k).name();
                    if ui.small_button(&n).clicked() {
                        base_key = k;
                    }
                }
                // Symbols
                for &k in &[
                    0x2Du16, 0x2E, 0x2F, 0x30, 0x31, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38,
                ] {
                    let n = Keycode(k).name();
                    if ui.small_button(&n).clicked() {
                        base_key = k;
                    }
                }
                // Editing keys
                for &k in &[0x28u16, 0x2C, 0x29, 0x2A, 0x2B] {
                    let n = Keycode(k).name();
                    if ui.small_button(&n).clicked() {
                        base_key = k;
                    }
                }
            });
            ui.add_space(4.0);
            let preview = Keycode::layer_tap(layer, base_key as u8);
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(format!(
                        "Result: {} (0x{:04X})",
                        preview.name(),
                        preview.raw()
                    ))
                    .monospace()
                    .size(15.0)
                    .color(COL_RESULT),
                );
                if ui.button("Apply").clicked() {
                    result = Some(preview.raw());
                }
            });
        }
        1 => {
            // MT(mod, key)
            ui.label(egui::RichText::new("Hold Modifiers:").size(15.0));
            render_mod_checkboxes(ui, &mut mods);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Tap Key:").size(15.0));
                let name = if base_key == 0 {
                    "---".to_string()
                } else {
                    Keycode(base_key).name()
                };
                ui.label(egui::RichText::new(&name).monospace().size(15.0));
            });
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(2.0, 2.0);
                // Letters
                for k in 0x04u16..=0x1Du16 {
                    let n = Keycode(k).name();
                    if ui.small_button(&n).clicked() {
                        base_key = k;
                    }
                }
                // Numbers
                for k in 0x1Eu16..=0x27u16 {
                    let n = Keycode(k).name();
                    if ui.small_button(&n).clicked() {
                        base_key = k;
                    }
                }
                // Symbols
                for &k in &[
                    0x2Du16, 0x2E, 0x2F, 0x30, 0x31, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38,
                ] {
                    let n = Keycode(k).name();
                    if ui.small_button(&n).clicked() {
                        base_key = k;
                    }
                }
                // Editing keys
                for &k in &[0x28u16, 0x2C, 0x29, 0x2A, 0x2B] {
                    let n = Keycode(k).name();
                    if ui.small_button(&n).clicked() {
                        base_key = k;
                    }
                }
            });
            ui.add_space(4.0);
            let preview = Keycode::mod_tap(mods, base_key as u8);
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(format!(
                        "Result: {} (0x{:04X})",
                        preview.name(),
                        preview.raw()
                    ))
                    .monospace()
                    .size(15.0)
                    .color(COL_RESULT),
                );
                if ui.button("Apply").clicked() {
                    result = Some(preview.raw());
                }
            });
        }
        2 => {
            // Mod+Key
            ui.label(egui::RichText::new("Modifiers:").size(15.0));
            render_mod_checkboxes(ui, &mut mods);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Key:").size(15.0));
                let name = if base_key == 0 {
                    "---".to_string()
                } else {
                    Keycode(base_key).name()
                };
                ui.label(egui::RichText::new(&name).monospace().size(15.0));
            });
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(2.0, 2.0);
                // Letters
                for k in 0x04u16..=0x1Du16 {
                    let n = Keycode(k).name();
                    if ui.small_button(&n).clicked() {
                        base_key = k;
                    }
                }
                // Numbers
                for k in 0x1Eu16..=0x27u16 {
                    let n = Keycode(k).name();
                    if ui.small_button(&n).clicked() {
                        base_key = k;
                    }
                }
                // Symbols
                for &k in &[
                    0x2Du16, 0x2E, 0x2F, 0x30, 0x31, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38,
                ] {
                    let n = Keycode(k).name();
                    if ui.small_button(&n).clicked() {
                        base_key = k;
                    }
                }
                // Editing keys
                for &k in &[0x28u16, 0x2C, 0x29, 0x2A, 0x2B] {
                    let n = Keycode(k).name();
                    if ui.small_button(&n).clicked() {
                        base_key = k;
                    }
                }
            });
            ui.add_space(4.0);
            let preview = Keycode::mod_key(mods, base_key as u8);
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(format!(
                        "Result: {} (0x{:04X})",
                        preview.name(),
                        preview.raw()
                    ))
                    .monospace()
                    .size(15.0)
                    .color(COL_RESULT),
                );
                if ui.button("Apply").clicked() {
                    result = Some(preview.raw());
                }
            });
        }
        3 => {
            // OSM(mod)
            ui.label(egui::RichText::new("One-Shot Modifier:").size(15.0));
            ui.label(
                egui::RichText::new("Applies modifier to the next keypress only.")
                    .size(14.0)
                    .color(COL_HINT),
            );
            render_mod_checkboxes(ui, &mut mods);
            ui.add_space(4.0);
            let preview = Keycode::one_shot_mod(mods);
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(format!(
                        "Result: {} (0x{:04X})",
                        preview.name(),
                        preview.raw()
                    ))
                    .monospace()
                    .size(15.0)
                    .color(COL_RESULT),
                );
                if ui.button("Apply").clicked() {
                    result = Some(preview.raw());
                }
            });
        }
        _ => {}
    }

    // Persist state
    ui.memory_mut(|mem| {
        mem.data.insert_temp(builder_layer_id, layer);
        mem.data.insert_temp(builder_mods_id, mods);
        mem.data.insert_temp(builder_key_id, base_key);
    });

    result
}

/// Render modifier checkboxes for builder UIs.
fn render_mod_checkboxes(ui: &mut egui::Ui, mods: &mut u8) {
    ui.horizontal(|ui| {
        let labels = [
            (0x01u8, "Ctrl"),
            (0x02, "Shift"),
            (0x04, "Alt"),
            (0x08, "GUI"),
        ];
        for (bit, label) in labels {
            let mut checked = *mods & bit != 0;
            if ui.checkbox(&mut checked, label).changed() {
                if checked {
                    *mods |= bit;
                } else {
                    *mods &= !bit;
                }
            }
        }
        // Right-side toggle
        let mut right = *mods & 0x10 != 0;
        if ui.checkbox(&mut right, "Right").changed() {
            if right {
                *mods |= 0x10;
            } else {
                *mods &= !0x10;
            }
        }
    });
}
