use std::collections::HashMap;

use eframe::egui;
use via_protocol::{
    EncoderKey,
    EncoderPosition,
    Keycode,
    KeycodeGroup,
};

use crate::{
    theme::Theme,
    types::{
        DynamicEntryData,
        EditTarget,
        FlashKind,
        KeyFlash,
        KeymapData,
        ViarApp,
    },
    ui::keycode_builder::render_keycode_builder,
    util::{
        CategoryStyle,
        aliased_name,
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

/// Pulse color for a "copied" flash.
const COL_FLASH_COPY: egui::Color32 = egui::Color32::from_rgb(120, 200, 255);
/// Pulse color for a "pasted" flash.
const COL_FLASH_PASTE: egui::Color32 = egui::Color32::from_rgb(130, 230, 150);
/// How long the copy/paste pulse animation lasts, in seconds.
const FLASH_DURATION: f64 = 0.45;

fn flash_color(kind: FlashKind) -> egui::Color32 {
    match kind {
        FlashKind::Copy => COL_FLASH_COPY,
        FlashKind::Paste => COL_FLASH_PASTE,
    }
}

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

        // Copy/paste pulse for this frame: (slot, 0..1 progress, color). Keep the
        // frame repainting while it plays out.
        let now = ui.input(|i| i.time);
        let flash = self.flash.and_then(|f| {
            let progress = ((now - f.start) / FLASH_DURATION) as f32;
            (progress < 1.0).then(|| (f.target, progress, flash_color(f.kind)))
        });
        if flash.is_some() {
            ui.ctx().request_repaint();
        }

        // --- Layer tabs, keyboard render, and selection handling ---
        // Scoped so the keymap_data borrow is released before the picker
        // (a &mut self method) runs.
        let (selected, selected_rect, layer_idx, copy, paste) = {
            let data = self.keymap_data.as_mut().unwrap();

            ui.horizontal(|ui| {
                ui.add_space(8.0);
                // A single "Layer" label, then just the numbers as tabs (there can
                // be many layers, so repeating "Layer N" gets noisy).
                ui.label(egui::RichText::new("Layer").color(self.theme.text_secondary()));
                for layer in 0..data.layer_count as usize {
                    let label = format!("{layer}");
                    let selected = data.selected_layer == layer;
                    if themed_tab(ui, selected, &label, &self.theme).clicked() {
                        data.selected_layer = layer;
                        data.selected = None;
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
                    flash,
                };
                render_keyboard(ui, &ctx)
            };
            let clicked = render.clicked;
            let selected_rect = render.selected_rect;
            let copy = render.copy;
            let paste = render.paste;

            // Toggle selection on click
            if let Some(target) = clicked {
                if data.selected == Some(target) {
                    data.selected = None;
                } else {
                    data.selected = Some(target);
                }
            }

            // Close picker on Escape
            if data.selected.is_some() && ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                data.selected = None;
            }

            // Close picker on click outside slots and picker (a shift+left paste
            // lands on a slot, not "outside", so it shouldn't close the picker).
            if data.selected.is_some() && clicked.is_none() && paste.is_none() {
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
                        data.selected = None;
                    }
                }
            }

            (data.selected, selected_rect, layer_idx, copy, paste)
        };

        // Copy / paste a slot's keycode (shift + right/left click), pulsing the
        // affected slot.
        if let Some(target) = copy {
            self.copy_slot(target);
            self.flash = Some(KeyFlash {
                target,
                start: now,
                kind: FlashKind::Copy,
            });
            ui.ctx().request_repaint();
        }
        if let Some(target) = paste {
            let pasted = self.copied_keycode.is_some();
            self.paste_slot(target);
            if pasted {
                self.flash = Some(KeyFlash {
                    target,
                    start: now,
                    kind: FlashKind::Paste,
                });
                ui.ctx().request_repaint();
            }
        }
        // Drop a finished flash so it doesn't linger in state.
        if self.flash.is_some_and(|f| now - f.start >= FLASH_DURATION) {
            self.flash = None;
        }

        // Floating popover picker (runs with keymap_data borrow released)
        if let (Some(target), Some(rect)) = (selected, selected_rect) {
            self.render_keycode_picker(ui, target, rect, layer_idx, aliases_ref);
        }

        // Handle deferred keycode application
        let pending: Option<(EditTarget, u16)> = ui.memory(|mem| {
            let kc: Option<u16> = mem.data.get_temp(egui::Id::new("pending_keycode"));
            let target: Option<EditTarget> = mem.data.get_temp(egui::Id::new("pending_target"));
            match (kc, target) {
                (Some(kc), Some(target)) => Some((target, kc)),
                _ => None,
            }
        });

        if let Some((target, new_kc)) = pending {
            ui.memory_mut(|mem| {
                mem.data.remove::<u16>(egui::Id::new("pending_keycode"));
                mem.data
                    .remove::<EditTarget>(egui::Id::new("pending_target"));
            });
            self.apply_edit(target, new_kc);
        }
    }

    /// Render the floating keycode picker popover for the selected slot.
    fn render_keycode_picker(
        &mut self,
        ui: &mut egui::Ui,
        target: EditTarget,
        anchor: egui::Rect,
        layer_idx: usize,
        aliases_ref: Option<&HashMap<String, String>>,
    ) {
        // Snapshot the slot's current keycode / header, then release the
        // keymap_data borrow so the picker body can freely touch other self fields.
        let (raw_kc, header, kc_name, kc_category) = {
            let Some(data) = self.keymap_data.as_ref() else {
                return;
            };
            let raw_kc = data.target_keycode(layer_idx, target);
            let keycode = Keycode(raw_kc);
            (
                raw_kc,
                target_header(layer_idx, target, data),
                aliased_name(raw_kc, aliases_ref),
                format!("{:?}", keycode.category()),
            )
        };

        let popover_w = 480.0_f32;
        let popover_h = 320.0_f32;
        let (pop_x, pop_y) =
            picker_popover_pos(anchor, ui.ctx().content_rect(), popover_w, popover_h);

        let mut open = true;
        egui::Window::new("Keycode Picker")
            .id(egui::Id::new("kc_picker"))
            .open(&mut open)
            .fixed_pos(egui::pos2(pop_x, pop_y))
            .fixed_size(egui::vec2(popover_w, popover_h))
            .collapsible(false)
            .title_bar(false)
            .show(ui.ctx(), |ui| {
                render_picker_header(ui, &header, &kc_name, raw_kc, &kc_category);
                ui.add_space(4.0);
                render_hex_input(ui, raw_kc, target);
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
                    queue_pending_keycode(ui, new_kc, target);
                }
            });

        // Close popover => clear selection
        if !open && let Some(data) = &mut self.keymap_data {
            data.selected = None;
        }
    }
}

/// Picker header line describing the slot being edited.
fn target_header(layer: usize, target: EditTarget, data: &KeymapData) -> String {
    match target {
        EditTarget::Encoder { index, clockwise } => {
            let dir = if clockwise { "CW ↻" } else { "CCW ↺" };
            format!("Layer {layer}  Enc {index} {dir}")
        }
        _ => match data.target_matrix(target) {
            Some((row, col)) => format!("Layer {layer}  [{row},{col}]"),
            None => format!("Layer {layer}"),
        },
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

/// Picker header: slot description, current keycode name + hex, category.
fn render_picker_header(
    ui: &mut egui::Ui,
    header: &str,
    kc_name: &str,
    raw_kc: u16,
    kc_category: &str,
) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(header).strong().size(17.0));
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
fn render_hex_input(ui: &mut egui::Ui, raw_kc: u16, target: EditTarget) {
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
            queue_pending_keycode(ui, v, target);
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
            queue_pending_keycode(ui, v, target);
        }
    });
}

/// Stash a keycode + edit target in egui memory for the main loop to apply after
/// the frame (avoids mutating self from inside the picker closures).
fn queue_pending_keycode(ui: &egui::Ui, keycode: u16, target: EditTarget) {
    ui.memory_mut(|mem| {
        mem.data
            .insert_temp(egui::Id::new("pending_keycode"), keycode);
        mem.data
            .insert_temp(egui::Id::new("pending_target"), target);
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
    /// Active copy/paste pulse: (flashing slot, 0..1 progress, color).
    flash:     Option<(EditTarget, f32, egui::Color32)>,
}

/// Interaction outcome of rendering keys / encoders for a frame.
#[derive(Default)]
struct RenderResult {
    /// The slot clicked this frame (plain left-click), if any.
    clicked:       Option<EditTarget>,
    /// Slot whose keycode should be copied (shift + right-click).
    copy:          Option<EditTarget>,
    /// Slot the copied keycode should be pasted into (shift + left-click).
    paste:         Option<EditTarget>,
    /// Screen rect of the selected slot, for anchoring the picker.
    selected_rect: Option<egui::Rect>,
}

impl RenderResult {
    /// Fold in a child slot's outcome, letting later hits win (matches draw
    /// order — the last thing drawn is on top).
    fn merge(&mut self, other: RenderResult) {
        if other.clicked.is_some() {
            self.clicked = other.clicked;
        }
        if other.copy.is_some() {
            self.copy = other.copy;
        }
        if other.paste.is_some() {
            self.paste = other.paste;
        }
        if other.selected_rect.is_some() {
            self.selected_rect = other.selected_rect;
        }
    }
}

/// Classify a click on a hovered slot into select / copy / paste based on the
/// mouse button and Shift modifier.
fn slot_click(ui: &egui::Ui, is_hovered: bool, target: EditTarget) -> RenderResult {
    if !is_hovered {
        return RenderResult::default();
    }
    let (primary, secondary, shift) = ui.input(|i| {
        (
            i.pointer.primary_clicked(),
            i.pointer.secondary_clicked(),
            i.modifiers.shift,
        )
    });
    RenderResult {
        // Plain left-click selects; Shift+left-click pastes.
        clicked:       (primary && !shift).then_some(target),
        paste:         (primary && shift).then_some(target),
        // Shift+right-click copies.
        copy:          (secondary && shift).then_some(target),
        selected_rect: None,
    }
}

/// Draw the copy/paste pulse on `rect` if `target` is the currently flashing
/// slot: a ring that fades out and expands outward over the flash duration.
fn draw_slot_flash(
    painter: &egui::Painter,
    ctx: &KeymapRender,
    rect: egui::Rect,
    rounding: egui::CornerRadius,
    target: EditTarget,
) {
    let Some((flash_target, progress, color)) = ctx.flash else {
        return;
    };
    if flash_target != target {
        return;
    }
    let flash_rect = rect.expand(progress * 4.0);
    painter.rect_stroke(
        flash_rect,
        rounding,
        egui::Stroke::new(2.0_f32, color.gamma_multiply(1.0 - progress)),
        egui::StrokeKind::Outside,
    );
}

/// Draw the whole keyboard — every keycap and encoder — for the selected layer,
/// aggregating the click and the selected slot's rect.
fn render_keyboard(ui: &egui::Ui, ctx: &KeymapRender) -> RenderResult {
    let mut result = RenderResult::default();
    for key_idx in 0..ctx.data.layout.keys.len() {
        result.merge(render_key(ui, ctx, key_idx));
    }
    for enc in &ctx.data.layout.encoders {
        result.merge(render_encoder(ui, ctx, enc));
    }
    for ek in &ctx.data.layout.encoder_keys {
        result.merge(render_encoder_key(ui, ctx, ek));
    }
    result
}

/// Draw a standard-format encoder direction key (its own position, one rotation
/// direction), like a keycap with a ↺/↻ glyph.
fn render_encoder_key(ui: &egui::Ui, ctx: &KeymapRender, ek: &EncoderKey) -> RenderResult {
    let px = ctx.origin.x + ek.x * ctx.key_size;
    let py = ctx.origin.y + ek.y * ctx.key_size;
    let pw = ek.w * ctx.key_size - ctx.gap;
    let ph = ek.h * ctx.key_size - ctx.gap;
    let slot = EncoderSlot {
        rect:     egui::Rect::from_min_size(egui::pos2(px, py), egui::vec2(pw, ph)),
        target:   EditTarget::Encoder {
            index:     ek.index,
            clockwise: ek.clockwise,
        },
        glyph:    if ek.clockwise { GLYPH_CW } else { GLYPH_CCW },
        rounding: egui::CornerRadius::same(4),
    };
    draw_encoder_slot(ui, ctx, &slot)
}

/// Draw one keycap: background, labels (split for tap-dance / dual-role keys),
/// combo indicator dots, and its hover tooltip.
fn render_key(ui: &egui::Ui, ctx: &KeymapRender, key_idx: usize) -> RenderResult {
    let key_pos = &ctx.data.layout.keys[key_idx];
    let painter = ui.painter();

    let raw_kc = ctx.data.keycode_at(ctx.layer_idx, key_pos.row, key_pos.col);
    let keycode = Keycode(raw_kc);

    let px = ctx.origin.x + key_pos.x * ctx.key_size;
    let py = ctx.origin.y + key_pos.y * ctx.key_size;
    let pw = key_pos.w * ctx.key_size - ctx.gap;
    let ph = key_pos.h * ctx.key_size - ctx.gap;

    let rect = egui::Rect::from_min_size(egui::pos2(px, py), egui::vec2(pw, ph));

    let is_selected = ctx.data.selected == Some(EditTarget::Key(key_idx));
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

    draw_slot_flash(painter, ctx, rect, rounding, EditTarget::Key(key_idx));

    if is_hovered {
        render_key_tooltip(ui, ctx, key_idx, key_pos.row, key_pos.col, raw_kc, &label);
    }

    let mut result = slot_click(ui, is_hovered, EditTarget::Key(key_idx));
    result.selected_rect = is_selected.then_some(rect);
    result
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

// ===========================================================================
// Encoders
// ===========================================================================

/// Counter-clockwise / clockwise rotation glyphs.
const GLYPH_CCW: &str = "↺";
const GLYPH_CW: &str = "↻";

/// How an encoder's rect is subdivided into editable slots. Chosen from whether
/// the encoder can be pushed and whether its button is square / wide / tall.
enum EncoderShape {
    /// Pushable + square: push across the top, CCW / CW across the bottom.
    SplitQuad,
    /// Pushable + wide: CCW | push | CW in a row.
    RowTriple,
    /// Pushable + tall: CCW / push / CW in a column.
    ColumnTriple,
    /// No push + wide/square: CCW | CW side by side.
    SplitHorizontal,
    /// No push + tall: CCW / CW stacked.
    SplitVertical,
}

/// Pick the split layout for an encoder from its push capability and shape.
fn encoder_shape(enc: &EncoderPosition) -> EncoderShape {
    let square = (enc.w - enc.h).abs() < 0.25;
    let wide = enc.w >= enc.h;
    match (enc.push.is_some(), square) {
        (true, true) => EncoderShape::SplitQuad,
        (true, false) if wide => EncoderShape::RowTriple,
        (true, false) => EncoderShape::ColumnTriple,
        (false, _) if wide => EncoderShape::SplitHorizontal,
        (false, _) => EncoderShape::SplitVertical,
    }
}

/// One editable region of an encoder (a rotation direction or the push switch).
struct EncoderSlot {
    rect:     egui::Rect,
    target:   EditTarget,
    /// Direction glyph, or "" for the push slot.
    glyph:    &'static str,
    /// Only the encoder's outer corners are rounded; shared inner edges stay
    /// square so the split reads as one control.
    rounding: egui::CornerRadius,
}

/// Corner radius rounding only the requested (outer) corners, squaring the rest.
fn outer_corners(nw: bool, ne: bool, sw: bool, se: bool) -> egui::CornerRadius {
    let r = 3;
    egui::CornerRadius {
        nw: if nw { r } else { 0 },
        ne: if ne { r } else { 0 },
        sw: if sw { r } else { 0 },
        se: if se { r } else { 0 },
    }
}

/// Split a rect into `n` equal columns (left to right).
fn split_h(rect: egui::Rect, n: usize) -> Vec<egui::Rect> {
    let w = rect.width() / n as f32;
    (0..n)
        .map(|i| {
            egui::Rect::from_min_size(
                egui::pos2(rect.min.x + w * i as f32, rect.min.y),
                egui::vec2(w, rect.height()),
            )
        })
        .collect()
}

/// Split a rect into `n` equal rows (top to bottom).
fn split_v(rect: egui::Rect, n: usize) -> Vec<egui::Rect> {
    let h = rect.height() / n as f32;
    (0..n)
        .map(|i| {
            egui::Rect::from_min_size(
                egui::pos2(rect.min.x, rect.min.y + h * i as f32),
                egui::vec2(rect.width(), h),
            )
        })
        .collect()
}

/// Build the slots for an encoder within `rect`, per its shape.
fn encoder_slots(enc: &EncoderPosition, rect: egui::Rect) -> Vec<EncoderSlot> {
    let ccw = EditTarget::Encoder {
        index:     enc.index,
        clockwise: false,
    };
    let cw = EditTarget::Encoder {
        index:     enc.index,
        clockwise: true,
    };
    let push = enc.push.map(|(row, col)| EditTarget::Push { row, col });
    let slot = |rect: egui::Rect, target, glyph, rounding| EncoderSlot {
        rect: rect.shrink(1.0),
        target,
        glyph,
        rounding,
    };

    match encoder_shape(enc) {
        // push spans the top (round the two top corners); CCW / CW take the
        // bottom-left / bottom-right corners.
        EncoderShape::SplitQuad => {
            let top = egui::Rect::from_min_max(rect.min, egui::pos2(rect.max.x, rect.center().y));
            let bl = egui::Rect::from_min_max(
                egui::pos2(rect.min.x, rect.center().y),
                egui::pos2(rect.center().x, rect.max.y),
            );
            let br = egui::Rect::from_min_max(rect.center(), rect.max);
            let mut slots = Vec::new();
            if let Some(p) = push {
                slots.push(slot(top, p, "", outer_corners(true, true, false, false)));
            }
            slots.push(slot(
                bl,
                ccw,
                GLYPH_CCW,
                outer_corners(false, false, true, false),
            ));
            slots.push(slot(
                br,
                cw,
                GLYPH_CW,
                outer_corners(false, false, false, true),
            ));
            slots
        }
        // CCW | push | CW: outer columns round their left / right corners.
        EncoderShape::RowTriple => {
            let cols = split_h(rect, 3);
            let mut slots = vec![slot(
                cols[0],
                ccw,
                GLYPH_CCW,
                outer_corners(true, false, true, false),
            )];
            if let Some(p) = push {
                slots.push(slot(
                    cols[1],
                    p,
                    "",
                    outer_corners(false, false, false, false),
                ));
            }
            slots.push(slot(
                cols[2],
                cw,
                GLYPH_CW,
                outer_corners(false, true, false, true),
            ));
            slots
        }
        // CCW / push / CW: outer rows round their top / bottom corners.
        EncoderShape::ColumnTriple => {
            let rows = split_v(rect, 3);
            let mut slots = vec![slot(
                rows[0],
                ccw,
                GLYPH_CCW,
                outer_corners(true, true, false, false),
            )];
            if let Some(p) = push {
                slots.push(slot(
                    rows[1],
                    p,
                    "",
                    outer_corners(false, false, false, false),
                ));
            }
            slots.push(slot(
                rows[2],
                cw,
                GLYPH_CW,
                outer_corners(false, false, true, true),
            ));
            slots
        }
        EncoderShape::SplitHorizontal => {
            let cols = split_h(rect, 2);
            vec![
                slot(
                    cols[0],
                    ccw,
                    GLYPH_CCW,
                    outer_corners(true, false, true, false),
                ),
                slot(
                    cols[1],
                    cw,
                    GLYPH_CW,
                    outer_corners(false, true, false, true),
                ),
            ]
        }
        EncoderShape::SplitVertical => {
            let rows = split_v(rect, 2);
            vec![
                slot(
                    rows[0],
                    ccw,
                    GLYPH_CCW,
                    outer_corners(true, true, false, false),
                ),
                slot(
                    rows[1],
                    cw,
                    GLYPH_CW,
                    outer_corners(false, false, true, true),
                ),
            ]
        }
    }
}

/// Draw one encoder as its shape-appropriate split of push / CCW / CW slots.
/// When encoder-map data is absent (unsupported firmware) only the push switch
/// is shown, so rotation editing stays hidden.
fn render_encoder(ui: &egui::Ui, ctx: &KeymapRender, enc: &EncoderPosition) -> RenderResult {
    let px = ctx.origin.x + enc.x * ctx.key_size;
    let py = ctx.origin.y + enc.y * ctx.key_size;
    let pw = enc.w * ctx.key_size - ctx.gap;
    let ph = enc.h * ctx.key_size - ctx.gap;
    let rect = egui::Rect::from_min_size(egui::pos2(px, py), egui::vec2(pw, ph));

    let rotation_enabled = ctx
        .data
        .layers
        .get(ctx.layer_idx)
        .is_some_and(|l| !l.encoders.is_empty());

    let slots = if rotation_enabled {
        encoder_slots(enc, rect)
    } else if let Some((row, col)) = enc.push {
        // Standalone push (rotation disabled): a normal fully-rounded key.
        vec![EncoderSlot {
            rect:     rect.shrink(1.0),
            target:   EditTarget::Push { row, col },
            glyph:    "",
            rounding: egui::CornerRadius::same(3),
        }]
    } else {
        Vec::new()
    };

    let mut result = RenderResult::default();
    for slot in &slots {
        result.merge(draw_encoder_slot(ui, ctx, slot));
    }
    result
}

/// Draw a single encoder slot (background, direction glyph, keycode label) and
/// report its click / selection like a key.
fn draw_encoder_slot(ui: &egui::Ui, ctx: &KeymapRender, slot: &EncoderSlot) -> RenderResult {
    let painter = ui.painter();
    let raw_kc = ctx.data.target_keycode(ctx.layer_idx, slot.target);
    let keycode = Keycode(raw_kc);
    let is_selected = ctx.data.selected == Some(slot.target);
    let is_hovered = ui.rect_contains_pointer(slot.rect);

    let bg = if is_selected {
        COL_SELECTED_BG
    } else if is_hovered {
        COL_HOVER_BG
    } else {
        keycode.category().bg()
    };
    let border = if is_selected {
        COL_SELECTED_BORDER
    } else {
        COL_BORDER
    };
    painter.rect_filled(slot.rect, slot.rounding, bg);
    painter.rect_stroke(
        slot.rect,
        slot.rounding,
        egui::Stroke::new(1.0_f32, border),
        egui::StrokeKind::Outside,
    );

    // Direction glyph in the top-left corner (push slots have none).
    if !slot.glyph.is_empty() {
        painter.text(
            slot.rect.left_top() + egui::vec2(3.0, 1.0),
            egui::Align2::LEFT_TOP,
            slot.glyph,
            egui::FontId::proportional((slot.rect.height() * 0.34).min(14.0)),
            if is_selected {
                egui::Color32::WHITE
            } else {
                COL_HOLD_LABEL
            },
        );
    }

    let text_color = if is_selected {
        egui::Color32::WHITE
    } else if raw_kc == 0 || raw_kc == 1 {
        COL_TEXT_DIM
    } else {
        COL_TEXT
    };
    let label = aliased_name(raw_kc, ctx.aliases);
    let font_size = (slot.rect.height() * 0.3).clamp(7.0, 14.0);
    painter.text(
        slot.rect.center(),
        egui::Align2::CENTER_CENTER,
        &label,
        egui::FontId::proportional(font_size),
        text_color,
    );

    draw_slot_flash(painter, ctx, slot.rect, slot.rounding, slot.target);

    let mut result = slot_click(ui, is_hovered, slot.target);
    result.selected_rect = is_selected.then_some(slot.rect);
    result
}
