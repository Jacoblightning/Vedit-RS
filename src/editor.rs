use crate::keyboard;

use crate::VeditApp;

use eframe::egui;
use eframe::egui::RichText;

pub fn editor_display(app: &mut VeditApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::MenuBar::default().ui(ui, |ui| {
            let mut open_files: Vec<&crate::OpenFile> = app.open_file_mapping.values().collect();
            open_files.sort_by_key(|k| k.id);
            for file in open_files {
                let rtext = if file.backing_file_name.is_empty() {
                    RichText::new(format!(
                        "Untitled file {}{}",
                        file.id,
                        if file.saved { "" } else { " *" }
                    ))
                } else {
                    RichText::new(format!(
                        "{}{}",
                        &file.backing_file_name,
                        if file.saved { "" } else { " *" }
                    ))
                };

                if ui
                    .add(egui::Button::new(rtext.size(20.0)).selected(app.selected_file == file.id))
                    .clicked()
                {
                    app.selected_file = file.id;
                }
            }
        });

        if app.selected_file == 0 {
            ui.label(RichText::new("Open a file to get started!").size(36.0));
        } else {
            ui.separator();

            // The actual editor portion

            egui::ScrollArea::vertical().show(ui, |ui| {
                // Return the buffer for the currently selected file
    
                let text_editor = egui::TextEdit::multiline(
                    &mut app
                        .open_file_mapping
                        .get_mut(&app.selected_file)
                        .unwrap()
                        .buffer,
                )
                .frame(false)
                .font(egui::TextStyle::Heading);
    
                let response = ui.add_sized(ui.available_size(), text_editor);
                response.request_focus();
                // Little * save icon
                //app.open_file_mapping.get_mut(&app.selected_file).unwrap().saved = response.changed();
            });
        }
        keyboard::run_shortcuts(app, ui);
    });
}
