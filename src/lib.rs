mod canvas;
mod color;
mod render;
mod ui;

use gtk::prelude::*;
use gtk::Application;

/// Entry point to the editor application.
pub fn main_func() {
    let app = Application::builder()
        .application_id("me.kodopp.oxipaint")
        .build();

    app.connect_activate(|app| {
        ui::build_ui(app);
    });

    app.run();
}
