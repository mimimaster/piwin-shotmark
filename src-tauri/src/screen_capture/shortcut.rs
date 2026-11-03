//! System-wide screenshot shortcut: ⇧⌘A on macOS, Ctrl+Shift+A elsewhere.
//!
//! The app is tray-resident, so there is no window to forward the key press to:
//! the capture starts right here. When registration fails (another app holds
//! the chord, or a Wayland session without X11 grabs), the tray menu still
//! starts the same flow.

use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

use super::{take, ScreenCaptureOutcome, ScreenCaptureState};

fn capture_shortcut() -> Shortcut {
    #[cfg(target_os = "macos")]
    let modifiers = Modifiers::SUPER | Modifiers::SHIFT;
    #[cfg(not(target_os = "macos"))]
    let modifiers = Modifiers::CONTROL | Modifiers::SHIFT;
    Shortcut::new(Some(modifiers), Code::KeyA)
}

pub(crate) fn install(app: &AppHandle) {
    let registered = app
        .global_shortcut()
        .on_shortcut(capture_shortcut(), |app, _shortcut, event| {
            if event.state != ShortcutState::Pressed {
                return;
            }
            spawn_capture(app);
        });
    if let Err(error) = registered {
        eprintln!("[shotmark] global screenshot shortcut unavailable, tray only: {error}");
    }
}

/// Starts one capture off the UI thread; the tray shares this path.
pub(crate) fn spawn_capture(app: &AppHandle) {
    let app = app.clone();
    tauri::async_runtime::spawn(async move {
        let state = app.state::<ScreenCaptureState>();
        match take(&app, &state).await {
            ScreenCaptureOutcome::Failed { code } => {
                eprintln!("[shotmark] capture failed: {code}");
            }
            ScreenCaptureOutcome::Captured { copied: false, .. } => {
                eprintln!("[shotmark] capture finished but nothing reached the clipboard");
            }
            _ => {}
        }
    });
}
