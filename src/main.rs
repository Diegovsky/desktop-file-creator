#![feature(trait_alias)]
use std::ffi::OsStr;
use std::path::Path;

use gtk::prelude::*;
use gtk::{glib};

use libadwaita::prelude::*;
use libadwaita::{ Application, ApplicationWindow, self as adw};

mod window;
mod dialogs;

const APP_ID: &str = "dev.diegovsky.DesktopFileCreator";

fn main() -> glib::ExitCode {
    color_eyre::install().unwrap();
    // Register and include resources
    gtk::gio::resources_register_include!("data.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(run);
    

    // Run the application
    app.run()
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Document {
    #[serde(rename = "Desktop Entry")]
    model: DesktopFileModel,
}

pub fn save_model(model: DesktopFileModel, filename: &str) -> color_eyre::Result<()> {
    let file = std::fs::File::create(filename)?;
    serde_ini::to_writer(file, &Document{model})?;
    Ok(())
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DesktopFileModel {
    name: String,
    exec: String,
    categories: String,
    icon: String,
}

fn run(app: &Application) {
    // Create a new window
    let window = window::Window::new(app);

    window.connect_file_open(|window, read| {
        if let Some(document) = window.handle_error(|| serde_ini::from_read::<_, Document>(read)) {
            window.set_model(document.model);
        }
    });
    

    window.connect_file_save(|window, mode| {
        let model = window.get_model();
        let filename = format!("{}.desktop", model.name);
        window.handle_error(|| save_model(model, &filename)).unwrap()
    });

    // Show the window
    window.set_visible(true);

}
