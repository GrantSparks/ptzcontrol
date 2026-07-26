use std::sync::mpsc;
use std::thread;

use grafton_visca::camera::profiles::GenericVisca;
use grafton_visca::camera::CameraBuilder;
use grafton_visca::command::{PanTiltDirection, PresetNumber};
use grafton_visca::runtime::TokioRuntime;
use grafton_visca::testing::ViscaCameraSimulator;
use grafton_visca::types::{PanSpeed, SpeedLevel, TiltSpeed};
use grafton_visca::{FocusControl, PanTiltControl, PresetsControl, ZoomControl};
use tracing::info;

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

pub struct CameraService {
    tx: mpsc::Sender<CameraAction>,
}

impl CameraService {
    pub fn start() -> Self {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let runtime = TokioRuntime::from_current().unwrap();
                let simulator = ViscaCameraSimulator::new();
                let camera = CameraBuilder::with_executor(runtime)
                    .open_async::<GenericVisca, _>(simulator)
                    .await
                    .unwrap();

                while let Ok(action) = rx.recv() {
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
            });
        });

        Self { tx }
    }

    pub fn sender(&self) -> mpsc::Sender<CameraAction> {
        self.tx.clone()
    }
}
