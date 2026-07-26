use gtk::prelude::*;
use relm4::prelude::*;

use crate::camera::CameraAction;

pub struct ZoomFocus;

#[relm4::component(pub)]
impl SimpleComponent for ZoomFocus {
    type Init = ();
    type Input = ();
    type Output = CameraAction;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 20,
            set_halign: gtk::Align::Center,
            set_margin_all: 10,

            append = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,

                append = &gtk::Label {
                    set_label: "Zoom",
                    set_halign: gtk::Align::Start,
                },

                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 5,

                    #[name = "zoom_in_btn"]
                    gtk::Button {
                        set_icon_name: crate::icon_names::ZOOM_IN,
                        set_width_request: 80,
                        set_height_request: 80,
                    },

                    #[name = "zoom_out_btn"]
                    gtk::Button {
                        set_icon_name: crate::icon_names::ZOOM_OUT,
                        set_width_request: 80,
                        set_height_request: 80,
                    },
                },
            },

            append = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,

                append = &gtk::Label {
                    set_label: "Focus",
                    set_halign: gtk::Align::Start,
                },

                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 5,

                    #[name = "focus_near_btn"]
                    gtk::Button {
                        set_icon_name: crate::icon_names::LIST_ADD,
                        set_width_request: 80,
                        set_height_request: 80,
                    },

                    #[name = "focus_far_btn"]
                    gtk::Button {
                        set_icon_name: crate::icon_names::LIST_REMOVE,
                        set_width_request: 80,
                        set_height_request: 80,
                    },
                },
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ZoomFocus;
        let widgets = view_output!();

        let actions = [
            (&widgets.zoom_in_btn, CameraAction::ZoomIn, CameraAction::ZoomStop),
            (&widgets.zoom_out_btn, CameraAction::ZoomOut, CameraAction::ZoomStop),
            (&widgets.focus_near_btn, CameraAction::FocusNear, CameraAction::FocusStop),
            (&widgets.focus_far_btn, CameraAction::FocusFar, CameraAction::FocusStop),
        ];

        for (button, press_action, release_action) in actions {
            let gesture = gtk::GestureClick::new();
            gesture.set_propagation_phase(gtk::PropagationPhase::Capture);
            let sender_pressed = sender.clone();
            let action_pressed = press_action.clone();
            gesture.connect_pressed(move |_, _, _, _| {
                let _ = sender_pressed.output(action_pressed.clone());
            });
            let sender_released = sender.clone();
            let action_released = release_action.clone();
            gesture.connect_released(move |_, _, _, _| {
                let _ = sender_released.output(action_released.clone());
            });
            button.add_controller(gesture);
        }

        ComponentParts { model, widgets }
    }
}
