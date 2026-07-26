use crate::app::AppMsg;
use gtk::prelude::*;
use relm4::prelude::*;

pub struct Header;

#[relm4::component(pub)]
impl SimpleComponent for Header {
    type Init = ();
    type Input = ();
    type Output = AppMsg;

    view! {
        adw::HeaderBar {
            #[wrap(Some)]
            set_title_widget = &adw::WindowTitle {
                set_title: "PTZ Camera Controller"
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Header;
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
