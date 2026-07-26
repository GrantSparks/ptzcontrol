use relm4::prelude::*;

mod app;
mod header;

mod icon_names {
    pub use shipped::*;
    include!(concat!(env!("OUT_DIR"), "/icon_names.rs"));
}

fn main() {
    relm4_icons::initialize_icons(icon_names::GRESOURCE_BYTES, icon_names::RESOURCE_PREFIX);

    let app = RelmApp::new("xyz.michaelt.ptzcontrol");
    app.run::<app::App>(0);
}
