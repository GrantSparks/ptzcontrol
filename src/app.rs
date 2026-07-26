use std::convert::identity;

use gtk::prelude::*;
use relm4::adw::prelude::*;
use relm4::prelude::*;

use crate::header::Header;

pub struct App {
    counter: u8,
    header: Controller<Header>,
}

#[derive(Debug)]
pub enum AppMsg {
    Increment,
    Decrement,
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = u8;
    type Input = AppMsg;
    type Output = ();

    view! {
        adw::ApplicationWindow {
            set_title: Some("PTZ Camera Controller"),
            set_default_size: (300, 100),

            #[wrap(Some)]
            set_content = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                append = model.header.widget(),

                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 5,
                    set_margin_all: 5,

                    gtk::Button {
                        set_label: "Increment",
                        connect_clicked => AppMsg::Increment,
                    },

                    gtk::Button {
                        set_label: "Decrement",
                        connect_clicked => AppMsg::Decrement,
                    },

                    gtk::Label {
                        #[watch]
                        set_label: &format!("Counter: {}", model.counter),
                        set_margin_all: 5,
                    }
                }
            }
        }
    }

    // Initialize the component.
    fn init(
        counter: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let header = Header::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let model = App { counter, header };

        // Insert the code generation of the view! macro here
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::Increment => {
                self.counter = self.counter.wrapping_add(1);
            }
            AppMsg::Decrement => {
                self.counter = self.counter.wrapping_sub(1);
            }
        }
    }
}
