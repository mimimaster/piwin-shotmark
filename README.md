# piwin-shotmark

A QQ-style screenshot tool for the desktop. Press the shortcut, the screen
freezes, drag out a region, annotate it in place, and the finished PNG is on
your clipboard.

Tray-resident: no main window, no account, no network calls.

- **macOS / Windows**: the app takes the shot itself and draws its own overlay
  (region select → toolbar next to the selection → ✓ / Enter / double-click to
  finish). Annotations: rectangle, ellipse, arrow, pen, mosaic, text, undo/redo,
  crop, pin.
- **Linux**: `xdg-desktop-portal` owns the region picker; the result goes to the
  clipboard through `wl-copy` (Wayland) or `xclip` (X11).

## Status

| Platform | Capture | Overlay + annotate | Clipboard | Verified here |
|---|---|---|---|---|
| macOS 13+ | `screencapture -x -R` | yes | `NSPasteboard` (PNG + TIFF) | built, launched, `cargo test` green — **screen capture needs the Screen Recording grant** |
| Windows 10+ | GDI `BitBlt` | yes | `CF_DIB` + registered `"PNG"` | **not run on hardware** — cross-checked only |
| Linux | portal / `grim` / `gnome-screenshot` | system picker (no in-app annotate yet) | `wl-copy` / `xclip` | **not run on hardware** — type-checked only |

The Windows and Linux columns are honest gaps, not marketing: the code paths
exist and compile for their targets, but nobody has put them on a real desktop
yet. See `docs/platform-support.md`.

## Install

No signed builds yet. Build from source:

```bash
npm install
npm run app:build     # produces a .app / .msi / .AppImage under src-tauri/target/release/bundle
```

## Use

| | Shortcut |
|---|---|
| macOS | `⇧⌘A` |
| Windows / Linux | `Ctrl+Shift+A` |

The tray icon does the same thing: left click captures, the menu has **截图** and
**退出**.

While capturing: drag to select (a single click takes the whole monitor),
right-click goes back to selecting, `Esc` cancels, the pin button keeps the shot
as a floating window.

## macOS permission

System Settings → Privacy & Security → **Screen Recording** → enable
`piwin-shotmark`, then relaunch. Without the grant the app reports
`SCREEN_CAPTURE_DENIED` instead of producing a wallpaper-only image.

## Development

```bash
npm install
npm run app:dev        # vite + tauri dev
cargo test --manifest-path src-tauri/Cargo.toml
npm run build          # typecheck + frontend bundle
```

Layout:

```
src-tauri/src/screen_capture/   capture backends, overlay window, pin windows
src/screen-capture/             React overlay editor (pure TS geometry + gesture)
src/lib/                        locale, shortcut display, Mantine wrappers
```

`src/screen-capture/annotation-model.ts` has no imports: it is the annotation
history and geometry model on its own.

## Relation to piwin

Extracted from the piwin desktop shell's built-in screenshot feature. The code
here carries no piwin dependency: no host service, no `@piwin/*` packages, no
account. Behaviour changes land here first only if piwin adopts them back.

## License

MIT. See `LICENSE`.
