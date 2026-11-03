//! Tray icon and menu. The app is tray-resident: no window opens at launch, and
//! closing every window must not quit. The tray is the permanent entry point.

use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::AppHandle;

use crate::screen_capture::shortcut::spawn_capture;

pub(crate) const TRAY_ID: &str = "shotmark";

pub(crate) fn install(app: &AppHandle) -> tauri::Result<()> {
    let capture = MenuItem::with_id(app, "capture", "截图  Ctrl+Shift+A", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let menu = Menu::with_items(
        app,
        &[&capture, &PredefinedMenuItem::separator(app)?, &quit],
    )?;

    let Some(icon) = app.default_window_icon().cloned() else {
        eprintln!("[shotmark] no bundle icon available, the tray stays hidden");
        return Ok(());
    };

    TrayIconBuilder::with_id(TRAY_ID)
        .icon(icon)
        .icon_as_template(true)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "capture" => spawn_capture(app),
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                spawn_capture(tray.app_handle());
            }
        })
        .build(app)?;
    Ok(())
}
