fn main() {
    relm4_icons_build::bundle_icons(
        "icon_names.rs",
        Some("xyz.michaelt.ptzcontrol"),
        None::<&str>,
        None::<&str>,
        ["open-menu"],
    );
}
