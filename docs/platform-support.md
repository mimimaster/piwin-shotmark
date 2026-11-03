# Platform support

What has actually been run, and what has only been compiled.

## macOS

Verified on this machine:

- `cargo test --manifest-path src-tauri/Cargo.toml` — 17 tests green.
- `npm run build` — both entry points bundle.
- `npm run tauri build -- --debug` — `.app` and `.dmg` produced.
- The bundled app launches, registers with the system and creates its tray item.

Not verified automatically, because it needs a human at the screen:

- First-run Screen Recording grant and the relaunch that follows it.
- The overlay covering the menu bar, multi-monitor placement, focus hand-back.
- Pasting the result into another app.

## Windows

Code path: `src-tauri/src/screen_capture/windows.rs` (GDI `BitBlt` of the monitor
under the cursor) and `windows_clipboard.rs` (`CF_DIB` plus the registered
`"PNG"` format).

Compiled for `x86_64-pc-windows-msvc` only. Nobody has run it on Windows.

## Linux

Code path: `linux.rs` (xdg-desktop-portal Screenshot), `file_uri.rs`, and
`capture_flow.rs::copy_png_linux` (`wl-copy` / `xclip`).

Type-checked only. Linux has no in-app annotation yet: the portal already picked
the region, and the editor window that piwin used for that path was not carried
over.

## Known gaps

- No signed or notarized installers, so no `brew install` / winget entry.
- No auto-update.
- No settings window: the shortcut is fixed.
- Linux annotation is missing (see above).
