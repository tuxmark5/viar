use eframe::egui;
use via_protocol::KeyboardInfo;

use crate::types::{
    AppScreen,
    ViarApp,
};

impl ViarApp {
    pub fn render_detecting(&self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(ui.available_height() / 3.0);
            ui.heading("Detecting keyboards...");
            ui.spinner();
        });
    }

    pub fn render_no_permission(&self, ui: &mut egui::Ui) {
        let msg = match &self.screen {
            AppScreen::NoPermission(m) => m.clone(),
            _ => String::new(),
        };
        ui.vertical_centered(|ui| {
            ui.add_space(ui.available_height() / 3.0);
            ui.heading("Cannot access HID devices");
            ui.add_space(12.0);
            ui.label(&msg);
            ui.add_space(20.0);
            ui.label(
                "After adding a udev rule, unplug and replug your keyboard,\n\
                 or reload udev rules with: sudo udevadm control --reload-rules",
            );
        });
    }

    pub fn render_no_keyboards(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(ui.available_height() / 3.0);
            ui.heading("No VIA keyboards found");
            ui.add_space(12.0);
            ui.label("Make sure your keyboard firmware has VIA enabled.");
            ui.add_space(20.0);
            if ui.button("Retry").clicked() {
                self.refresh();
            }
        });
    }

    pub fn render_select_keyboard(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.heading("Select a keyboard");
            ui.add_space(20.0);
        });

        let mut connect_idx = None;
        let width = 400.0_f32.min(ui.available_width() - 40.0);

        ui.vertical_centered(|ui| {
            for (i, kb) in self.keyboards.iter().enumerate() {
                if self.render_keyboard_button(ui, kb, width).clicked() {
                    connect_idx = Some(i);
                }
                ui.add_space(4.0);
            }
        });

        if let Some(idx) = connect_idx {
            self.connect_to_keyboard(idx);
        }
    }

    /// Render one entry in the keyboard-selection list: a two-line button (name
    /// and vendor/product ids) with both lines centered. Returns its response.
    /// egui's multi-line text left-aligns its rows, so the lines are painted
    /// individually centered rather than as one block.
    fn render_keyboard_button(
        &self,
        ui: &mut egui::Ui,
        kb: &KeyboardInfo,
        width: f32,
    ) -> egui::Response {
        let (rect, response) = ui.allocate_at_least(egui::vec2(width, 50.0), egui::Sense::click());
        let response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
        let visuals = ui.style().interact(&response);
        ui.painter().rect(
            rect,
            visuals.corner_radius,
            visuals.weak_bg_fill,
            visuals.bg_stroke,
            egui::StrokeKind::Inside,
        );

        let cx = rect.center().x;
        let cy = rect.center().y;
        ui.painter().text(
            egui::pos2(cx, cy - 9.0),
            egui::Align2::CENTER_CENTER,
            format!("{} {}", kb.manufacturer, kb.product),
            egui::FontId::proportional(14.0),
            self.theme.text_primary(),
        );
        ui.painter().text(
            egui::pos2(cx, cy + 10.0),
            egui::Align2::CENTER_CENTER,
            format!("{:04x}:{:04x}", kb.vendor_id, kb.product_id),
            egui::FontId::proportional(12.0),
            self.theme.text_secondary(),
        );

        response
    }
}
