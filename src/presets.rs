use gtk::prelude::*;
use relm4::prelude::*;

pub struct Presets;

#[relm4::component(pub)]
impl SimpleComponent for Presets {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        gtk::Grid {
            set_row_spacing: 5,
            set_column_spacing: 5,
            set_hexpand: true,
            set_margin_all: 10,

            attach[0, 0, 1, 1] = &gtk::Button {
                set_label: "Preset 1",
                set_hexpand: true,
                set_height_request: 50,
            },

            attach[1, 0, 1, 1] = &gtk::Button {
                set_label: "Preset 2",
                set_hexpand: true,
                set_height_request: 50,
            },

            attach[2, 0, 1, 1] = &gtk::Button {
                set_label: "Preset 3",
                set_hexpand: true,
                set_height_request: 50,
            },

            attach[0, 1, 1, 1] = &gtk::Button {
                set_label: "Preset 4",
                set_hexpand: true,
                set_height_request: 50,
            },

            attach[1, 1, 1, 1] = &gtk::Button {
                set_label: "Preset 5",
                set_hexpand: true,
                set_height_request: 50,
            },

            attach[2, 1, 1, 1] = &gtk::Button {
                set_label: "Preset 6",
                set_hexpand: true,
                set_height_request: 50,
            },

            attach[0, 2, 1, 1] = &gtk::Button {
                set_label: "Preset 7",
                set_hexpand: true,
                set_height_request: 50,
            },

            attach[1, 2, 1, 1] = &gtk::Button {
                set_label: "Preset 8",
                set_hexpand: true,
                set_height_request: 50,
            },

            attach[2, 2, 1, 1] = &gtk::Button {
                set_label: "Preset 9",
                set_hexpand: true,
                set_height_request: 50,
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Presets;
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
