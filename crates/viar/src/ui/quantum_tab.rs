use eframe::egui;
use via_protocol::{
    Keycode,
    quantum_key_types,
};

use crate::types::ViarApp;

impl ViarApp {
    pub fn render_quantum_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Quantum Keys");
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new(
                "Configure Mod-Tap, One-Shot, and Layer-Tap keys. \
                 Select a type and base key to generate the keycode, then assign it in the Keymap tab.",
            )
            .size(14.0)
            .color(egui::Color32::from_rgb(160, 160, 175)),
        );
        ui.add_space(8.0);

        // Category filter
        let categories: Vec<&str> = {
            let types = quantum_key_types();
            let mut cats: Vec<&'static str> = types.iter().map(|t| t.category).collect();
            cats.dedup();
            cats
        };

        let cat_id = egui::Id::new("quantum_tab_category");
        let mut selected_cat: usize = ui.memory(|mem| mem.data.get_temp(cat_id)).unwrap_or(0);

        ui.horizontal(|ui| {
            for (i, cat) in categories.iter().enumerate() {
                let sel = selected_cat == i;
                let text = egui::RichText::new(*cat).size(15.0);
                if ui.selectable_label(sel, text).clicked() {
                    selected_cat = i;
                }
            }
        });
        ui.memory_mut(|mem| mem.data.insert_temp(cat_id, selected_cat));

        ui.add_space(4.0);
        ui.separator();
        ui.add_space(4.0);

        let active_cat = categories.get(selected_cat).copied().unwrap_or("Mod-Tap");
        let types = quantum_key_types();
        let filtered: Vec<_> = types.iter().filter(|t| t.category == active_cat).collect();

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| match active_cat {
                "Mod-Tap" => self.render_quantum_mod_tap(ui, &filtered),
                "One-Shot Mod" => self.render_quantum_osm(ui, &filtered),
                "Layer-Tap" | "Layer" => self.render_quantum_layer(ui, &filtered),
                _ => {
                    for qt in &filtered {
                        ui.label(format!("{} — {}", qt.name, qt.description));
                    }
                }
            });
    }

    fn render_quantum_mod_tap(&self, ui: &mut egui::Ui, types: &[&via_protocol::QuantumKeyType]) {
        ui.label(
            egui::RichText::new("Mod-Tap: tap sends a key, hold activates a modifier.")
                .size(14.0)
                .color(egui::Color32::from_rgb(140, 140, 155)),
        );
        ui.add_space(8.0);

        // Show a grid of all mod-tap types with example keycodes
        let col_width = 160.0;
        egui::Grid::new("quantum_modtap_grid")
            .min_col_width(col_width)
            .spacing([12.0, 6.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Type").strong().size(14.0));
                ui.label(egui::RichText::new("Hold").strong().size(14.0));
                ui.label(egui::RichText::new("Example").strong().size(14.0));
                ui.label(egui::RichText::new("Raw").strong().size(14.0));
                ui.end_row();

                for qt in types {
                    let Some(mod_mask) = qt.mod_mask else {
                        continue;
                    };
                    // Example with 'D' key (0x07)
                    let example_kc = Keycode::mod_tap(mod_mask, 0x07);
                    let example_name = example_kc.name();

                    ui.label(
                        egui::RichText::new(qt.name)
                            .monospace()
                            .size(14.0)
                            .color(egui::Color32::from_rgb(180, 220, 255)),
                    );
                    ui.label(
                        egui::RichText::new(qt.description.split(": ").last().unwrap_or(""))
                            .size(13.0),
                    );
                    ui.label(
                        egui::RichText::new(&example_name)
                            .monospace()
                            .size(13.0)
                            .color(egui::Color32::from_rgb(140, 200, 140)),
                    );
                    ui.label(
                        egui::RichText::new(format!("0x{:04X}", example_kc.raw()))
                            .monospace()
                            .size(12.0)
                            .color(egui::Color32::from_rgb(120, 120, 135)),
                    );
                    ui.end_row();
                }
            });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        // Interactive builder
        ui.label(egui::RichText::new("Quick Builder").strong().size(16.0));
        ui.add_space(4.0);

        let type_id = egui::Id::new("qt_mt_type");
        let key_id = egui::Id::new("qt_mt_key");

        let mut sel_type: usize = ui.memory(|mem| mem.data.get_temp(type_id)).unwrap_or(1); // default LSFT_T
        let mut sel_key: u16 = ui.memory(|mem| mem.data.get_temp(key_id)).unwrap_or(0x07); // D

        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Modifier:").size(14.0));
            for (i, qt) in types.iter().enumerate() {
                if qt.mod_mask.is_none() {
                    continue;
                }
                let sel = sel_type == i;
                if ui
                    .selectable_label(sel, egui::RichText::new(qt.name).size(13.0))
                    .clicked()
                {
                    sel_type = i;
                }
            }
        });

        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Tap Key:").size(14.0));
            let name = if sel_key == 0 {
                "---".to_string()
            } else {
                Keycode(sel_key).name()
            };
            ui.label(egui::RichText::new(&name).monospace().size(14.0));
        });

        // Key buttons
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(2.0, 2.0);
            for k in 0x04u16..=0x1Du16 {
                if ui.small_button(&Keycode(k).name()).clicked() {
                    sel_key = k;
                }
            }
            for k in 0x1Eu16..=0x27u16 {
                if ui.small_button(&Keycode(k).name()).clicked() {
                    sel_key = k;
                }
            }
            for &k in &[
                0x2Du16, 0x2E, 0x2F, 0x30, 0x31, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38,
            ] {
                if ui.small_button(&Keycode(k).name()).clicked() {
                    sel_key = k;
                }
            }
            for &k in &[0x28u16, 0x2C, 0x29, 0x2A, 0x2B] {
                if ui.small_button(&Keycode(k).name()).clicked() {
                    sel_key = k;
                }
            }
        });

        ui.add_space(8.0);

        // Preview result
        if let Some(qt) = types.get(sel_type)
            && let Some(mod_mask) = qt.mod_mask
        {
            let result = Keycode::mod_tap(mod_mask, sel_key as u8);
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(format!(
                        "Result: {} (0x{:04X})",
                        result.name(),
                        result.raw()
                    ))
                    .monospace()
                    .size(15.0)
                    .color(egui::Color32::from_rgb(140, 200, 140)),
                );
            });

            // Keycap preview
            ui.add_space(8.0);
            render_quantum_keycap_preview(ui, result);
        }

        ui.memory_mut(|mem| {
            mem.data.insert_temp(type_id, sel_type);
            mem.data.insert_temp(key_id, sel_key);
        });
    }

    fn render_quantum_osm(&self, ui: &mut egui::Ui, types: &[&via_protocol::QuantumKeyType]) {
        ui.label(
            egui::RichText::new("One-Shot Modifiers: applies modifier to the next keypress only.")
                .size(14.0)
                .color(egui::Color32::from_rgb(140, 140, 155)),
        );
        ui.add_space(8.0);

        egui::Grid::new("quantum_osm_grid")
            .min_col_width(120.0)
            .spacing([12.0, 8.0])
            .show(ui, |ui| {
                for qt in types {
                    let Some(mod_mask) = qt.mod_mask else {
                        continue;
                    };
                    let kc = Keycode::one_shot_mod(mod_mask);

                    ui.label(
                        egui::RichText::new(qt.name)
                            .monospace()
                            .size(14.0)
                            .color(egui::Color32::from_rgb(220, 180, 255)),
                    );
                    ui.label(egui::RichText::new(qt.description).size(13.0));
                    ui.label(
                        egui::RichText::new(format!("0x{:04X}", kc.raw()))
                            .monospace()
                            .size(12.0)
                            .color(egui::Color32::from_rgb(120, 120, 135)),
                    );
                    ui.end_row();
                }
            });
    }

    fn render_quantum_layer(&self, ui: &mut egui::Ui, types: &[&via_protocol::QuantumKeyType]) {
        ui.label(
            egui::RichText::new("Layer functions for switching and activating layers.")
                .size(14.0)
                .color(egui::Color32::from_rgb(140, 140, 155)),
        );
        ui.add_space(8.0);

        egui::Grid::new("quantum_layer_grid")
            .min_col_width(80.0)
            .spacing([12.0, 8.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Function").strong().size(14.0));
                ui.label(egui::RichText::new("Description").strong().size(14.0));
                ui.label(egui::RichText::new("Example (L0)").strong().size(14.0));
                ui.end_row();

                for qt in types {
                    let example = match qt.name {
                        "LT" => Keycode::layer_tap(0, 0x2C).name(), // LT(0, Space)
                        "MO" => Keycode::layer_momentary(0).name(),
                        "TG" => Keycode::layer_toggle(0).name(),
                        "TO" => Keycode::layer_on(0).name(),
                        "TT" => Keycode::layer_tap_toggle(0).name(),
                        "DF" => Keycode::layer_default(0).name(),
                        "OSL" => Keycode::one_shot_layer(0).name(),
                        "LM" => Keycode::layer_mod(0, 0x02).name(), // LM(0, Shift)
                        _ => "—".to_string(),
                    };

                    ui.label(
                        egui::RichText::new(qt.name)
                            .monospace()
                            .size(14.0)
                            .color(egui::Color32::from_rgb(180, 255, 200)),
                    );
                    ui.label(egui::RichText::new(qt.description).size(13.0));
                    ui.label(
                        egui::RichText::new(&example)
                            .monospace()
                            .size(13.0)
                            .color(egui::Color32::from_rgb(140, 200, 140)),
                    );
                    ui.end_row();
                }
            });
    }
}

/// Render a preview of what a quantum keycap looks like in the keymap view.
fn render_quantum_keycap_preview(ui: &mut egui::Ui, kc: Keycode) {
    let size = egui::vec2(64.0, 48.0);
    let (rect, _response) = ui.allocate_exact_size(size, egui::Sense::hover());

    let rounding = egui::CornerRadius::same(6);
    let bg = egui::Color32::from_rgb(55, 55, 65);
    let border = egui::Color32::from_rgb(100, 140, 200);

    ui.painter().rect_filled(rect, rounding, bg);
    ui.painter().rect_stroke(
        rect,
        rounding,
        egui::Stroke::new(2.0_f32, border),
        egui::StrokeKind::Outside,
    );

    if let Some((tap, hold)) = kc.dual_labels() {
        // Top half: tap label
        let top_center = egui::pos2(rect.center().x, rect.min.y + rect.height() * 0.35);
        ui.painter().text(
            top_center,
            egui::Align2::CENTER_CENTER,
            &tap,
            egui::FontId::proportional(14.0),
            egui::Color32::WHITE,
        );
        // Bottom half: hold label
        let bot_center = egui::pos2(rect.center().x, rect.min.y + rect.height() * 0.7);
        ui.painter().text(
            bot_center,
            egui::Align2::CENTER_CENTER,
            &hold,
            egui::FontId::proportional(11.0),
            egui::Color32::from_rgb(150, 180, 220),
        );
    } else {
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            &kc.name(),
            egui::FontId::proportional(12.0),
            egui::Color32::WHITE,
        );
    }
}
