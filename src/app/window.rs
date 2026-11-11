use eframe::egui;

use crate::tray::Tray;

pub struct FeatherweightApp {
    _tray: Tray,
}

impl FeatherweightApp {
    pub fn new(
        _cc: &eframe::CreationContext<'_>,
        tray: Tray,
    ) -> Self {
        Self { _tray: tray }
    }
}

impl eframe::App for FeatherweightApp {
    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        _frame: &mut eframe::Frame,
    ) {
        // Intercept window close requests (X button) and route them through
        // the tray's quit state instead of letting the window actually close.
        let close_requested = ui
            .ctx()
            .input(|i| i.viewport().close_requested());

        if close_requested && !self._tray.is_quit_requested() {
            // Cancel the actual window close and hide the window instead.
            ui.ctx().send_viewport_cmd(
                egui::ViewportCommand::CancelClose,
            );

            ui.ctx().send_viewport_cmd(
                egui::ViewportCommand::Visible(false),
            );
        }

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("EGUI-Tray-Template");
            ui.label("background activity, made lightweight.");
        });
    }
}