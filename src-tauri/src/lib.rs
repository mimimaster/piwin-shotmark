//! piwin-shotmark: a tray-resident screenshot tool.
//!
//! Capture, annotation and the pin window all run in-process. There is no host
//! service and no account: the finished PNG goes to the system clipboard.

mod screen_capture;
mod tray;

use tauri::WindowEvent;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage(screen_capture::ScreenCaptureState::default())
        .invoke_handler(tauri::generate_handler![
            screen_capture::screen_capture_capabilities,
            screen_capture::screen_capture_authorization_status,
            screen_capture::screen_capture_request_authorization,
            screen_capture::screen_capture_open_system_settings,
            screen_capture::screen_capture_restart_app,
            screen_capture::screen_capture_read,
            screen_capture::overlay::screen_capture_overlay_image,
            screen_capture::overlay::screen_capture_overlay_ready,
            screen_capture::overlay::screen_capture_overlay_finish,
            screen_capture::overlay::screen_capture_overlay_pin,
            screen_capture::overlay::screen_capture_overlay_cancel,
            screen_capture::pin::screen_capture_pin,
            screen_capture::pin::screen_capture_pin_image,
            screen_capture::pin::screen_capture_pin_close,
        ])
        .setup(|app| {
            tray::install(app.handle())?;
            screen_capture::shortcut::install(app.handle());
            Ok(())
        })
        .on_window_event(|window, event| {
            // Closing the capture overlay cancels that capture, never the app.
            if window.label() == screen_capture::overlay::OVERLAY_LABEL {
                return;
            }
            // Screenshot pins are reference windows; closing one must not quit.
            if screen_capture::pin::is_pin_window_label(window.label()) {
                if matches!(event, WindowEvent::Moved(_) | WindowEvent::Focused(_)) {
                    let _ = window.set_always_on_top(true);
                    let _ = window.set_visible_on_all_workspaces(true);
                }
                return;
            }
        })
        .build(tauri::generate_context!())
        .expect("piwin-shotmark failed to build")
        .run(|_app, event| {
            // A tray app outlives its windows: only an explicit quit ends it.
            if let tauri::RunEvent::ExitRequested { code: None, api, .. } = event {
                api.prevent_exit();
            }
        });
}
