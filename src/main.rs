use adw::Application;
use adw::prelude::*;
use gtk::gdk::Display;
use gtk::glib;
use gtk::{CssProvider, gio};

pub mod clip_info_boxed;
mod widgets;

use widgets::application_window::ApplicationWindow;

const APP_ID: &str = "io.github.foriequal0.SteamClipExporter";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("steam-clip-exporter.gresource")
        .expect("Failed to register resources");

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    app.run()
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_resource("/io/github/foriequal0/steam-clip-exporter/style.css");

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.present();
}
