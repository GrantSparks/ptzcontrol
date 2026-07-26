use std::convert::identity;

use gtk::prelude::*;
use relm4::adw::prelude::*;
use relm4::prelude::*;

use crate::d_pad::DPad;
use crate::header::Header;
use crate::presets::Presets;
use crate::zoom_focus::ZoomFocus;

pub struct App {
    header: Controller<Header>,
    d_pad: Controller<DPad>,
    zoom_focus: Controller<ZoomFocus>,
    presets: Controller<Presets>,
}

#[derive(Debug)]
pub enum AppMsg {
    Noop,
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = u8;
    type Input = AppMsg;
    type Output = ();

    view! {
        adw::ApplicationWindow {
            set_title: Some("PTZ Camera Controller"),
            set_default_size: (400, 700),

            #[wrap(Some)]
            set_content = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                append = model.header.widget(),

                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 10,
                    set_margin_all: 10,

                    append = model.d_pad.widget(),
                    append = model.zoom_focus.widget(),
                    append = model.presets.widget(),
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let header = Header::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let d_pad = DPad::builder()
            .launch(())
            .forward(sender.input_sender(), |_| AppMsg::Noop);

        let zoom_focus = ZoomFocus::builder()
            .launch(())
            .forward(sender.input_sender(), |_| AppMsg::Noop);

        let presets = Presets::builder()
            .launch(())
            .forward(sender.input_sender(), |_| AppMsg::Noop);

        let model = App {
            header,
            d_pad,
            zoom_focus,
            presets,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, _msg: Self::Input, _sender: ComponentSender<Self>) {}
}
