use crate::{VeditApp, common};
use eframe::egui;

pub fn run_shortcuts(app: &mut VeditApp, ui: &mut egui::Ui) {
    if ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::N)) {
        common::new_file(app);
    }
    if ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::Q)) {
        common::close_file(app);
    }
    if ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S)) {
        common::save_file(app);
    }
    if ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::O)) {
        common::open_files(app);
    }
}
