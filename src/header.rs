use crate::app::AppMsg;
use gtk::prelude::*;
use relm4::adw::prelude::*;
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
            },

            #[name = "menu_button"]
            pack_end = &gtk::MenuButton {
                set_icon_name: crate::icon_names::OPEN_MENU,
                set_tooltip_text: Some("Main Menu"),
                set_primary: true,
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Header;
        let widgets = view_output!();

        let menu = gtk::gio::Menu::new();
        menu.append(Some("_About"), Some("app.about"));
        widgets.menu_button.set_menu_model(Some(&menu));

        let app = relm4::main_application();
        if app.lookup_action("about").is_none() {
            let app_for_action = app.clone();
            let about_action = gtk::gio::SimpleAction::new("about", None);
            about_action.connect_activate(move |_, _| {
                let dialog = adw::AboutDialog::builder()
                    .application_name("PTZ Camera Controller")
                    .application_icon("xyz.michaelt.ptzcontrol")
                    .version(env!("CARGO_PKG_VERSION"))
                    .developer_name("Michael Thomas")
                    .developers(["Michael Thomas"].as_slice())
                    .comments("Control PTZ cameras from your desktop.")
                    .license_type(gtk::License::Gpl30)
                    .copyright("© 2026 Michael Thomas")
                    .website("https://github.com/michaelhthomas/ptzcontrol")
                    .issue_url("https://github.com/michaelhthomas/ptzcontrol/issues")
                    .translator_credits("translator-credits")
                    .build();

                if let Some(window) = app_for_action.active_window() {
                    dialog.present(Some(&window));
                }
            });
            app.add_action(&about_action);
        }

        ComponentParts { model, widgets }
    }
}
