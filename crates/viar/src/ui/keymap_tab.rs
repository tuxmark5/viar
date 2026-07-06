use std::collections::HashMap;

use eframe::egui;
use tracing::{
    info,
    warn,
};
use via_protocol::{
    Keycode,
    KeycodeGroup,
    ViaProtocol,
};

use crate::{
    theme::Theme,
    types::{
        DynamicEntryData,
        KeyChange,
        KeymapData,
        StatusMessage,
        ViarApp,
    },
    ui::keycode_builder::render_keycode_builder,
    util::{
        CategoryStyle,
        aliased_name,
        is_disconnect_error,
        themed_tab,
    },
};

// ===========================================================================
// Colors
// ===========================================================================
// Shared keycap / picker palette. `from_rgb` is const, so these are constants.

/// Background of a selected keycap or the current keycode in the picker.
const COL_SELECTED_BG: egui::Color32 = egui::Color32::from_rgb(70, 130, 180);
/// Background of a hovered keycap.
const COL_HOVER_BG: egui::Color32 = egui::Color32::from_rgb(80, 80, 90);
/// Border of a selected / current keycap.
const COL_SELECTED_BORDER: egui::Color32 = egui::Color32::from_rgb(100, 180, 255);
/// Default keycap border.
const COL_BORDER: egui::Color32 = egui::Color32::from_rgb(60, 60, 65);
/// Primary keycap label text.
const COL_TEXT: egui::Color32 = egui::Color32::from_rgb(220, 220, 230);
/// Dimmed text for empty (KC_NO / KC_TRNS) keys.
const COL_TEXT_DIM: egui::Color32 = egui::Color32::from_rgb(100, 100, 110);
/// Hold/sub label on a split keycap (default state).
const COL_HOLD_LABEL: egui::Color32 = egui::Color32::from_rgb(140, 140, 160);
/// Hold/sub label on a split keycap (selected state).
const COL_HOLD_LABEL_SEL: egui::Color32 = egui::Color32::from_rgb(180, 200, 230);

/// Keycode name in the picker header.
const COL_KC_NAME: egui::Color32 = egui::Color32::from_rgb(160, 160, 175);
/// Keycode category label in the picker header.
const COL_CATEGORY: egui::Color32 = egui::Color32::from_rgb(120, 120, 135);
/// "Hex:" field label in the picker.
const COL_HEX_LABEL: egui::Color32 = egui::Color32::from_rgb(130, 130, 145);
/// Live "→ name" preview next to the hex field.
const COL_PREVIEW: egui::Color32 = egui::Color32::from_rgb(140, 170, 200);

/// Combo description text in the key tooltip.
const COL_COMBO_DESC: egui::Color32 = egui::Color32::from_rgb(180, 200, 220);
/// Tap-dance summary text in the key tooltip.
const COL_TAPDANCE_INFO: egui::Color32 = egui::Color32::from_rgb(200, 180, 140);

/// Default background of a "My Quantum" favorite cell.
const COL_FAV_BG: egui::Color32 = egui::Color32::from_rgb(50, 55, 65);
/// Default border of a "My Quantum" favorite cell.
const COL_FAV_BORDER: egui::Color32 = egui::Color32::from_rgb(70, 100, 140);
/// Hold/sub label on a split favorite cell.
const COL_FAV_HOLD_LABEL: egui::Color32 = egui::Color32::from_rgb(150, 180, 220);

impl ViarApp {
    pub fn render_keymap_tab(&mut self, ui: &mut egui::Ui) {
        // Clone aliases up front so the picker can borrow them without holding
        // a borrow on self.dynamic_data.
        let aliases = self.dynamic_data.as_ref().map(|d| d.aliases.clone());
        let aliases_ref = aliases.as_ref();

        if self.keymap_data.is_none() {
            ui.label("No keymap data loaded.");
            return;
        }

        // Overlays derived from dynamic data — owned, so they don't tie up the
        // keymap_data borrow below.
        let combo_map = build_combo_map(self.dynamic_data.as_ref());
        let td_labels = build_td_labels(self.dynamic_data.as_ref());

        // --- Layer tabs, keyboard render, and selection handling ---
        // Scoped so the keymap_data borrow is released before the picker
        // (a &mut self method) runs.
        let (selected_key, selected_key_rect, layer_idx) = {
            let data = self.keymap_data.as_mut().unwrap();

            ui.horizontal(|ui| {
                ui.add_space(8.0);
                for layer in 0..data.layer_count as usize {
                    let label = format!("Layer {layer}");
                    let selected = data.selected_layer == layer;
                    if themed_tab(ui, selected, &label, &self.theme).clicked() {
                        data.selected_layer = layer;
                        data.selected_key = None;
                    }
                }
            });

            ui.separator();

            // Keyboard visualization geometry
            let available = ui.available_size();
            let layout = &data.layout;
            let layout_w = layout.width();
            let layout_h = layout.height();
            let padding = 40.0;
            let key_size = ((available.x - padding * 2.0) / layout_w)
                .min((available.y - padding * 2.0) / layout_h)
                .clamp(32.0, 64.0);
            let gap = 8.0;

            let total_w = layout_w * key_size;
            let total_h = layout_h * key_size;
            let x_offset = (available.x - total_w) / 2.0;
            let y_offset = (available.y - total_h) / 2.0;
            let origin = ui.min_rect().min + egui::vec2(x_offset, y_offset);
            let layer_idx = data.selected_layer;

            let render = {
                let ctx = KeymapRender {
                    data: &*data,
                    layer_idx,
                    origin,
                    key_size,
                    gap,
                    combo_map: &combo_map,
                    td: &td_labels,
                    aliases: aliases_ref,
                };
                render_keys(ui, &ctx)
            };
            let clicked_key = render.clicked_key;
            let selected_key_rect = render.selected_key_rect;

            // Toggle selection on click
            if let Some(idx) = clicked_key {
                if data.selected_key == Some(idx) {
                    data.selected_key = None;
                } else {
                    data.selected_key = Some(idx);
                }
            }

            // Close picker on Escape
            if data.selected_key.is_some() && ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                data.selected_key = None;
            }

            // Close picker on click outside keys and picker
            if data.selected_key.is_some() && clicked_key.is_none() {
                let click_pos = ui.input(|i| {
                    if i.pointer.primary_clicked() {
                        i.pointer.interact_pos()
                    } else {
                        None
                    }
                });
                if let Some(pos) = click_pos {
                    let picker_id = egui::Id::new("kc_picker");
                    let in_picker = ui
                        .ctx()
                        .memory(|mem| mem.area_rect(picker_id))
                        .is_some_and(|r| r.contains(pos));
                    if !in_picker {
                        data.selected_key = None;
                    }
                }
            }

            (data.selected_key, selected_key_rect, layer_idx)
        };

        // Floating popover picker (runs with keymap_data borrow released)
        if let (Some(key_idx), Some(key_rect)) = (selected_key, selected_key_rect) {
            self.render_keycode_picker(ui, key_idx, key_rect, layer_idx, aliases_ref);
        }

        // Handle deferred keycode application
        let pending: Option<(usize, u16)> = ui.memory(|mem| {
            let kc: Option<u16> = mem.data.get_temp(egui::Id::new("pending_keycode"));
            let idx: Option<usize> = mem.data.get_temp(egui::Id::new("pending_key_idx"));
            match (kc, idx) {
                (Some(kc), Some(idx)) => Some((idx, kc)),
                _ => None,
            }
        });

        if let Some((key_idx, new_kc)) = pending {
            ui.memory_mut(|mem| {
                mem.data.remove::<u16>(egui::Id::new("pending_keycode"));
                mem.data.remove::<usize>(egui::Id::new("pending_key_idx"));
            });
            self.apply_keycode(key_idx, new_kc);
        }
    }

    /// Render the floating keycode picker popover for the selected key.
    fn render_keycode_picker(
        &mut self,
        ui: &mut egui::Ui,
        key_idx: usize,
        key_rect: egui::Rect,
        layer_idx: usize,
        aliases_ref: Option<&HashMap<String, String>>,
    ) {
        // Snapshot the key's current keycode / matrix position, then release the
        // keymap_data borrow so the picker body can freely touch other self fields.
        let (raw_kc, row, col, kc_name, kc_category) = {
            let Some(data) = self.keymap_data.as_ref() else {
                return;
            };
            let key_pos = &data.layout.keys[key_idx];
            let raw_kc = data
                .keymap
                .get(layer_idx)
                .and_then(|l| l.get(key_pos.row as usize))
                .and_then(|r| r.get(key_pos.col as usize))
                .copied()
                .unwrap_or(0);
            let keycode = Keycode(raw_kc);
            (
                raw_kc,
                key_pos.row,
                key_pos.col,
                aliased_name(raw_kc, aliases_ref),
                format!("{:?}", keycode.category()),
            )
        };

        let popover_w = 480.0_f32;
        let popover_h = 320.0_f32;
        let (pop_x, pop_y) =
            picker_popover_pos(key_rect, ui.ctx().content_rect(), popover_w, popover_h);

        let mut open = true;
        egui::Window::new("Keycode Picker")
            .id(egui::Id::new("kc_picker"))
            .open(&mut open)
            .fixed_pos(egui::pos2(pop_x, pop_y))
            .fixed_size(egui::vec2(popover_w, popover_h))
            .collapsible(false)
            .title_bar(false)
            .show(ui.ctx(), |ui| {
                render_picker_header(ui, layer_idx, row, col, &kc_name, raw_kc, &kc_category);
                ui.add_space(4.0);
                render_hex_input(ui, raw_kc, key_idx);
                ui.add_space(2.0);

                // Group tabs + My Quantum tab + Builders tab
                let has_quantum_favs = !self.quantum_favorites.is_empty();
                let (quantum_tab_idx, builder_tab_idx) = render_picker_tabs(
                    ui,
                    &self.picker_groups,
                    &mut self.picker_selected_group,
                    has_quantum_favs,
                    &self.theme,
                );

                ui.add_space(2.0);
                ui.separator();
                ui.add_space(2.0);

                // Keycode grid or builder UI
                let group_idx = self.picker_selected_group;
                let picked_kc: Option<u16> = if group_idx == builder_tab_idx {
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .show(ui, |ui| render_keycode_builder(ui, raw_kc))
                        .inner
                } else if has_quantum_favs && group_idx == quantum_tab_idx {
                    render_quantum_favorites(ui, &self.quantum_favorites, raw_kc)
                } else {
                    render_keycode_grid(ui, self.picker_groups.get(group_idx), raw_kc, aliases_ref)
                };

                if let Some(new_kc) = picked_kc {
                    ui.memory_mut(|mem| {
                        mem.data
                            .insert_temp(egui::Id::new("pending_keycode"), new_kc);
                        mem.data
                            .insert_temp(egui::Id::new("pending_key_idx"), key_idx);
                    });
                }
            });

        // Close popover => deselect key
        if !open && let Some(data) = &mut self.keymap_data {
            data.selected_key = None;
        }
    }

    pub fn apply_keycode(&mut self, key_idx: usize, new_keycode: u16) {
        let Some(data) = &mut self.keymap_data else {
            return;
        };
        let key_pos = &data.layout.keys[key_idx];
        let layer = data.selected_layer;
        let row = key_pos.row;
        let col = key_pos.col;

        let old_keycode = data
            .keymap
            .get(layer)
            .and_then(|l| l.get(row as usize))
            .and_then(|r| r.get(col as usize))
            .copied()
            .unwrap_or(0);

        if old_keycode == new_keycode {
            return;
        }

        if let Some(layer_data) = data.keymap.get_mut(layer)
            && let Some(row_data) = layer_data.get_mut(row as usize)
            && let Some(cell) = row_data.get_mut(col as usize)
        {
            *cell = new_keycode;
        }

        data.undo_stack.push(KeyChange {
            layer,
            row,
            col,
            key_idx,
            old_keycode,
            new_keycode,
        });
        data.dirty = true;

        if let Some(dev) = &self.connected_device {
            let proto = ViaProtocol::new(dev);
            match proto.set_keycode(layer as u8, row, col, new_keycode) {
                Ok(()) => {
                    let kc_name = Keycode(new_keycode).name();
                    info!(
                        layer,
                        row,
                        col,
                        keycode = kc_name,
                        "keycode written to device"
                    );
                    self.set_status(StatusMessage::info(format!(
                        "Set [{row},{col}] -> {kc_name}"
                    )));
                }
                Err(e) => {
                    let err_str = format!("{e}");
                    warn!(error = %e, "failed to write keycode to device");
                    self.set_status(StatusMessage::error(format!("Write failed: {e}")));
                    if is_disconnect_error(&err_str) {
                        self.handle_disconnect();
                    }
                }
            }
        }
    }

    pub fn undo(&mut self) {
        let Some(data) = &mut self.keymap_data else {
            return;
        };
        let Some(change) = data.undo_stack.pop() else {
            return;
        };

        if let Some(layer_data) = data.keymap.get_mut(change.layer)
            && let Some(row_data) = layer_data.get_mut(change.row as usize)
            && let Some(cell) = row_data.get_mut(change.col as usize)
        {
            *cell = change.old_keycode;
        }

        if data.undo_stack.is_empty() {
            data.dirty = false;
        }

        if let Some(dev) = &self.connected_device {
            let proto = ViaProtocol::new(dev);
            match proto.set_keycode(
                change.layer as u8,
                change.row,
                change.col,
                change.old_keycode,
            ) {
                Ok(()) => {
                    let name = Keycode(change.old_keycode).name();
                    info!(
                        layer = change.layer,
                        row = change.row,
                        col = change.col,
                        keycode = name,
                        "undo applied"
                    );
                    self.set_status(StatusMessage::info(format!(
                        "Undo: [{},{}] -> {name}",
                        change.row, change.col
                    )));
                }
                Err(e) => {
                    let err_str = format!("{e}");
                    warn!(error = %e, "undo write failed");
                    self.set_status(StatusMessage::error(format!("Undo write failed: {e}")));
                    if is_disconnect_error(&err_str) {
                        self.handle_disconnect();
                    }
                }
            }
        }
    }
}

// ===========================================================================
// Keycode picker
// ===========================================================================

/// Compute the on-screen top-left corner for the picker popover, preferring
/// below the key and flipping above / clamping to stay within the window.
fn picker_popover_pos(
    key_rect: egui::Rect,
    screen_rect: egui::Rect,
    popover_w: f32,
    popover_h: f32,
) -> (f32, f32) {
    let mut pop_x = key_rect.center().x - popover_w / 2.0;
    let mut pop_y = key_rect.max.y + 8.0;

    if pop_y + popover_h > screen_rect.max.y - 10.0 {
        pop_y = key_rect.min.y - popover_h - 8.0;
    }
    pop_x = pop_x.clamp(screen_rect.min.x + 5.0, screen_rect.max.x - popover_w - 5.0);
    pop_y = pop_y.clamp(screen_rect.min.y + 5.0, screen_rect.max.y - popover_h - 5.0);
    (pop_x, pop_y)
}

/// Picker header: layer / matrix position, current keycode name + hex, category.
fn render_picker_header(
    ui: &mut egui::Ui,
    layer_idx: usize,
    row: u8,
    col: u8,
    kc_name: &str,
    raw_kc: u16,
    kc_category: &str,
) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(format!("Layer {layer_idx} [{row},{col}]"))
                .strong()
                .size(17.0),
        );
        ui.separator();
        ui.label(
            egui::RichText::new(format!("{kc_name}  {:#06x}", raw_kc))
                .size(16.0)
                .color(COL_KC_NAME),
        );
        ui.separator();
        ui.label(
            egui::RichText::new(kc_category)
                .size(15.0)
                .color(COL_CATEGORY),
        );
    });
}

/// Raw-hex keycode entry row. Queues a pending keycode on Enter or the Set button.
fn render_hex_input(ui: &mut egui::Ui, raw_kc: u16, key_idx: usize) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Hex:").size(15.0).color(COL_HEX_LABEL));
        let hex_id = egui::Id::new("picker_hex_input");
        let mut hex_str: String = ui
            .memory(|mem| mem.data.get_temp(hex_id))
            .unwrap_or_else(|| format!("{:04X}", raw_kc));
        let resp = ui.add(
            egui::TextEdit::singleline(&mut hex_str)
                .desired_width(60.0)
                .font(egui::TextStyle::Monospace),
        );
        ui.memory_mut(|mem| mem.data.insert_temp(hex_id, hex_str.clone()));

        if resp.lost_focus()
            && ui.input(|i| i.key_pressed(egui::Key::Enter))
            && let Ok(v) = u16::from_str_radix(hex_str.trim(), 16)
        {
            queue_pending_keycode(ui, v, key_idx);
        }

        let preview_kc = u16::from_str_radix(hex_str.trim(), 16).unwrap_or(0);
        if preview_kc != 0 {
            let preview = Keycode(preview_kc);
            ui.label(
                egui::RichText::new(format!("→ {}", preview.name()))
                    .size(15.0)
                    .color(COL_PREVIEW),
            );
        }

        if ui
            .add(
                egui::Button::new(egui::RichText::new("Set").size(15.0))
                    .corner_radius(egui::CornerRadius::same(3)),
            )
            .clicked()
            && let Ok(v) = u16::from_str_radix(hex_str.trim(), 16)
        {
            queue_pending_keycode(ui, v, key_idx);
        }
    });
}

/// Stash a keycode + key index in egui memory for the main loop to apply after
/// the frame (avoids mutating self from inside the picker closures).
fn queue_pending_keycode(ui: &egui::Ui, keycode: u16, key_idx: usize) {
    ui.memory_mut(|mem| {
        mem.data
            .insert_temp(egui::Id::new("pending_keycode"), keycode);
        mem.data
            .insert_temp(egui::Id::new("pending_key_idx"), key_idx);
    });
}

/// Picker group tab bar: keycode groups, then optional "My Quantum" and
/// "Builders" tabs. Mutates `selected` and returns the (quantum, builder) tab
/// indices so the caller can dispatch the body.
fn render_picker_tabs(
    ui: &mut egui::Ui,
    groups: &[KeycodeGroup],
    selected: &mut usize,
    has_quantum_favs: bool,
    theme: &Theme,
) -> (usize, usize) {
    let quantum_tab_idx = groups.len();
    let builder_tab_idx = if has_quantum_favs {
        groups.len() + 1
    } else {
        groups.len()
    };
    ui.horizontal_wrapped(|ui| {
        for (i, group) in groups.iter().enumerate() {
            let sel = *selected == i;
            let label = egui::RichText::new(group.name).size(15.5);
            if themed_tab(ui, sel, label, theme).clicked() {
                *selected = i;
            }
        }
        // My Quantum tab (only if favorites exist)
        if has_quantum_favs {
            let sel = *selected == quantum_tab_idx;
            let label = egui::RichText::new("My Quantum").size(15.5);
            if themed_tab(ui, sel, label, theme).clicked() {
                *selected = quantum_tab_idx;
            }
        }
        // Builders tab (for LT, MT, Mod+Key, OSM)
        let sel = *selected == builder_tab_idx;
        let label = egui::RichText::new("Builders").size(15.5);
        if themed_tab(ui, sel, label, theme).clicked() {
            *selected = builder_tab_idx;
        }
    });
    (quantum_tab_idx, builder_tab_idx)
}

/// "My Quantum" favorites grid. Returns a clicked keycode, if any.
fn render_quantum_favorites(ui: &mut egui::Ui, favorites: &[u16], raw_kc: u16) -> Option<u16> {
    let mut picked_kc = None;
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(4.0, 4.0);
                for &fav_raw in favorites {
                    let kc = Keycode(fav_raw);
                    let name = kc.name();
                    let is_current = fav_raw == raw_kc;
                    let size = egui::vec2(56.0, 36.0);
                    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
                    let is_hovered = response.hovered();

                    let bg = if is_current {
                        COL_SELECTED_BG
                    } else if is_hovered {
                        COL_HOVER_BG
                    } else {
                        COL_FAV_BG
                    };
                    let border = if is_current {
                        COL_SELECTED_BORDER
                    } else {
                        COL_FAV_BORDER
                    };

                    let rounding = egui::CornerRadius::same(4);
                    ui.painter().rect_filled(rect, rounding, bg);
                    ui.painter().rect_stroke(
                        rect,
                        rounding,
                        egui::Stroke::new(1.0_f32, border),
                        egui::StrokeKind::Outside,
                    );

                    // Dual-label rendering
                    if let Some((tap, hold)) = kc.dual_labels() {
                        let top = egui::pos2(rect.center().x, rect.min.y + rect.height() * 0.32);
                        ui.painter().text(
                            top,
                            egui::Align2::CENTER_CENTER,
                            &tap,
                            egui::FontId::proportional(12.0),
                            egui::Color32::WHITE,
                        );
                        let bot = egui::pos2(rect.center().x, rect.min.y + rect.height() * 0.72);
                        ui.painter().text(
                            bot,
                            egui::Align2::CENTER_CENTER,
                            &hold,
                            egui::FontId::proportional(9.0),
                            COL_FAV_HOLD_LABEL,
                        );
                    } else {
                        let font_size = if name.len() <= 5 { 11.0 } else { 9.0 };
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            &name,
                            egui::FontId::proportional(font_size),
                            COL_TEXT,
                        );
                    }

                    if response.clicked() {
                        picked_kc = Some(fav_raw);
                    }
                    response.on_hover_text(kc.description());
                }
            });
        });
    picked_kc
}

/// Standard keycode grid for a picker group. Returns a clicked keycode, if any.
fn render_keycode_grid(
    ui: &mut egui::Ui,
    group: Option<&KeycodeGroup>,
    raw_kc: u16,
    aliases_ref: Option<&HashMap<String, String>>,
) -> Option<u16> {
    let mut picked_kc = None;
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(4.0, 4.0);
                let Some(group) = group else {
                    return;
                };
                for kc in &group.codes {
                    let name = aliased_name(kc.0, aliases_ref);
                    let is_current = kc.0 == raw_kc;
                    let size = egui::vec2(44.0, 28.0);
                    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
                    let is_hovered = response.hovered();

                    let bg = if is_current {
                        COL_SELECTED_BG
                    } else if is_hovered {
                        COL_HOVER_BG
                    } else {
                        kc.category().bg()
                    };
                    let border = if is_current {
                        COL_SELECTED_BORDER
                    } else {
                        COL_BORDER
                    };
                    let text_col = if is_current {
                        egui::Color32::WHITE
                    } else if kc.0 == 0 || kc.0 == 1 {
                        COL_TEXT_DIM
                    } else {
                        COL_TEXT
                    };

                    let rounding = egui::CornerRadius::same(4);
                    ui.painter().rect_filled(rect, rounding, bg);
                    ui.painter().rect_stroke(
                        rect,
                        rounding,
                        egui::Stroke::new(1.0_f32, border),
                        egui::StrokeKind::Outside,
                    );

                    let font_size = if name.len() <= 2 {
                        12.0
                    } else if name.len() <= 5 {
                        10.5
                    } else {
                        8.5
                    };
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        &name,
                        egui::FontId::proportional(font_size),
                        text_col,
                    );

                    if response.clicked() {
                        picked_kc = Some(kc.0);
                    }
                    response.on_hover_text(kc.description());
                }
            });
        });
    picked_kc
}

// ===========================================================================
// Keyboard rendering
// ===========================================================================

/// A combo that a keycode participates in, with its assigned color.
struct ComboInfo {
    color:       egui::Color32,
    description: String,
}

/// Assign a distinct, muted color to a combo based on its index.
fn combo_color(index: usize, total: usize) -> egui::Color32 {
    let hue = if total <= 1 {
        0.35 // green
    } else {
        (index as f64) / (total as f64) // spread evenly across 0..1
    };
    // HSL to RGB with S=0.5, L=0.55 for muted but distinct pastels
    let s = 0.5_f64;
    let l = 0.55_f64;
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let h_prime = hue * 6.0;
    let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());
    let (r1, g1, b1) = match h_prime as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = l - c / 2.0;
    egui::Color32::from_rgb(
        ((r1 + m) * 255.0) as u8,
        ((g1 + m) * 255.0) as u8,
        ((b1 + m) * 255.0) as u8,
    )
}

/// Build a map from keycode to the combos it participates in (with colors),
/// for rendering combo indicator dots and tooltips.
fn build_combo_map(dynamic: Option<&DynamicEntryData>) -> HashMap<u16, Vec<ComboInfo>> {
    let Some(d) = dynamic else {
        return HashMap::new();
    };
    let active_combos: Vec<_> = d
        .combos
        .iter()
        .enumerate()
        .filter(|(_, c)| c.input.iter().any(|&k| k != 0))
        .collect();
    let total = active_combos.len();
    let mut map: HashMap<u16, Vec<ComboInfo>> = HashMap::new();
    for (color_idx, (combo_idx, combo)) in active_combos.iter().enumerate() {
        let active_inputs: Vec<u16> = combo.input.iter().copied().filter(|&k| k != 0).collect();
        let color = combo_color(color_idx, total);
        let input_names: Vec<String> = active_inputs.iter().map(|&kc| Keycode(kc).name()).collect();
        let output_name = Keycode(combo.output).name();
        let combo_display = d.combo_name(*combo_idx);
        let desc = format!(
            "{}: {} -> {}",
            combo_display,
            input_names.join(" + "),
            output_name
        );
        for &kc in &active_inputs {
            map.entry(kc).or_default().push(ComboInfo {
                color,
                description: desc.clone(),
            });
        }
    }
    map
}

/// Tap-dance overlays: per-keycode tooltip summaries and split keycap labels.
struct TdLabels {
    /// keycode -> full tooltip summary line.
    summaries: HashMap<u16, String>,
    /// keycode -> (top label, bottom label) for the keycap.
    keycaps:   HashMap<u16, (String, String)>,
}

/// Build tap-dance tooltip summaries and keycap labels from dynamic data.
fn build_td_labels(dynamic: Option<&DynamicEntryData>) -> TdLabels {
    let mut summaries: HashMap<u16, String> = HashMap::new();
    let mut keycaps: HashMap<u16, (String, String)> = HashMap::new();
    if let Some(d) = dynamic {
        for (i, td) in d.tap_dances.iter().enumerate() {
            if td.is_empty() {
                continue;
            }
            let kc_raw = 0x5700u16 | i as u16;
            let td_display = d.td_name(i);
            let mut parts = Vec::new();
            if td.on_tap != 0 {
                parts.push(format!("Tap: {}", Keycode(td.on_tap).name()));
            }
            if td.on_hold != 0 {
                parts.push(format!("Hold: {}", Keycode(td.on_hold).name()));
            }
            if td.on_double_tap != 0 {
                parts.push(format!("DTap: {}", Keycode(td.on_double_tap).name()));
            }
            if td.on_tap_hold != 0 {
                parts.push(format!("THold: {}", Keycode(td.on_tap_hold).name()));
            }
            if td.tapping_term > 0 {
                parts.push(format!("{}ms", td.tapping_term));
            }
            summaries.insert(kc_raw, format!("{}: {}", td_display, parts.join("  |  ")));

            // Keycap: show tap key on top, TD name on bottom
            let tap_label = if td.on_tap != 0 {
                Keycode(td.on_tap).name()
            } else {
                td_display.clone()
            };
            keycaps.insert(kc_raw, (tap_label, td_display));
        }
    }
    TdLabels { summaries, keycaps }
}

/// Immutable context shared while drawing the keyboard's keycaps: the loaded
/// keymap, the active layer, on-screen geometry, and the combo / tap-dance /
/// alias overlays. Passed by reference so `render_keys` and `render_key` (and
/// future encoder rendering) don't each carry a long argument list.
struct KeymapRender<'a> {
    data:      &'a KeymapData,
    layer_idx: usize,
    /// Top-left of the layout in screen coordinates.
    origin:    egui::Pos2,
    /// Pixel size of one key unit.
    key_size:  f32,
    /// Gap subtracted from each key's width/height.
    gap:       f32,
    combo_map: &'a HashMap<u16, Vec<ComboInfo>>,
    td:        &'a TdLabels,
    aliases:   Option<&'a HashMap<String, String>>,
}

/// Aggregate outcome of rendering all keycaps for a layer.
struct KeyRenderResult {
    /// Index of a key clicked this frame, if any.
    clicked_key:       Option<usize>,
    /// Screen rect of the currently selected key, for anchoring the picker.
    selected_key_rect: Option<egui::Rect>,
}

/// Interaction outcome for a single keycap.
struct KeyDrawResult {
    /// Whether this key was clicked this frame.
    clicked:       bool,
    /// This key's screen rect if it is the selected one (for picker anchoring).
    selected_rect: Option<egui::Rect>,
}

/// Draw every keycap for the selected layer, aggregating the click and the
/// selected key's rect.
fn render_keys(ui: &egui::Ui, ctx: &KeymapRender) -> KeyRenderResult {
    let mut clicked_key = None;
    let mut selected_key_rect = None;
    for key_idx in 0..ctx.data.layout.keys.len() {
        let res = render_key(ui, ctx, key_idx);
        if res.clicked {
            clicked_key = Some(key_idx);
        }
        if res.selected_rect.is_some() {
            selected_key_rect = res.selected_rect;
        }
    }
    KeyRenderResult {
        clicked_key,
        selected_key_rect,
    }
}

/// Draw one keycap: background, labels (split for tap-dance / dual-role keys),
/// combo indicator dots, and its hover tooltip.
fn render_key(ui: &egui::Ui, ctx: &KeymapRender, key_idx: usize) -> KeyDrawResult {
    let key_pos = &ctx.data.layout.keys[key_idx];
    let painter = ui.painter();

    let raw_kc = ctx
        .data
        .keymap
        .get(ctx.layer_idx)
        .and_then(|l| l.get(key_pos.row as usize))
        .and_then(|r| r.get(key_pos.col as usize))
        .copied()
        .unwrap_or(0);
    let keycode = Keycode(raw_kc);

    let px = ctx.origin.x + key_pos.x * ctx.key_size;
    let py = ctx.origin.y + key_pos.y * ctx.key_size;
    let pw = key_pos.w * ctx.key_size - ctx.gap;
    let ph = key_pos.h * ctx.key_size - ctx.gap;

    let rect = egui::Rect::from_min_size(egui::pos2(px, py), egui::vec2(pw, ph));

    let is_selected = ctx.data.selected_key == Some(key_idx);
    let is_hovered = ui.rect_contains_pointer(rect);

    let bg_color = if is_selected {
        COL_SELECTED_BG
    } else if is_hovered {
        COL_HOVER_BG
    } else {
        keycode.category().bg()
    };

    let border_color = if is_selected {
        COL_SELECTED_BORDER
    } else {
        COL_BORDER
    };

    let rounding = egui::CornerRadius::same(4);
    painter.rect_filled(rect, rounding, bg_color);
    painter.rect_stroke(
        rect,
        rounding,
        egui::Stroke::new(1.0_f32, border_color),
        egui::StrokeKind::Outside,
    );

    let label = aliased_name(raw_kc, ctx.aliases);

    let text_color = if is_selected {
        egui::Color32::WHITE
    } else if raw_kc == 0 || raw_kc == 1 {
        COL_TEXT_DIM
    } else {
        COL_TEXT
    };

    // Determine split labels: TD keycap labels take priority, then dual_labels
    let split_labels = ctx
        .td
        .keycaps
        .get(&raw_kc)
        .cloned()
        .or_else(|| keycode.dual_labels());

    if let Some((tap, hold)) = split_labels {
        // Split keycap: tap label on top, hold label on bottom
        let tap_font_size = if tap.len() <= 2 {
            ctx.key_size * 0.32
        } else if tap.len() <= 5 {
            ctx.key_size * 0.22
        } else {
            ctx.key_size * 0.16
        };
        let hold_font_size = ctx.key_size * 0.14;

        let tap_pos = egui::pos2(rect.center().x, rect.min.y + rect.height() * 0.35);
        let hold_pos = egui::pos2(rect.center().x, rect.min.y + rect.height() * 0.75);

        painter.text(
            tap_pos,
            egui::Align2::CENTER_CENTER,
            &tap,
            egui::FontId::proportional(tap_font_size),
            text_color,
        );

        let hold_color = if is_selected {
            COL_HOLD_LABEL_SEL
        } else {
            COL_HOLD_LABEL
        };
        painter.text(
            hold_pos,
            egui::Align2::CENTER_CENTER,
            &hold,
            egui::FontId::proportional(hold_font_size),
            hold_color,
        );
    } else {
        let font_size = if label.len() <= 2 {
            ctx.key_size * 0.35
        } else if label.len() <= 5 {
            ctx.key_size * 0.25
        } else {
            ctx.key_size * 0.18
        };
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            &label,
            egui::FontId::proportional(font_size),
            text_color,
        );
    }

    // Combo indicators: colored dots in top-right corner, one per combo
    if let Some(combos) = ctx.combo_map.get(&raw_kc) {
        let dot_radius = ctx.key_size * 0.06;
        for (i, combo) in combos.iter().enumerate() {
            let dot_pos = egui::pos2(
                rect.max.x - dot_radius * 2.5 - (i as f32) * dot_radius * 3.0,
                rect.min.y + dot_radius * 2.5,
            );
            painter.circle_filled(dot_pos, dot_radius, combo.color);
        }
    }

    let clicked = is_hovered && ui.input(|i| i.pointer.primary_clicked());

    if is_hovered {
        render_key_tooltip(ui, ctx, key_idx, key_pos.row, key_pos.col, raw_kc, &label);
    }

    KeyDrawResult {
        clicked,
        selected_rect: is_selected.then_some(rect),
    }
}

/// Hover tooltip for a key: keycode name + hex + matrix position, plus any
/// combos and tap-dance detail attached to that keycode.
fn render_key_tooltip(
    ui: &egui::Ui,
    ctx: &KeymapRender,
    key_idx: usize,
    row: u8,
    col: u8,
    raw_kc: u16,
    label: &str,
) {
    egui::Tooltip::always_open(
        ui.ctx().clone(),
        ui.layer_id(),
        ui.id().with(("key_tip", key_idx)),
        egui::PopupAnchor::Pointer,
    )
    .show(|ui| {
        ui.label(
            egui::RichText::new(format!("{label}\n0x{raw_kc:04X}  matrix ({row},{col})"))
                .monospace()
                .size(16.0),
        );
        if let Some(combos) = ctx.combo_map.get(&raw_kc) {
            ui.add_space(4.0);
            for combo in combos {
                ui.horizontal(|ui| {
                    let (r, _) = ui.allocate_exact_size(egui::vec2(8.0, 8.0), egui::Sense::hover());
                    ui.painter().circle_filled(r.center(), 4.0, combo.color);
                    ui.label(
                        egui::RichText::new(&combo.description)
                            .size(15.0)
                            .color(COL_COMBO_DESC),
                    );
                });
            }
        }
        if let Some(td_info) = ctx.td.summaries.get(&raw_kc) {
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new(format!("Tap Dance: {td_info}"))
                    .size(15.0)
                    .color(COL_TAPDANCE_INFO),
            );
        }
    });
}
