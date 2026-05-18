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
                "Build Mod-Tap, One-Shot, and Layer-Tap keys. \
                 Select multiple base keys to generate a batch, then add them to your favorites for quick assignment in the Keymap tab.",
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
            .show(ui, |ui| {
                match active_cat {
                    "Mod-Tap" => self.render_quantum_mod_tap_batch(ui, &filtered),
                    "One-Shot Mod" => self.render_quantum_osm_batch(ui, &filtered),
                    "Layer-Tap" | "Layer" => self.render_quantum_layer(ui, &filtered),
                    _ => {
                        for qt in &filtered {
                            ui.label(format!("{} — {}", qt.name, qt.description));
                        }
                    }
                }

                // Always show favorites at the bottom
                ui.add_space(16.0);
                ui.separator();
                ui.add_space(8.0);
                self.render_quantum_favorites(ui);
            });
    }

    fn render_quantum_mod_tap_batch(
        &mut self,
        ui: &mut egui::Ui,
        types: &[&via_protocol::QuantumKeyType],
    ) {
        ui.label(
            egui::RichText::new("1. Pick a modifier type:")
                .strong()
                .size(15.0),
        );
        ui.add_space(4.0);

        let type_id = egui::Id::new("qt_mt_type");
        let mut sel_type: usize = ui.memory(|mem| mem.data.get_temp(type_id)).unwrap_or(1);

        ui.horizontal_wrapped(|ui| {
            for (i, qt) in types.iter().enumerate() {
                if qt.mod_mask.is_none() {
                    continue;
                }
                let sel = sel_type == i;
                let text = egui::RichText::new(qt.name).size(13.5);
                if ui.selectable_label(sel, text).clicked() {
                    sel_type = i;
                }
            }
        });
        ui.memory_mut(|mem| mem.data.insert_temp(type_id, sel_type));

        if let Some(qt) = types.get(sel_type) {
            ui.label(
                egui::RichText::new(qt.description)
                    .size(13.0)
                    .color(egui::Color32::from_rgb(140, 140, 155)),
            );
        }

        ui.add_space(8.0);
        ui.label(
            egui::RichText::new("2. Select base keys (click to toggle, select multiple):")
                .strong()
                .size(15.0),
        );
        ui.add_space(4.0);

        // Multi-select keys using a bitset stored in memory
        let keys_id = egui::Id::new("qt_mt_selected_keys");
        let mut selected_keys: Vec<u16> = ui
            .memory(|mem| mem.data.get_temp::<Vec<u16>>(keys_id))
            .unwrap_or_default();

        // Key grid with toggle behavior
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(3.0, 3.0);
            let all_keys: Vec<u16> = (0x04u16..=0x1Du16) // A-Z
                .chain(0x1Eu16..=0x27u16) // 0-9
                .chain(
                    [
                        0x2Du16, 0x2E, 0x2F, 0x30, 0x31, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38,
                    ]
                    .into_iter(),
                ) // symbols
                .chain([0x28u16, 0x2C, 0x29, 0x2A, 0x2B].into_iter()) // Enter, Space, Esc, Bksp, Tab
                .collect();

            for k in &all_keys {
                let name = Keycode(*k).name();
                let is_selected = selected_keys.contains(k);
                let size = egui::vec2(32.0, 24.0);
                let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

                let bg = if is_selected {
                    egui::Color32::from_rgb(60, 120, 80)
                } else if response.hovered() {
                    egui::Color32::from_rgb(70, 70, 80)
                } else {
                    egui::Color32::from_rgb(45, 45, 55)
                };
                let border = if is_selected {
                    egui::Color32::from_rgb(100, 200, 130)
                } else {
                    egui::Color32::from_rgb(60, 60, 65)
                };

                ui.painter()
                    .rect_filled(rect, egui::CornerRadius::same(3), bg);
                ui.painter().rect_stroke(
                    rect,
                    egui::CornerRadius::same(3),
                    egui::Stroke::new(1.0_f32, border),
                    egui::StrokeKind::Outside,
                );
                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    &name,
                    egui::FontId::proportional(11.0),
                    egui::Color32::from_rgb(220, 220, 230),
                );

                if response.clicked() {
                    if is_selected {
                        selected_keys.retain(|x| x != k);
                    } else {
                        selected_keys.push(*k);
                    }
                }
            }
        });

        ui.memory_mut(|mem| mem.data.insert_temp(keys_id, selected_keys.clone()));

        ui.add_space(8.0);

        // Show generated keycodes
        if !selected_keys.is_empty() {
            if let Some(qt) = types.get(sel_type)
                && let Some(mod_mask) = qt.mod_mask
            {
                ui.label(
                    egui::RichText::new(format!("3. Generated {} keycodes:", selected_keys.len()))
                        .strong()
                        .size(15.0),
                );
                ui.add_space(4.0);

                let generated: Vec<u16> = selected_keys
                    .iter()
                    .map(|&k| Keycode::mod_tap(mod_mask, k as u8).raw())
                    .collect();

                // Show as keycap previews
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(6.0, 6.0);
                    for &raw in &generated {
                        render_quantum_keycap_preview(ui, Keycode(raw));
                    }
                });

                ui.add_space(8.0);

                // Add all to favorites button
                let new_count = generated
                    .iter()
                    .filter(|kc| !self.quantum_favorites.contains(kc))
                    .count();

                ui.horizontal(|ui| {
                    if ui
                        .button(
                            egui::RichText::new(format!(
                                "Add all {} to My Quantum Keys",
                                new_count
                            ))
                            .size(14.0),
                        )
                        .clicked()
                        && new_count > 0
                    {
                        for kc in &generated {
                            if !self.quantum_favorites.contains(kc) {
                                self.quantum_favorites.push(*kc);
                            }
                        }
                    }

                    if ui
                        .button(egui::RichText::new("Clear selection").size(13.0))
                        .clicked()
                    {
                        ui.memory_mut(|mem| {
                            mem.data.insert_temp::<Vec<u16>>(keys_id, Vec::new());
                        });
                    }
                });
            }
        } else {
            ui.label(
                egui::RichText::new("Select one or more base keys above to generate keycodes.")
                    .size(13.0)
                    .color(egui::Color32::from_rgb(120, 120, 135)),
            );
        }
    }

    fn render_quantum_osm_batch(
        &mut self,
        ui: &mut egui::Ui,
        types: &[&via_protocol::QuantumKeyType],
    ) {
        ui.label(
            egui::RichText::new(
                "One-Shot Modifiers: applies modifier to the next keypress only. Click to add to favorites.",
            )
            .size(14.0)
            .color(egui::Color32::from_rgb(140, 140, 155)),
        );
        ui.add_space(8.0);

        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(8.0, 8.0);
            for qt in types {
                let Some(mod_mask) = qt.mod_mask else {
                    continue;
                };
                let kc = Keycode::one_shot_mod(mod_mask);
                let raw = kc.raw();
                let in_favs = self.quantum_favorites.contains(&raw);

                let btn_text = if in_favs {
                    egui::RichText::new(format!("{} (added)", qt.name))
                        .size(13.5)
                        .color(egui::Color32::from_rgb(100, 180, 100))
                } else {
                    egui::RichText::new(qt.name).size(13.5)
                };

                if ui.button(btn_text).on_hover_text(qt.description).clicked() && !in_favs {
                    self.quantum_favorites.push(raw);
                }
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
                        "LT" => Keycode::layer_tap(0, 0x2C).name(),
                        "MO" => Keycode::layer_momentary(0).name(),
                        "TG" => Keycode::layer_toggle(0).name(),
                        "TO" => Keycode::layer_on(0).name(),
                        "TT" => Keycode::layer_tap_toggle(0).name(),
                        "DF" => Keycode::layer_default(0).name(),
                        "OSL" => Keycode::one_shot_layer(0).name(),
                        "LM" => Keycode::layer_mod(0, 0x02).name(),
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

    fn render_quantum_favorites(&mut self, ui: &mut egui::Ui) {
        ui.label(
            egui::RichText::new(format!(
                "My Quantum Keys ({}) — available in keymap picker",
                self.quantum_favorites.len()
            ))
            .strong()
            .size(16.0),
        );
        ui.add_space(4.0);

        if self.quantum_favorites.is_empty() {
            ui.label(
                egui::RichText::new(
                    "No quantum keys added yet. Use the builder above to generate and add them.",
                )
                .size(13.0)
                .color(egui::Color32::from_rgb(120, 120, 135)),
            );
            return;
        }

        let mut to_remove: Option<usize> = None;

        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(6.0, 6.0);
            for (i, &raw) in self.quantum_favorites.iter().enumerate() {
                let kc = Keycode(raw);
                let size = egui::vec2(72.0, 52.0);
                let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

                let bg = if response.hovered() {
                    egui::Color32::from_rgb(70, 45, 45)
                } else {
                    egui::Color32::from_rgb(55, 55, 65)
                };
                let border = egui::Color32::from_rgb(100, 140, 200);

                ui.painter()
                    .rect_filled(rect, egui::CornerRadius::same(6), bg);
                ui.painter().rect_stroke(
                    rect,
                    egui::CornerRadius::same(6),
                    egui::Stroke::new(1.5_f32, border),
                    egui::StrokeKind::Outside,
                );

                if let Some((tap, hold)) = kc.dual_labels() {
                    let top = egui::pos2(rect.center().x, rect.min.y + rect.height() * 0.3);
                    ui.painter().text(
                        top,
                        egui::Align2::CENTER_CENTER,
                        &tap,
                        egui::FontId::proportional(13.0),
                        egui::Color32::WHITE,
                    );
                    let bot = egui::pos2(rect.center().x, rect.min.y + rect.height() * 0.6);
                    ui.painter().text(
                        bot,
                        egui::Align2::CENTER_CENTER,
                        &hold,
                        egui::FontId::proportional(10.0),
                        egui::Color32::from_rgb(150, 180, 220),
                    );
                } else {
                    ui.painter().text(
                        egui::pos2(rect.center().x, rect.min.y + rect.height() * 0.4),
                        egui::Align2::CENTER_CENTER,
                        &kc.name(),
                        egui::FontId::proportional(11.0),
                        egui::Color32::WHITE,
                    );
                }

                // Small X indicator
                let x_pos = egui::pos2(rect.center().x, rect.min.y + rect.height() * 0.85);
                ui.painter().text(
                    x_pos,
                    egui::Align2::CENTER_CENTER,
                    if response.hovered() {
                        "click to remove"
                    } else {
                        ""
                    },
                    egui::FontId::proportional(8.0),
                    egui::Color32::from_rgb(200, 100, 100),
                );

                if response.clicked() {
                    to_remove = Some(i);
                }
                response.on_hover_text(format!("{} (0x{:04X}) — click to remove", kc.name(), raw));
            }
        });

        if let Some(idx) = to_remove {
            self.quantum_favorites.remove(idx);
        }

        ui.add_space(8.0);
        if ui
            .button(egui::RichText::new("Clear all").size(13.0))
            .clicked()
        {
            self.quantum_favorites.clear();
        }
    }
}

/// Render a preview of what a quantum keycap looks like.
fn render_quantum_keycap_preview(ui: &mut egui::Ui, kc: Keycode) {
    let size = egui::vec2(56.0, 40.0);
    let (rect, _response) = ui.allocate_exact_size(size, egui::Sense::hover());

    let rounding = egui::CornerRadius::same(5);
    let bg = egui::Color32::from_rgb(55, 55, 65);
    let border = egui::Color32::from_rgb(80, 120, 170);

    ui.painter().rect_filled(rect, rounding, bg);
    ui.painter().rect_stroke(
        rect,
        rounding,
        egui::Stroke::new(1.5_f32, border),
        egui::StrokeKind::Outside,
    );

    if let Some((tap, hold)) = kc.dual_labels() {
        let top_center = egui::pos2(rect.center().x, rect.min.y + rect.height() * 0.35);
        ui.painter().text(
            top_center,
            egui::Align2::CENTER_CENTER,
            &tap,
            egui::FontId::proportional(13.0),
            egui::Color32::WHITE,
        );
        let bot_center = egui::pos2(rect.center().x, rect.min.y + rect.height() * 0.72);
        ui.painter().text(
            bot_center,
            egui::Align2::CENTER_CENTER,
            &hold,
            egui::FontId::proportional(9.5),
            egui::Color32::from_rgb(150, 180, 220),
        );
    } else {
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            &kc.name(),
            egui::FontId::proportional(11.0),
            egui::Color32::WHITE,
        );
    }
}
