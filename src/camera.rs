use std::sync::mpsc;
use std::thread;

use grafton_visca::camera::profiles::GenericVisca;
#[cfg(feature = "simulator")]
use grafton_visca::camera::CameraBuilder;
use grafton_visca::command::{PanTiltDirection, PresetNumber};
use grafton_visca::runtime::TokioRuntime;
#[cfg(feature = "simulator")]
use grafton_visca::testing::ViscaCameraSimulator;
use grafton_visca::types::{PanSpeed, SpeedLevel, TiltSpeed};
use grafton_visca::{FocusControl, PanTiltControl, PresetsControl, ZoomControl};
use tracing::{error, info};

use crate::config::ConnectionConfig;

#[derive(Debug, Clone)]
pub enum CameraAction {
    PanTiltMove { direction: PanTiltDirection },
    PanTiltStop,
    PanTiltHome,
    ZoomIn,
    ZoomOut,
    ZoomStop,
    FocusNear,
    FocusFar,
    FocusStop,
    PresetRecall(u8),
}

#[derive(Debug, Clone)]
pub enum CameraConnection {
    #[cfg(feature = "simulator")]
    Simulator,
    Tcp { address: String },
    Serial { port: String, baud_rate: u32 },
}

impl From<&ConnectionConfig> for CameraConnection {
    fn from(config: &ConnectionConfig) -> Self {
        match config {
            ConnectionConfig::Ip { address } => CameraConnection::Tcp {
                address: address.clone(),
            },
            ConnectionConfig::Serial { port, baud_rate } => CameraConnection::Serial {
                port: port.clone(),
                baud_rate: *baud_rate,
            },
        }
    }
}

pub enum CameraCommand {
    Action(CameraAction),
    Reconnect(CameraConnection),
}

pub struct CameraService {
    tx: mpsc::Sender<CameraCommand>,
}

enum CameraHandle {
    #[cfg(feature = "simulator")]
    Simulator(
        grafton_visca::camera::Camera<
            grafton_visca::mode::Async,
            GenericVisca,
            ViscaCameraSimulator,
            TokioRuntime,
        >,
    ),
    Session(
        grafton_visca::camera::CameraSession<
            grafton_visca::mode::Async,
            GenericVisca,
            grafton_visca::runtime::TransportHandle<TokioRuntime>,
            TokioRuntime,
        >,
    ),
}

impl CameraService {
    pub fn start() -> Self {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let runtime = TokioRuntime::from_current().unwrap();
                let mut camera: Option<CameraHandle> = None;

                while let Ok(cmd) = rx.recv() {
                    match cmd {
                        CameraCommand::Action(action) => {
                            let Some(cam) = &mut camera else {
                                continue;
                            };
                            handle_action(cam, action).await;
                        }
                        CameraCommand::Reconnect(conn) => {
                            info!("Reconnecting camera: {:?}", conn);
                            match connect_camera(&runtime, conn).await {
                                Ok(cam) => {
                                    info!("Camera connected successfully");
                                    camera = Some(cam);
                                }
                                Err(e) => {
                                    error!("Failed to connect camera: {}", e);
                                    camera = None;
                                }
                            }
                        }
                    }
                }
            });
        });

        Self { tx }
    }

    pub fn send_action(&self, action: CameraAction) {
        let _ = self.tx.send(CameraCommand::Action(action));
    }

    pub fn reconnect(&self, connection: CameraConnection) {
        let _ = self.tx.send(CameraCommand::Reconnect(connection));
    }
}

async fn connect_camera(
    runtime: &TokioRuntime,
    connection: CameraConnection,
) -> Result<CameraHandle, String> {
    match connection {
        #[cfg(feature = "simulator")]
        CameraConnection::Simulator => {
            let simulator = ViscaCameraSimulator::new();
            let cam = CameraBuilder::with_executor(runtime.clone())
                .open_async::<GenericVisca, _>(simulator)
                .await
                .map_err(|e| format!("Simulator connection failed: {}", e))?;
            Ok(CameraHandle::Simulator(cam))
        }
        CameraConnection::Tcp { address } => {
            let config = grafton_visca::camera::CameraConfig::<GenericVisca>::tcp(&address);
            let session = config
                .open_async(runtime.clone())
                .await
                .map_err(|e| format!("TCP connection to {} failed: {}", address, e))?;
            Ok(CameraHandle::Session(session))
        }
        CameraConnection::Serial { port, baud_rate } => {
            let config =
                grafton_visca::camera::CameraConfig::<GenericVisca>::serial(&port, baud_rate);
            let session = config
                .open_serial_async(runtime.clone())
                .await
                .map_err(|e| format!("Serial connection to {} failed: {}", port, e))?;
            Ok(CameraHandle::Session(session))
        }
    }
}

async fn handle_action(camera: &mut CameraHandle, action: CameraAction) {
    match camera {
        #[cfg(feature = "simulator")]
        CameraHandle::Simulator(cam) => handle_simulator_action(cam, action).await,
        CameraHandle::Session(session) => handle_session_action(session, action).await,
    }
}

#[cfg(feature = "simulator")]
async fn handle_simulator_action(
    camera: &mut grafton_visca::camera::Camera<
        grafton_visca::mode::Async,
        GenericVisca,
        ViscaCameraSimulator,
        TokioRuntime,
    >,
    action: CameraAction,
) {
    match action {
        CameraAction::PanTiltMove { direction } => {
            info!("Camera: pan_tilt_move({:?})", direction);
            let _ = camera
                .pan_tilt_move(
                    direction,
                    PanSpeed::from(SpeedLevel::Medium),
                    TiltSpeed::from(SpeedLevel::Medium),
                )
                .await;
        }
        CameraAction::PanTiltStop => {
            info!("Camera: pan_tilt_stop()");
            let _ = camera.pan_tilt_stop().await;
        }
        CameraAction::PanTiltHome => {
            info!("Camera: pan_tilt_home()");
            let _ = camera.pan_tilt_home().await;
        }
        CameraAction::ZoomIn => {
            info!("Camera: zoom_tele()");
            let _ = camera.zoom_tele(None).await;
        }
        CameraAction::ZoomOut => {
            info!("Camera: zoom_wide()");
            let _ = camera.zoom_wide(None).await;
        }
        CameraAction::ZoomStop => {
            info!("Camera: zoom_stop()");
            let _ = camera.zoom_stop().await;
        }
        CameraAction::FocusNear => {
            info!("Camera: focus_near()");
            let _ = camera.focus_near(SpeedLevel::Medium).await;
        }
        CameraAction::FocusFar => {
            info!("Camera: focus_far()");
            let _ = camera.focus_far(SpeedLevel::Medium).await;
        }
        CameraAction::FocusStop => {
            info!("Camera: focus_stop()");
            let _ = camera.focus_stop().await;
        }
        CameraAction::PresetRecall(n) => {
            info!("Camera: preset_recall({})", n);
            if let Ok(preset) = PresetNumber::new(n) {
                let _ = camera.preset_recall(preset).await;
            }
        }
    }
}

async fn handle_session_action(
    camera: &mut grafton_visca::camera::CameraSession<
        grafton_visca::mode::Async,
        GenericVisca,
        grafton_visca::runtime::TransportHandle<TokioRuntime>,
        TokioRuntime,
    >,
    action: CameraAction,
) {
    match action {
        CameraAction::PanTiltMove { direction } => {
            info!("Camera: pan_tilt_move({:?})", direction);
            let _ = camera
                .pan_tilt_move(
                    direction,
                    PanSpeed::from(SpeedLevel::Medium),
                    TiltSpeed::from(SpeedLevel::Medium),
                )
                .await;
        }
        CameraAction::PanTiltStop => {
            info!("Camera: pan_tilt_stop()");
            let _ = camera.pan_tilt_stop().await;
        }
        CameraAction::PanTiltHome => {
            info!("Camera: pan_tilt_home()");
            let _ = camera.pan_tilt_home().await;
        }
        CameraAction::ZoomIn => {
            info!("Camera: zoom_tele()");
            let _ = camera.zoom_tele(None).await;
        }
        CameraAction::ZoomOut => {
            info!("Camera: zoom_wide()");
            let _ = camera.zoom_wide(None).await;
        }
        CameraAction::ZoomStop => {
            info!("Camera: zoom_stop()");
            let _ = camera.zoom_stop().await;
        }
        CameraAction::FocusNear => {
            info!("Camera: focus_near()");
            let _ = camera.focus_near(SpeedLevel::Medium).await;
        }
        CameraAction::FocusFar => {
            info!("Camera: focus_far()");
            let _ = camera.focus_far(SpeedLevel::Medium).await;
        }
        CameraAction::FocusStop => {
            info!("Camera: focus_stop()");
            let _ = camera.focus_stop().await;
        }
        CameraAction::PresetRecall(n) => {
            info!("Camera: preset_recall({})", n);
            if let Ok(preset) = PresetNumber::new(n) {
                let _ = camera.preset_recall(preset).await;
            }
        }
    }
}
