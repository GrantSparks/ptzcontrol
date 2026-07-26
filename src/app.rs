use std::convert::identity;

use gtk::prelude::*;
use relm4::adw::prelude::*;
use relm4::prelude::*;

use crate::camera::{CameraAction, CameraService};
use crate::d_pad::DPad;
use crate::header::Header;
use crate::presets::Presets;
use crate::zoom_focus::ZoomFocus;

pub struct App {
    header: Controller<Header>,
    d_pad: Controller<DPad>,
    zoom_focus: Controller<ZoomFocus>,
    presets: Controller<Presets>,
    camera_service: CameraService,
}

#[derive(Debug)]
pub enum AppMsg {
    CameraAction(CameraAction),
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
        let camera_service = CameraService::start();

        let header = Header::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let d_pad = DPad::builder()
            .launch(())
            .forward(sender.input_sender(), AppMsg::CameraAction);

        let zoom_focus = ZoomFocus::builder()
            .launch(())
            .forward(sender.input_sender(), AppMsg::CameraAction);

        let presets = Presets::builder()
            .launch(())
            .forward(sender.input_sender(), AppMsg::CameraAction);

        let model = App {
            header,
            d_pad,
            zoom_focus,
            presets,
            camera_service,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::CameraAction(action) => {
                let _ = self.camera_service.sender().send(action);
            }
        }
    }
}
