use gtk::prelude::*;
use relm4::prelude::*;

pub struct ZoomFocus;

#[relm4::component(pub)]
impl SimpleComponent for ZoomFocus {
    type Init = ();
    type Input = ();
    type Output = ();

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

                    gtk::Button {
                        set_icon_name: crate::icon_names::ZOOM_IN,
                        set_width_request: 80,
                        set_height_request: 80,
                    },

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

                    gtk::Button {
                        set_icon_name: crate::icon_names::LIST_ADD,
                        set_width_request: 80,
                        set_height_request: 80,
                    },

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
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ZoomFocus;
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
