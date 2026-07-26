use relm4::prelude::*;

mod app;
mod header;

fn main() {
    let app = RelmApp::new("xyz.michaelt.ptzcontrol");
    app.run::<app::App>(0);
}
