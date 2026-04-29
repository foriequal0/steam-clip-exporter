use adw::Application;
use adw::prelude::*;
use gtk::gio;
use gtk::glib;

pub mod clip_info_boxed;
mod utils;
mod widgets;

use widgets::application_window::ApplicationWindow;

const APP_ID: &str = "io.github.foriequal0.SteamClipExporter";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("steam-clip-exporter.gresource")
        .expect("Failed to register resources");

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.present();
}
