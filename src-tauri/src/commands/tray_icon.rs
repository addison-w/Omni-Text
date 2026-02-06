use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::image::Image;
use tauri::tray::TrayIcon;
use tauri::{AppHandle, Manager, Runtime};

/// Icon bytes embedded at compile time
const ICON_READY: &[u8] = include_bytes!("../../icons/cat-ready-44.png");
const ICON_PROCESSING_1: &[u8] = include_bytes!("../../icons/cat-processing-1-44.png");
const ICON_PROCESSING_2: &[u8] = include_bytes!("../../icons/cat-processing-2-44.png");
const ICON_ERROR: &[u8] = include_bytes!("../../icons/cat-error-44.png");

/// Holds the tray icon handle and animation state
pub struct TrayState<R: Runtime> {
    pub tray: TrayIcon<R>,
    pub animating: Arc<AtomicBool>,
}

pub fn set_tray_icon<R: Runtime>(tray: &TrayIcon<R>, icon_bytes: &[u8]) {
    if let Ok(img) = Image::from_bytes(icon_bytes) {
        let _ = tray.set_icon(Some(img));
    }
}

#[tauri::command]
pub fn set_tray_state(app: AppHandle, state: String) -> Result<(), String> {
    let tray_state = app
        .try_state::<TrayState<tauri::Wry>>()
        .ok_or("Tray state not initialized")?;

    // Stop any running animation
    tray_state.animating.store(false, Ordering::SeqCst);

    match state.as_str() {
        "ready" => {
            set_tray_icon(&tray_state.tray, ICON_READY);
            let _ = tray_state.tray.set_tooltip(Some("Omni Text"));
        }
        "processing" => {
            set_tray_icon(&tray_state.tray, ICON_PROCESSING_1);
            let _ = tray_state.tray.set_tooltip(Some("Omni Text - Processing..."));

            // Start blink animation
            let animating = tray_state.animating.clone();
            animating.store(true, Ordering::SeqCst);
            let app_handle = app.clone();

            std::thread::spawn(move || {
                let mut frame = false;
                while animating.load(Ordering::SeqCst) {
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    if !animating.load(Ordering::SeqCst) {
                        break;
                    }
                    if let Some(ts) = app_handle.try_state::<TrayState<tauri::Wry>>() {
                        let icon_bytes = if frame {
                            ICON_PROCESSING_1
                        } else {
                            ICON_PROCESSING_2
                        };
                        set_tray_icon(&ts.tray, icon_bytes);
                    }
                    frame = !frame;
                }
            });
        }
        "error" => {
            set_tray_icon(&tray_state.tray, ICON_ERROR);
            let _ = tray_state
                .tray
                .set_tooltip(Some("Omni Text - Error"));
        }
        _ => return Err(format!("Unknown tray state: {}", state)),
    }

    Ok(())
}
