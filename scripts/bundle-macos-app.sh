#!/usr/bin/env bash
set -euo pipefail

APP_NAME="Minimal Terminal"
BINARY_NAME="terminal-app"
TARGET_DIR="target/debug"
APP_DIR="${TARGET_DIR}/${APP_NAME}.app"
CONTENTS_DIR="${APP_DIR}/Contents"
MACOS_DIR="${CONTENTS_DIR}/MacOS"

cargo build -p "${BINARY_NAME}"

rm -rf "${APP_DIR}"
mkdir -p "${MACOS_DIR}"

cp "${TARGET_DIR}/${BINARY_NAME}" "${MACOS_DIR}/${BINARY_NAME}"

cat > "${CONTENTS_DIR}/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
  "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleExecutable</key>
  <string>${BINARY_NAME}</string>
  <key>CFBundleIdentifier</key>
  <string>dev.minimal-terminal.app</string>
  <key>CFBundleName</key>
  <string>${APP_NAME}</string>
  <key>CFBundleDisplayName</key>
  <string>${APP_NAME}</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleShortVersionString</key>
  <string>0.1.0</string>
  <key>CFBundleVersion</key>
  <string>1</string>
  <key>LSMinimumSystemVersion</key>
  <string>12.0</string>
  <key>NSHighResolutionCapable</key>
  <true/>
</dict>
</plist>
PLIST

echo "${APP_DIR}"
