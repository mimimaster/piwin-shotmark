//! From a silent capture to a finished image: the overlay path (macOS, Windows)
//! and the OS-picker path (Linux), plus handing focus back to the app the user
//! was in.

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
use std::path::Path;

use tauri::AppHandle;

#[cfg(target_os = "linux")]
use super::linux;
#[cfg(target_os = "macos")]
use super::macos;
#[cfg(any(target_os = "macos", target_os = "windows"))]
use super::overlay;
#[cfg(target_os = "windows")]
use super::windows;
#[cfg(target_os = "windows")]
use super::windows_clipboard;
#[cfg(target_os = "macos")]
use super::collect_capture_file;
#[cfg(not(target_os = "windows"))]
use super::capture_temp_path;
use super::{
    failed, png_dimensions, CaptureError, ScreenCaptureOutcome, ScreenCaptureState, ERROR_FAILED,
    ERROR_UNSUPPORTED,
};
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
use super::WINDOW_SETTLE_DELAY;

/// The app the user was in, so focus can go back to it when the overlay took it.
pub(super) struct PreviousApp {
    #[cfg(target_os = "macos")]
    pid: Option<i32>,
    #[cfg(target_os = "windows")]
    window: isize,
}

impl PreviousApp {
    pub(super) fn record() -> Self {
        PreviousApp {
            #[cfg(target_os = "macos")]
            pid: macos::frontmost_other_app(),
            #[cfg(target_os = "windows")]
            window: windows::foreground_window(),
        }
    }

    pub(super) fn reactivate(&self) {
        #[cfg(target_os = "macos")]
        if let Some(pid) = self.pid {
            macos::activate_app(pid);
        }
        #[cfg(target_os = "windows")]
        windows::restore_foreground_window(self.window);
    }
}

/// macOS / Windows: freeze the monitor under the pointer, then let the overlay
/// pick the region and annotate it in place.
#[cfg(any(target_os = "macos", target_os = "windows"))]
pub(super) async fn capture_and_finish(
    app: &AppHandle,
    state: &ScreenCaptureState,
) -> ScreenCaptureOutcome {
    let shot = tauri::async_runtime::spawn_blocking(capture_monitor).await;
    let shot = match shot {
        Ok(Ok(shot)) => shot,
        Ok(Err(CaptureError::Unsupported)) => return failed(ERROR_UNSUPPORTED),
        Ok(Err(CaptureError::Failed(error))) => {
            eprintln!("[shotmark] screen capture failed: {error}");
            return failed(ERROR_FAILED);
        }
        Err(error) => {
            eprintln!("[shotmark] screen capture task failed: {error}");
            return failed(ERROR_FAILED);
        }
    };
    match overlay::run_overlay(app, &state.overlay, shot).await {
        Ok(overlay::OverlayReply::Finished(png)) => finish_overlay_capture(&png),
        Ok(overlay::OverlayReply::Pinned(png)) => pin_overlay_capture(app, state, png),
        Ok(overlay::OverlayReply::Cancelled) => ScreenCaptureOutcome::Cancelled,
        Err(error) => {
            eprintln!("[shotmark] screen capture overlay failed: {error}");
            failed(ERROR_FAILED)
        }
    }
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn pin_overlay_capture(
    app: &AppHandle,
    state: &ScreenCaptureState,
    png: Vec<u8>,
) -> ScreenCaptureOutcome {
    // Pin is an on-screen reference: copy like ✓, never a saved file.
    let outcome = finish_overlay_capture(&png);
    if let Err(error) = super::pin::open_pin_window(app, state, png) {
        eprintln!("[shotmark] screenshot pin window did not open: {error}");
    }
    outcome
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn finish_overlay_capture(png: &[u8]) -> ScreenCaptureOutcome {
    let Some((width, height)) = png_dimensions(png) else {
        return failed(ERROR_FAILED);
    };
    #[cfg(target_os = "macos")]
    let copy = macos::copy_png_to_clipboard(png);
    #[cfg(target_os = "windows")]
    let copy = windows_clipboard::copy_png_to_clipboard(png);
    if let Err(error) = &copy {
        eprintln!("[shotmark] screenshot was not copied to the clipboard: {error}");
    }
    ScreenCaptureOutcome::Captured {
        id: None,
        width,
        height,
        byte_length: png.len(),
        annotated: true,
        copied: copy.is_ok(),
    }
}

#[cfg(target_os = "macos")]
fn capture_monitor() -> Result<overlay::MonitorShot, CaptureError> {
    let output = capture_temp_path();
    let bounds = macos::capture_display_under_cursor(&output).map_err(CaptureError::Failed)?;
    match collect_capture_file(&output) {
        Ok(Some(png)) => Ok(overlay::MonitorShot { png, bounds }),
        Ok(None) => Err(CaptureError::Failed(
            "screencapture wrote no image".to_string(),
        )),
        Err(error) => Err(CaptureError::Failed(error.to_string())),
    }
}

#[cfg(target_os = "windows")]
fn capture_monitor() -> Result<overlay::MonitorShot, CaptureError> {
    windows::capture_cursor_monitor().map_err(CaptureError::Failed)
}

/// Linux: the portal (or a capture tool) draws the region picker, then the
/// result goes straight to the clipboard.
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub(super) async fn capture_and_finish(
    _app: &AppHandle,
    state: &ScreenCaptureState,
) -> ScreenCaptureOutcome {
    let output = capture_temp_path();
    let capture = tauri::async_runtime::spawn_blocking(move || {
        std::thread::sleep(WINDOW_SETTLE_DELAY);
        run_system_picker_capture(&output)
    })
    .await;
    let bytes = match capture {
        Ok(Ok(Some(bytes))) => bytes,
        Ok(Ok(None)) => return ScreenCaptureOutcome::Cancelled,
        Ok(Err(CaptureError::Unsupported)) => return failed(ERROR_UNSUPPORTED),
        Ok(Err(CaptureError::Failed(error))) => {
            eprintln!("[shotmark] screen capture failed: {error}");
            return failed(ERROR_FAILED);
        }
        Err(error) => {
            eprintln!("[shotmark] screen capture task failed: {error}");
            return failed(ERROR_FAILED);
        }
    };
    let Some((width, height)) = png_dimensions(&bytes) else {
        eprintln!("[shotmark] screen capture produced an unreadable PNG");
        return failed(ERROR_FAILED);
    };
    let byte_length = bytes.len();
    let copied = match copy_png_linux(&bytes) {
        Ok(()) => true,
        Err(error) => {
            eprintln!("[shotmark] PNG not copied to the clipboard: {error}");
            false
        }
    };
    // Without a clipboard entry the image still stays readable by id.
    let id = (!copied).then(|| state.store.put(bytes));
    ScreenCaptureOutcome::Captured {
        id,
        width,
        height,
        byte_length,
        annotated: false,
        copied,
    }
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn run_system_picker_capture(output: &Path) -> Result<Option<Vec<u8>>, CaptureError> {
    #[cfg(target_os = "linux")]
    {
        linux::capture_interactive(output)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = output;
        Err(CaptureError::Unsupported)
    }
}

/// Linux has no clipboard API in-tree, so hand the PNG to the session tool that
/// owns the clipboard: wl-copy on Wayland, xclip on X11.
#[cfg(target_os = "linux")]
fn copy_png_linux(png: &[u8]) -> Result<(), String> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let wayland = std::env::var_os("WAYLAND_DISPLAY").is_some();
    let (program, args): (&str, &[&str]) = if wayland {
        ("wl-copy", &["--type", "image/png"])
    } else {
        (
            "xclip",
            &["-selection", "clipboard", "-t", "image/png", "-i"],
        )
    };
    let mut child = Command::new(program)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| format!("{program} unavailable: {error}"))?;
    child
        .stdin
        .as_mut()
        .ok_or_else(|| format!("{program} produced no stdin"))?
        .write_all(png)
        .map_err(|error| error.to_string())?;
    child.wait().map_err(|error| error.to_string())?;
    Ok(())
}
