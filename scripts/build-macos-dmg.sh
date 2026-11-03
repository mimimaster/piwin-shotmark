#!/usr/bin/env bash
# Builds a signed, notarized, stapled macOS dmg.
#
# Tauri's own dmg bundler produced an empty volume on this machine (the app was
# never copied into the image), so the disk image is assembled by hand from the
# already-signed .app. Everything after `tauri build --bundles app` is plain
# Apple tooling.
#
# Required environment:
#   APPLE_SIGNING_IDENTITY  "Developer ID Application: ... (TEAMID)"
#   APPLE_API_KEY           App Store Connect key id
#   APPLE_API_ISSUER        App Store Connect issuer uuid
#   APPLE_API_KEY_PATH      path to AuthKey_<id>.p8
set -euo pipefail

: "${APPLE_SIGNING_IDENTITY:?set APPLE_SIGNING_IDENTITY}"
: "${APPLE_API_KEY:?set APPLE_API_KEY}"
: "${APPLE_API_ISSUER:?set APPLE_API_ISSUER}"
: "${APPLE_API_KEY_PATH:?set APPLE_API_KEY_PATH}"

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
VERSION="$(node -pe "require('$ROOT/package.json').version")"
APP="$ROOT/src-tauri/target/release/bundle/macos/piwin-shotmark.app"
OUT="$ROOT/src-tauri/target/release/bundle/dmg/piwin-shotmark_${VERSION}_$(uname -m).dmg"
STAGE="$(mktemp -d)"

[[ -d "$APP" ]] || { echo "missing $APP; run: npm run tauri build -- --bundles app" >&2; exit 1; }

cp -R "$APP" "$STAGE/"
ln -s /Applications "$STAGE/Applications"
mkdir -p "$(dirname "$OUT")"
rm -f "$OUT"
hdiutil create -volname "piwin-shotmark" -srcfolder "$STAGE" -ov -format UDZO "$OUT" -quiet
rm -rf "$STAGE"

codesign --force --sign "$APPLE_SIGNING_IDENTITY" --timestamp "$OUT"
xcrun notarytool submit "$OUT" --key "$APPLE_API_KEY_PATH" --key-id "$APPLE_API_KEY" \
  --issuer "$APPLE_API_ISSUER" --wait
xcrun stapler staple "$OUT"
spctl -a -t open --context context:primary-signature -vv "$OUT"
shasum -a 256 "$OUT"
