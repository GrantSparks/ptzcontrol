use gtk::prelude::*;
use relm4::prelude::*;

use crate::camera::CameraAction;

pub struct Presets;

#[relm4::component(pub)]
impl SimpleComponent for Presets {
    type Init = ();
    type Input = ();
    type Output = CameraAction;

    view! {
        gtk::Grid {
            set_row_spacing: 5,
            set_column_spacing: 5,
            set_hexpand: true,
            set_margin_all: 10,

            #[name = "preset_1_btn"]
            attach[0, 0, 1, 1] = &gtk::Button {
                set_label: "Preset 1",
                set_hexpand: true,
                set_height_request: 50,
            },

            #[name = "preset_2_btn"]
            attach[1, 0, 1, 1] = &gtk::Button {
                set_label: "Preset 2",
                set_hexpand: true,
                set_height_request: 50,
            },

            #[name = "preset_3_btn"]
            attach[2, 0, 1, 1] = &gtk::Button {
                set_label: "Preset 3",
                set_hexpand: true,
                set_height_request: 50,
            },

            #[name = "preset_4_btn"]
            attach[0, 1, 1, 1] = &gtk::Button {
                set_label: "Preset 4",
                set_hexpand: true,
                set_height_request: 50,
            },

            #[name = "preset_5_btn"]
            attach[1, 1, 1, 1] = &gtk::Button {
                set_label: "Preset 5",
                set_hexpand: true,
                set_height_request: 50,
            },

            #[name = "preset_6_btn"]
            attach[2, 1, 1, 1] = &gtk::Button {
                set_label: "Preset 6",
                set_hexpand: true,
                set_height_request: 50,
            },

            #[name = "preset_7_btn"]
            attach[0, 2, 1, 1] = &gtk::Button {
                set_label: "Preset 7",
                set_hexpand: true,
                set_height_request: 50,
            },

            #[name = "preset_8_btn"]
            attach[1, 2, 1, 1] = &gtk::Button {
                set_label: "Preset 8",
                set_hexpand: true,
                set_height_request: 50,
            },

            #[name = "preset_9_btn"]
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
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Presets;
        let widgets = view_output!();

        let preset_buttons = [
            (&widgets.preset_1_btn, 1),
            (&widgets.preset_2_btn, 2),
            (&widgets.preset_3_btn, 3),
            (&widgets.preset_4_btn, 4),
            (&widgets.preset_5_btn, 5),
            (&widgets.preset_6_btn, 6),
            (&widgets.preset_7_btn, 7),
            (&widgets.preset_8_btn, 8),
            (&widgets.preset_9_btn, 9),
        ];

        for (button, preset_num) in preset_buttons {
            let sender_clicked = sender.clone();
            let action = CameraAction::PresetRecall(preset_num);
            button.connect_clicked(move |_| {
                let _ = sender_clicked.output(action.clone());
            });
        }

        ComponentParts { model, widgets }
    }
}
