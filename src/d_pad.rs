use gtk::prelude::*;
use relm4::prelude::*;

pub struct DPad;

#[relm4::component(pub)]
impl SimpleComponent for DPad {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        gtk::Grid {
            set_row_spacing: 5,
            set_column_spacing: 5,
            set_halign: gtk::Align::Center,
            set_margin_all: 10,

            attach[0, 0, 1, 1] = &gtk::Button {
                set_icon_name: crate::icon_names::ARROW_TOP_LEFT,
                set_width_request: 80,
                set_height_request: 80,
            },

            attach[1, 0, 1, 1] = &gtk::Button {
                set_icon_name: crate::icon_names::GO_UP,
                set_width_request: 80,
                set_height_request: 80,
            },

            attach[2, 0, 1, 1] = &gtk::Button {
                set_icon_name: crate::icon_names::ARROW_TOP_RIGHT,
                set_width_request: 80,
                set_height_request: 80,
            },

            attach[0, 1, 1, 1] = &gtk::Button {
                set_icon_name: crate::icon_names::GO_PREVIOUS,
                set_width_request: 80,
                set_height_request: 80,
            },

            attach[1, 1, 1, 1] = &gtk::Button {
                set_icon_name: crate::icon_names::GO_HOME,
                set_width_request: 80,
                set_height_request: 80,
            },

            attach[2, 1, 1, 1] = &gtk::Button {
                set_icon_name: crate::icon_names::GO_NEXT,
                set_width_request: 80,
                set_height_request: 80,
            },

            #[name = "down_left_btn"]
            attach[0, 2, 1, 1] = &gtk::Button {
                set_icon_name: crate::icon_names::ARROW_TOP_LEFT,
                set_width_request: 80,
                set_height_request: 80,
            },

            attach[1, 2, 1, 1] = &gtk::Button {
                set_icon_name: crate::icon_names::GO_DOWN,
                set_width_request: 80,
                set_height_request: 80,
            },

            #[name = "down_right_btn"]
            attach[2, 2, 1, 1] = &gtk::Button {
                set_icon_name: crate::icon_names::ARROW_TOP_RIGHT,
                set_width_request: 80,
                set_height_request: 80,
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = DPad;
        let widgets = view_output!();

        widgets.down_left_btn.set_css_classes(&["rotate-180"]);
        widgets.down_right_btn.set_css_classes(&["rotate-180"]);

        ComponentParts { model, widgets }
    }
}
