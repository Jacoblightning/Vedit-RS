#![feature(file_buffered)]
pub mod common;
mod editor;
pub mod keyboard;

use std::collections::HashMap;
use std::path::PathBuf;

use eframe::egui;
use eframe::egui::RichText;

// The intenal structure for an open file
struct OpenFile {
    id: u64,
    buffer: String,
    backing_file: Option<PathBuf>,
    backing_file_name: String,
    saved: bool,
}

struct Confirmation {
    filename: String,
}

#[derive(Default)]
pub struct VeditApp {
    // This stores the mapping of file IDs to the actual open file
    open_file_mapping: HashMap<u64, OpenFile>,
    // The ID of the file that is selected. As if there are any, at least one must be selected, 0 means there are no files
    selected_file: u64,
    // The last file ID that was created
    last_file: u64,
    // Whether we are currently showing a close confirmation window
    confirmclose: Option<Confirmation>,
    // Whether we are showing the about window
    about_open: bool,
}

fn main() -> eframe::Result {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };

    eframe::run_native(
        "Simple text editor app in Rust",
        options,
        Box::new(|_cc| Ok(Box::<VeditApp>::default())),
    )
}

impl eframe::App for VeditApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::default().ui(ui, |ui| {
                ui.menu_button(RichText::new("File").size(36.0), |ui| {
                    if ui
                        .button(RichText::new("New (Ctrl+N)").size(28.0))
                        .clicked()
                    {
                        common::new_file(self);
                    }
                    if ui
                        .button(RichText::new("Open (Ctrl+O)").size(28.0))
                        .clicked()
                    {
                        common::open_files(self);
                    }

                    // Only show these buttons if we have a file selected
                    let has_open_file = self.selected_file != 0;

                    if ui
                        .add_enabled(
                            has_open_file,
                            egui::Button::new(RichText::new("Save (Ctrl+S)").size(28.0)),
                        )
                        .clicked()
                    {
                        common::save_file(self);
                    }
                    if ui
                        .add_enabled(
                            has_open_file,
                            egui::Button::new(RichText::new("Save as...").size(28.0)),
                        )
                        .clicked()
                    {
                        common::save_file_as(self);
                    }
                    if ui
                        .add_enabled(
                            has_open_file,
                            egui::Button::new(RichText::new("Close (Ctrl+Q)").size(28.0)),
                        )
                        .clicked()
                    {
                        common::close_file(self);
                    }

                    if ui.button(RichText::new("Exit").size(28.0)).clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                //TODO: Edit menu
                ui.menu_button(RichText::new("Edit").size(36.0), |_ui| {});
                ui.menu_button(RichText::new("Help").size(36.0), |ui| {
                    if ui.button(RichText::new("About").size(28.0)).clicked() {
                        self.about_open = true;
                    }
                });
            });
        });

        editor::editor_display(self, ctx);

        // Saving
        if let Some(confirmer) = &self.confirmclose {
            egui::Window::new(format!("Save {} before closing?", confirmer.filename))
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("No").clicked() {
                            // It's safe to remove here as we immediatly set the selected file to 0 before another frame is rendered
                            self.open_file_mapping.remove(&self.selected_file);
                            self.selected_file = 0;
                            // Don't make this window appear again
                            self.confirmclose = None;
                        }

                        if ui.button("Yes").clicked() {
                            self.confirmclose = None;
                            common::save_file(self);
                            self.selected_file = 0;
                        }
                    });
                });
        }

        if self.about_open {
            egui::Window::new("About Vedit-RS").collapsible(false).resizable(true).show(ctx, |ui| {
                ui.heading("Vedit-RS (Visual Edit-RS) is a simple text editor I made for fun to improve my rust/egui skills.\n\nThere are still many more features I might add!");
                ui.separator();
                if ui.button("close").clicked() {
                    self.about_open = false;
                }
            });
        }
    }
}
