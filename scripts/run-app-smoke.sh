#!/usr/bin/env bash
set -euo pipefail

APP_DIR="target/debug/Minimal Terminal.app"
APP_BINARY="${APP_DIR}/Contents/MacOS/terminal-app"
SMOKE_SECONDS="${SMOKE_SECONDS:-3}"
LOG_DIR="target/debug/app-smoke"

scripts/bundle-macos-app.sh >/dev/null

if [[ ! -d "${APP_DIR}" ]]; then
  echo "app smoke failed: missing bundle ${APP_DIR}" >&2
  exit 1
fi

if [[ ! -x "${APP_BINARY}" ]]; then
  echo "app smoke failed: missing executable ${APP_BINARY}" >&2
  exit 1
fi

mkdir -p "${LOG_DIR}"

"${APP_BINARY}" >"${LOG_DIR}/stdout.log" 2>"${LOG_DIR}/stderr.log" &
pid=$!

cleanup() {
  osascript -e 'tell application id "dev.minimal-terminal.app" to quit' >/dev/null 2>&1 || true
  sleep 1
  if kill -0 "${pid}" >/dev/null 2>&1; then
    kill "${pid}" >/dev/null 2>&1 || true
    sleep 1
    wait "${pid}" >/dev/null 2>&1 || true
  fi
  if kill -0 "${pid}" >/dev/null 2>&1; then
    kill -9 "${pid}" >/dev/null 2>&1 || true
    wait "${pid}" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

sleep "${SMOKE_SECONDS}"

if ! kill -0 "${pid}" >/dev/null 2>&1; then
  wait "${pid}" || status=$?
  echo "app smoke failed: app exited before ${SMOKE_SECONDS}s, status=${status:-unknown}" >&2
  echo "stdout: ${LOG_DIR}/stdout.log" >&2
  echo "stderr: ${LOG_DIR}/stderr.log" >&2
  exit 1
fi

echo "app smoke passed: ${APP_BINARY} survived ${SMOKE_SECONDS}s"
