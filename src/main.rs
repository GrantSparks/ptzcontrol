use relm4::prelude::*;

mod app;
mod camera;
mod camera_preferences;
mod config;
mod d_pad;
mod header;
mod presets;
mod welcome;
mod zoom_focus;

mod icon_names {
    pub use shipped::*;
    include!(concat!(env!("OUT_DIR"), "/icon_names.rs"));
}

fn main() {
    tracing_subscriber::fmt::init();

    relm4_icons::initialize_icons(icon_names::GRESOURCE_BYTES, icon_names::RESOURCE_PREFIX);

    let css = ".rotate-180 { transform: rotate(180deg); }";
    let provider = gtk::CssProvider::new();
    provider.load_from_string(css);
    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let app = RelmApp::new("xyz.michaelt.ptzcontrol");
    app.run::<app::App>(0);
}
