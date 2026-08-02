use gtk::prelude::*;
use relm4::adw::prelude::*;
use relm4::factory::{DynamicIndex, FactoryComponent, FactorySender, FactoryVecDeque};
use relm4::prelude::*;

use crate::config::{AppConfig, CameraConfig, ConnectionConfig, list_serial_ports};

// Camera row factory component
#[derive(Debug)]
struct CameraRow {
    camera_id: u64,
    name: String,
    connection_type: String,
}

#[derive(Debug)]
enum CameraRowMsg {}

#[derive(Debug)]
enum CameraRowOutput {
    Selected(u64),
}

#[relm4::factory]
impl FactoryComponent for CameraRow {
    type Init = CameraRow;
    type Input = CameraRowMsg;
    type Output = CameraRowOutput;
    type CommandOutput = ();
    type ParentWidget = adw::PreferencesGroup;

    view! {
        #[root]
        adw::ActionRow {
            set_title: &self.name,
            set_subtitle: &self.connection_type,
            set_activatable: true,

            add_suffix = &gtk::Image {
                set_icon_name: Some("go-next-symbolic"),
            },

            connect_activated[sender, camera_id = self.camera_id] => move |_| {
                let _ = sender.output(CameraRowOutput::Selected(camera_id));
            },
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        init
    }
}

// Main preferences dialog component
pub struct CameraPreferences {
    config: AppConfig,
    serial_ports: Vec<String>,
    camera_rows: FactoryVecDeque<CameraRow>,
    dialog: adw::PreferencesDialog,
    edit_page: Option<Controller<CameraEditPage>>,
}

#[derive(Debug)]
pub enum CameraPreferencesMsg {
    AddCamera,
    RemoveCamera(u64),
    UpdateCameraName(u64, String),
    UpdateConnectionType(u64, ConnectionType),
    UpdateIpAddress(u64, String),
    UpdateSerialPort(u64, String),
    UpdateBaudRate(u64, u32),
    CameraSelected(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    Ip,
    Serial,
}

impl From<&ConnectionConfig> for ConnectionType {
    fn from(config: &ConnectionConfig) -> Self {
        match config {
            ConnectionConfig::Ip { .. } => ConnectionType::Ip,
            ConnectionConfig::Serial { .. } => ConnectionType::Serial,
        }
    }
}

impl ConnectionType {
    fn as_str(&self) -> &'static str {
        match self {
            ConnectionType::Ip => "IP (TCP)",
            ConnectionType::Serial => "Serial (USB)",
        }
    }
}

#[derive(Debug)]
pub enum CameraPreferencesOutput {
    ConfigChanged(AppConfig),
}

#[relm4::component(pub)]
impl SimpleComponent for CameraPreferences {
    type Init = AppConfig;
    type Input = CameraPreferencesMsg;
    type Output = CameraPreferencesOutput;

    view! {
        adw::PreferencesDialog {
            set_title: "Camera Settings",
            set_content_width: 500,
            set_content_height: 600,

            add = &adw::PreferencesPage {
                set_title: "Cameras",
                set_icon_name: Some("camera-switch-symbolic"),

                #[local_ref]
                camera_list -> adw::PreferencesGroup {
                    set_title: "Cameras",
                    set_description: Some("Manage your camera connections"),
                    #[wrap(Some)]
                    set_header_suffix = &gtk::Button {
                        #[wrap(Some)]
                        set_child = &adw::ButtonContent {
                            set_label: "Add Camera",
                            set_icon_name: crate::icon_names::PLUS,
                            add_css_class: "flat",
                        },
                        add_css_class: "flat",
                        connect_clicked => CameraPreferencesMsg::AddCamera,
                    },
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let serial_ports = list_serial_ports();
        let dialog = root.clone();

        // Initialize factory
        let camera_rows = FactoryVecDeque::builder()
            .launch(adw::PreferencesGroup::new())
            .forward(sender.input_sender(), |output| match output {
                CameraRowOutput::Selected(id) => CameraPreferencesMsg::CameraSelected(id),
            });

        let mut model = CameraPreferences {
            config: init,
            serial_ports,
            camera_rows,
            dialog,
            edit_page: None,
        };

        // Populate initial camera rows
        for camera in &model.config.cameras {
            let conn_type = ConnectionType::from(&camera.connection);
            model.camera_rows.guard().push_back(CameraRow {
                camera_id: camera.id,
                name: camera.name.clone(),
                connection_type: conn_type.as_str().to_string(),
            });
        }

        let camera_list = model.camera_rows.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            CameraPreferencesMsg::AddCamera => {
                let name = format!("Camera {}", self.config.cameras.len() + 1);
                self.config.add_camera(name);
                self.config.save();

                // Only add the new camera to the factory
                if let Some(camera) = self.config.cameras.last() {
                    let conn_type = ConnectionType::from(&camera.connection);
                    self.camera_rows.guard().push_back(CameraRow {
                        camera_id: camera.id,
                        name: camera.name.clone(),
                        connection_type: conn_type.as_str().to_string(),
                    });

                    // Navigate to the edit page for the newly added camera
                    let camera_clone = camera.clone();
                    let edit_page = CameraEditPage::builder()
                        .launch((camera_clone, self.serial_ports.clone()))
                        .forward(sender.input_sender(), |output| match output {
                            CameraEditOutput::NameChanged(id, name) => {
                                CameraPreferencesMsg::UpdateCameraName(id, name)
                            }
                            CameraEditOutput::ConnectionTypeChanged(id, conn_type) => {
                                CameraPreferencesMsg::UpdateConnectionType(id, conn_type)
                            }
                            CameraEditOutput::IpAddressChanged(id, address) => {
                                CameraPreferencesMsg::UpdateIpAddress(id, address)
                            }
                            CameraEditOutput::SerialPortChanged(id, port) => {
                                CameraPreferencesMsg::UpdateSerialPort(id, port)
                            }
                            CameraEditOutput::BaudRateChanged(id, baud_rate) => {
                                CameraPreferencesMsg::UpdateBaudRate(id, baud_rate)
                            }
                            CameraEditOutput::DeleteCamera(id) => {
                                CameraPreferencesMsg::RemoveCamera(id)
                            }
                        });

                    self.dialog.push_subpage(edit_page.widget());
                    self.edit_page = Some(edit_page);
                }

                let _ = sender.output(CameraPreferencesOutput::ConfigChanged(self.config.clone()));
            }
            CameraPreferencesMsg::RemoveCamera(id) => {
                // Remove from config
                self.config.remove_camera(id);
                self.config.save();

                // Remove from factory by finding and removing the matching row
                let mut guard = self.camera_rows.guard();
                let index = guard.iter().position(|row| row.camera_id == id);
                if let Some(index) = index {
                    guard.remove(index);
                }

                // Pop the subpage to go back to the list
                self.dialog.pop_subpage();
                self.edit_page = None;

                let _ = sender.output(CameraPreferencesOutput::ConfigChanged(self.config.clone()));
            }
            CameraPreferencesMsg::UpdateCameraName(id, name) => {
                if let Some(cam) = self.config.cameras.iter_mut().find(|c| c.id == id) {
                    cam.name = name.clone();
                    self.config.save();

                    // Update the factory row
                    let mut guard = self.camera_rows.guard();
                    if let Some(row) = guard.iter_mut().find(|r| r.camera_id == id) {
                        row.name = name;
                    }

                    let _ =
                        sender.output(CameraPreferencesOutput::ConfigChanged(self.config.clone()));
                }
            }
            CameraPreferencesMsg::UpdateConnectionType(id, conn_type) => {
                if let Some(cam) = self.config.cameras.iter_mut().find(|c| c.id == id) {
                    cam.connection = match conn_type {
                        ConnectionType::Ip => ConnectionConfig::Ip {
                            address: "192.168.1.100".to_string(),
                        },
                        ConnectionType::Serial => ConnectionConfig::Serial {
                            port: self.serial_ports.first().cloned().unwrap_or_default(),
                            baud_rate: 115200,
                        },
                    };
                    self.config.save();

                    // Update the factory row
                    let mut guard = self.camera_rows.guard();
                    if let Some(row) = guard.iter_mut().find(|r| r.camera_id == id) {
                        row.connection_type = conn_type.as_str().to_string();
                    }

                    let _ =
                        sender.output(CameraPreferencesOutput::ConfigChanged(self.config.clone()));
                }
            }
            CameraPreferencesMsg::UpdateIpAddress(id, address) => {
                if let Some(cam) = self.config.cameras.iter_mut().find(|c| c.id == id) {
                    cam.connection = ConnectionConfig::Ip { address };
                    self.config.save();
                    let _ =
                        sender.output(CameraPreferencesOutput::ConfigChanged(self.config.clone()));
                }
            }
            CameraPreferencesMsg::UpdateSerialPort(id, port) => {
                if let Some(cam) = self.config.cameras.iter_mut().find(|c| c.id == id) {
                    if let ConnectionConfig::Serial { baud_rate, .. } = cam.connection {
                        cam.connection = ConnectionConfig::Serial { port, baud_rate };
                        self.config.save();
                        let _ = sender
                            .output(CameraPreferencesOutput::ConfigChanged(self.config.clone()));
                    }
                }
            }
            CameraPreferencesMsg::UpdateBaudRate(id, baud_rate) => {
                if let Some(cam) = self.config.cameras.iter_mut().find(|c| c.id == id) {
                    if let ConnectionConfig::Serial { ref port, .. } = cam.connection {
                        cam.connection = ConnectionConfig::Serial {
                            port: port.clone(),
                            baud_rate,
                        };
                        self.config.save();
                        let _ = sender
                            .output(CameraPreferencesOutput::ConfigChanged(self.config.clone()));
                    }
                }
            }
            CameraPreferencesMsg::CameraSelected(id) => {
                if let Some(camera) = self.config.cameras.iter().find(|c| c.id == id).cloned() {
                    let edit_page = CameraEditPage::builder()
                        .launch((camera, self.serial_ports.clone()))
                        .forward(sender.input_sender(), |output| match output {
                            CameraEditOutput::NameChanged(id, name) => {
                                CameraPreferencesMsg::UpdateCameraName(id, name)
                            }
                            CameraEditOutput::ConnectionTypeChanged(id, conn_type) => {
                                CameraPreferencesMsg::UpdateConnectionType(id, conn_type)
                            }
                            CameraEditOutput::IpAddressChanged(id, address) => {
                                CameraPreferencesMsg::UpdateIpAddress(id, address)
                            }
                            CameraEditOutput::SerialPortChanged(id, port) => {
                                CameraPreferencesMsg::UpdateSerialPort(id, port)
                            }
                            CameraEditOutput::BaudRateChanged(id, baud_rate) => {
                                CameraPreferencesMsg::UpdateBaudRate(id, baud_rate)
                            }
                            CameraEditOutput::DeleteCamera(id) => {
                                CameraPreferencesMsg::RemoveCamera(id)
                            }
                        });

                    self.dialog.push_subpage(edit_page.widget());
                    self.edit_page = Some(edit_page);
                }
            }
        }
    }
}

// Camera edit page component
pub struct CameraEditPage {
    camera: CameraConfig,
    serial_ports: Vec<String>,
}

#[derive(Debug)]
pub enum CameraEditMsg {
    UpdateName(String),
    UpdateConnectionType(ConnectionType),
    UpdateIpAddress(String),
    UpdateSerialPort(String),
    UpdateBaudRate(u32),
    DeleteCamera,
}

#[derive(Debug)]
pub enum CameraEditOutput {
    NameChanged(u64, String),
    ConnectionTypeChanged(u64, ConnectionType),
    IpAddressChanged(u64, String),
    SerialPortChanged(u64, String),
    BaudRateChanged(u64, u32),
    DeleteCamera(u64),
}

#[relm4::component(pub)]
impl SimpleComponent for CameraEditPage {
    type Init = (CameraConfig, Vec<String>);
    type Input = CameraEditMsg;
    type Output = CameraEditOutput;

    view! {
        adw::NavigationPage {
            set_title: &model.camera.name,

            #[wrap(Some)]
            set_child = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                append = &adw::HeaderBar {
                    set_show_end_title_buttons: true,
                },

                append = &adw::PreferencesPage {
                    set_title: &model.camera.name,
                    set_icon_name: Some("camera-switch-symbolic"),

                    add = &adw::PreferencesGroup {
                        set_title: "General",

                        add = &adw::EntryRow {
                            set_title: "Name",
                            set_text: &model.camera.name,
                            connect_changed[sender] => move |row| {
                                sender.input(CameraEditMsg::UpdateName(row.text().to_string()));
                            },
                        },

                        add = &adw::ComboRow {
                            set_title: "Connection Type",
                            set_model: Some(&gtk::StringList::new(&["IP (TCP)", "Serial (USB)"])),
                            set_selected: match ConnectionType::from(&model.camera.connection) {
                                ConnectionType::Ip => 0,
                                ConnectionType::Serial => 1,
                            },
                            connect_selected_notify[sender] => move |row| {
                                let new_type = match row.selected() {
                                    0 => ConnectionType::Ip,
                                    1 => ConnectionType::Serial,
                                    _ => return,
                                };
                                sender.input(CameraEditMsg::UpdateConnectionType(new_type));
                            },
                        },
                    },

                    add = &adw::PreferencesGroup {
                        set_title: "IP Settings",
                        #[watch]
                        set_visible: matches!(&model.camera.connection, ConnectionConfig::Ip { .. }),

                        add = &adw::EntryRow {
                            set_title: "IP Address",
                            set_text: match &model.camera.connection {
                                ConnectionConfig::Ip { address } => address.as_str(),
                                _ => "",
                            },
                            connect_changed[sender] => move |row| {
                                sender.input(CameraEditMsg::UpdateIpAddress(row.text().to_string()));
                            },
                        },
                    },

                    add = &adw::PreferencesGroup {
                        set_title: "Serial Settings",
                        #[watch]
                        set_visible: matches!(&model.camera.connection, ConnectionConfig::Serial { .. }),

                        add = &adw::ComboRow {
                            set_title: "Serial Port",
                            set_model: Some(&gtk::StringList::new(
                                &model.serial_ports.iter().map(|s| s.as_str()).collect::<Vec<&str>>()
                            )),
                            set_selected: match &model.camera.connection {
                                ConnectionConfig::Serial { port, .. } => {
                                    model.serial_ports.iter().position(|p| p == port).unwrap_or(0) as u32
                                }
                                _ => 0,
                            },
                            connect_selected_notify[sender, ports = model.serial_ports.clone()] => move |row| {
                                let idx = row.selected() as usize;
                                if idx < ports.len() {
                                    sender.input(CameraEditMsg::UpdateSerialPort(ports[idx].clone()));
                                }
                            },
                        },

                        add = &adw::ComboRow {
                            set_title: "Baud Rate",
                            set_model: Some(&gtk::StringList::new(&[
                                "9600", "19200", "38400", "57600", "115200"
                            ])),
                            set_selected: match &model.camera.connection {
                                ConnectionConfig::Serial { baud_rate, .. } => match baud_rate {
                                    9600 => 0,
                                    19200 => 1,
                                    38400 => 2,
                                    57600 => 3,
                                    115200 => 4,
                                    _ => 4,
                                },
                                _ => 4,
                            },
                            connect_selected_notify[sender] => move |row| {
                                let baud_rates = [9600, 19200, 38400, 57600, 115200];
                                let idx = row.selected() as usize;
                                if idx < baud_rates.len() {
                                    sender.input(CameraEditMsg::UpdateBaudRate(baud_rates[idx]));
                                }
                            },
                        },
                    },

                    add = &adw::PreferencesGroup {
                        set_margin_top: 24,

                        add = &gtk::Button {
                            set_label: "Delete Camera",
                            add_css_class: "destructive-action",
                            connect_clicked => CameraEditMsg::DeleteCamera,
                        },
                    },
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let (camera, serial_ports) = init;
        let model = CameraEditPage {
            camera,
            serial_ports,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            CameraEditMsg::UpdateName(name) => {
                self.camera.name = name.clone();
                sender
                    .output(CameraEditOutput::NameChanged(self.camera.id, name))
                    .unwrap();
            }
            CameraEditMsg::UpdateConnectionType(conn_type) => {
                self.camera.connection = match conn_type {
                    ConnectionType::Ip => ConnectionConfig::Ip {
                        address: "192.168.1.100".to_string(),
                    },
                    ConnectionType::Serial => ConnectionConfig::Serial {
                        port: self.serial_ports.first().cloned().unwrap_or_default(),
                        baud_rate: 115200,
                    },
                };
                sender
                    .output(CameraEditOutput::ConnectionTypeChanged(
                        self.camera.id,
                        conn_type,
                    ))
                    .unwrap();
            }
            CameraEditMsg::UpdateIpAddress(address) => {
                if let ConnectionConfig::Ip {
                    address: ref mut addr,
                } = self.camera.connection
                {
                    *addr = address.clone();
                    sender
                        .output(CameraEditOutput::IpAddressChanged(self.camera.id, address))
                        .unwrap();
                }
            }
            CameraEditMsg::UpdateSerialPort(port) => {
                if let ConnectionConfig::Serial {
                    port: ref mut p, ..
                } = self.camera.connection
                {
                    *p = port.clone();
                    sender
                        .output(CameraEditOutput::SerialPortChanged(self.camera.id, port))
                        .unwrap();
                }
            }
            CameraEditMsg::UpdateBaudRate(baud_rate) => {
                if let ConnectionConfig::Serial {
                    baud_rate: ref mut br,
                    ..
                } = self.camera.connection
                {
                    *br = baud_rate;
                    sender
                        .output(CameraEditOutput::BaudRateChanged(self.camera.id, baud_rate))
                        .unwrap();
                }
            }
            CameraEditMsg::DeleteCamera => {
                sender
                    .output(CameraEditOutput::DeleteCamera(self.camera.id))
                    .unwrap();
            }
        }
    }
}
