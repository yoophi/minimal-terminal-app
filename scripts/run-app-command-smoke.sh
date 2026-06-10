#!/usr/bin/env bash
set -euo pipefail

APP_DIR="target/debug/Minimal Terminal.app"
APP_BINARY="${APP_DIR}/Contents/MacOS/terminal-app"
LOG_DIR="target/debug/app-command-smoke"
SNAPSHOT_PATH="${LOG_DIR}/snapshot.txt"
STDOUT_PATH="${LOG_DIR}/stdout.log"
STDERR_PATH="${LOG_DIR}/stderr.log"
MARKER="minimal-terminal-app-command-smoke-ok"
WAIT_SECONDS="${WAIT_SECONDS:-8}"

scripts/bundle-macos-app.sh >/dev/null

if [[ ! -x "${APP_BINARY}" ]]; then
  echo "app command smoke failed: missing executable ${APP_BINARY}" >&2
  exit 1
fi

mkdir -p "${LOG_DIR}"
rm -f "${SNAPSHOT_PATH}" "${STDOUT_PATH}" "${STDERR_PATH}"

SMOKE_INPUT=$'printf "minimal-terminal-app-command-smoke-ok\\n"\n'

MINIMAL_TERMINAL_SMOKE_INPUT="${SMOKE_INPUT}" \
MINIMAL_TERMINAL_SMOKE_SNAPSHOT_PATH="${SNAPSHOT_PATH}" \
MINIMAL_TERMINAL_SMOKE_INPUT_DELAY_MS=500 \
MINIMAL_TERMINAL_SMOKE_SNAPSHOT_DELAY_MS=2000 \
MINIMAL_TERMINAL_SMOKE_EXIT=1 \
"${APP_BINARY}" >"${STDOUT_PATH}" 2>"${STDERR_PATH}" &
pid=$!

cleanup() {
  if kill -0 "${pid}" >/dev/null 2>&1; then
    kill "${pid}" >/dev/null 2>&1 || true
    wait "${pid}" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

deadline=$((SECONDS + WAIT_SECONDS))
while kill -0 "${pid}" >/dev/null 2>&1 && [[ "${SECONDS}" -lt "${deadline}" ]]; do
  sleep 0.2
done

if kill -0 "${pid}" >/dev/null 2>&1; then
  echo "app command smoke failed: app did not exit within ${WAIT_SECONDS}s" >&2
  exit 1
fi

wait "${pid}"

if [[ ! -f "${SNAPSHOT_PATH}" ]]; then
  echo "app command smoke failed: missing snapshot ${SNAPSHOT_PATH}" >&2
  echo "stdout: ${STDOUT_PATH}" >&2
  echo "stderr: ${STDERR_PATH}" >&2
  exit 1
fi

if ! grep -Fq "${MARKER}" "${SNAPSHOT_PATH}"; then
  echo "app command smoke failed: marker not found in snapshot" >&2
  echo "snapshot: ${SNAPSHOT_PATH}" >&2
  exit 1
fi

echo "app command smoke passed: ${MARKER}"
