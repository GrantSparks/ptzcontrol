use gtk::prelude::*;
use relm4::prelude::*;

pub struct Welcome;

#[derive(Debug)]
pub enum WelcomeMsg {
    AddCamera,
}

#[relm4::component(pub)]
impl SimpleComponent for Welcome {
    type Init = ();
    type Input = WelcomeMsg;
    type Output = WelcomeMsg;

    view! {
        adw::StatusPage {
            set_title: "Welcome to PTZ Camera Controller",
            set_description: Some("Add a camera to get started"),
            set_icon_name: Some("camera-switch-symbolic"),
            set_vexpand: true,

            #[wrap(Some)]
            set_child = &gtk::Button {
                set_label: "Add Camera",
                add_css_class: "suggested-action",
                add_css_class: "pill",
                set_halign: gtk::Align::Center,
                connect_clicked[sender] => move |_| {
                    let _ = sender.output(WelcomeMsg::AddCamera);
                },
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Welcome;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
