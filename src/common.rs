use std::fs;
use std::path::PathBuf;

use crate::{Confirmation, OpenFile, VeditApp};

use rfd::FileDialog;

pub fn new_file(app: &mut VeditApp) {
    app.last_file += 1;
    app.open_file_mapping.insert(
        app.last_file,
        OpenFile {
            id: app.last_file,
            buffer: "".into(),
            backing_file: None,
            backing_file_name: "".into(),
            saved: true,
        },
    );
    app.selected_file = app.last_file;
}

pub fn close_file(app: &mut VeditApp) {
    if app.selected_file == 0 {
        return;
    }

    let current_file = &app.open_file_mapping[&app.selected_file];

    if current_file.saved {
        // Then it's simple

        //TODO: Grap previous file from mapping and open that instead of current one????
        app.open_file_mapping.remove(&app.selected_file);
        app.selected_file = 0;
    } else {
        let fname = if current_file.backing_file_name.is_empty() {
            format!("Untitled file {}", current_file.id)
        } else {
            current_file.backing_file_name.clone()
        };

        app.confirmclose = Some(Confirmation { filename: fname });
    }
}

pub fn save_file_as(app: &mut VeditApp) {
    if app.selected_file == 0 {
        return;
    }

    let current_file = &mut app.open_file_mapping.get_mut(&app.selected_file).unwrap();

    // If the user picked a file
    if let Some(new_file) = FileDialog::new().save_file() {
        current_file.backing_file_name = new_file.display().to_string();

        current_file.saved = true;

        let _ = fs::write(&new_file, &current_file.buffer);

        current_file.backing_file = Some(new_file);
    }
}

pub fn save_file(app: &mut VeditApp) {
    if app.selected_file == 0 {
        return;
    }

    let current_file = &mut app.open_file_mapping.get_mut(&app.selected_file).unwrap();

    current_file.saved = true;

    if let Some(new_file) = &current_file.backing_file {
        let _ = fs::write(new_file, &current_file.buffer);
    } else {
        // If we don't have a backing file then it's the same as a save as.
        save_file_as(app);
    }
}

pub fn open_specific_file(app: &mut VeditApp, file: PathBuf) {
    app.last_file += 1;

    if let Ok(file_contents) = fs::read_to_string(&file) {
        let backing_name = file.display().to_string();
        app.open_file_mapping.insert(
            app.last_file,
            OpenFile {
                id: app.last_file,
                buffer: file_contents,
                backing_file: Some(file),
                backing_file_name: backing_name,
                saved: true,
            },
        );
        app.selected_file = app.last_file;
    }
}

pub fn open_files(app: &mut VeditApp) {
    if let Some(files) = FileDialog::new().pick_files() {
        for i in files {
            open_specific_file(app, i);
        }
    }
}
