#!/usr/bin/env bash
set -euo pipefail

APP_DIR="target/debug/Minimal Terminal.app"
APP_BINARY="${APP_DIR}/Contents/MacOS/terminal-app"
LOG_DIR="target/debug/app-target-smokes"
WAIT_SECONDS="${WAIT_SECONDS:-12}"

scripts/bundle-macos-app.sh >/dev/null

if [[ ! -x "${APP_BINARY}" ]]; then
  echo "app target smoke failed: missing executable ${APP_BINARY}" >&2
  exit 1
fi

mkdir -p "${LOG_DIR}"

run_case() {
  local name="$1"
  local input="$2"
  local marker="$3"
  local snapshot_delay_ms="${4:-2500}"
  local case_dir="${LOG_DIR}/${name}"
  local snapshot_path="${case_dir}/snapshot.txt"
  local stdout_path="${case_dir}/stdout.log"
  local stderr_path="${case_dir}/stderr.log"

  mkdir -p "${case_dir}"
  rm -f "${snapshot_path}" "${stdout_path}" "${stderr_path}"

  MINIMAL_TERMINAL_SMOKE_INPUT="${input}" \
  MINIMAL_TERMINAL_SMOKE_SNAPSHOT_PATH="${snapshot_path}" \
  MINIMAL_TERMINAL_SMOKE_INPUT_DELAY_MS=500 \
  MINIMAL_TERMINAL_SMOKE_SNAPSHOT_DELAY_MS="${snapshot_delay_ms}" \
  MINIMAL_TERMINAL_SMOKE_EXIT=1 \
  "${APP_BINARY}" >"${stdout_path}" 2>"${stderr_path}" &
  local pid=$!

  local deadline=$((SECONDS + WAIT_SECONDS))
  while kill -0 "${pid}" >/dev/null 2>&1 && [[ "${SECONDS}" -lt "${deadline}" ]]; do
    sleep 0.2
  done

  if kill -0 "${pid}" >/dev/null 2>&1; then
    kill "${pid}" >/dev/null 2>&1 || true
    wait "${pid}" >/dev/null 2>&1 || true
    echo "app target smoke failed: ${name} did not exit within ${WAIT_SECONDS}s" >&2
    exit 1
  fi

  wait "${pid}"

  if [[ ! -f "${snapshot_path}" ]]; then
    echo "app target smoke failed: ${name} missing snapshot ${snapshot_path}" >&2
    exit 1
  fi

  if ! grep -Fq "${marker}" "${snapshot_path}"; then
    echo "app target smoke failed: ${name} marker not found: ${marker}" >&2
    echo "snapshot: ${snapshot_path}" >&2
    exit 1
  fi

  echo "app target smoke passed: ${name}"
}

run_case_with_required_markers() {
  local name="$1"
  local input="$2"
  local snapshot_delay_ms="$3"
  shift 3
  local case_dir="${LOG_DIR}/${name}"
  local snapshot_path="${case_dir}/snapshot.txt"
  local stdout_path="${case_dir}/stdout.log"
  local stderr_path="${case_dir}/stderr.log"

  mkdir -p "${case_dir}"
  rm -f "${snapshot_path}" "${stdout_path}" "${stderr_path}"

  MINIMAL_TERMINAL_SMOKE_INPUT="${input}" \
  MINIMAL_TERMINAL_SMOKE_SNAPSHOT_PATH="${snapshot_path}" \
  MINIMAL_TERMINAL_SMOKE_INPUT_DELAY_MS=500 \
  MINIMAL_TERMINAL_SMOKE_SNAPSHOT_DELAY_MS="${snapshot_delay_ms}" \
  MINIMAL_TERMINAL_SMOKE_EXIT=1 \
  "${APP_BINARY}" >"${stdout_path}" 2>"${stderr_path}" &
  local pid=$!

  local deadline=$((SECONDS + WAIT_SECONDS))
  while kill -0 "${pid}" >/dev/null 2>&1 && [[ "${SECONDS}" -lt "${deadline}" ]]; do
    sleep 0.2
  done

  if kill -0 "${pid}" >/dev/null 2>&1; then
    kill "${pid}" >/dev/null 2>&1 || true
    wait "${pid}" >/dev/null 2>&1 || true
    echo "app target smoke failed: ${name} did not exit within ${WAIT_SECONDS}s" >&2
    exit 1
  fi

  wait "${pid}"

  if [[ ! -f "${snapshot_path}" ]]; then
    echo "app target smoke failed: ${name} missing snapshot ${snapshot_path}" >&2
    exit 1
  fi

  local marker
  for marker in "$@"; do
    if ! grep -Fq "${marker}" "${snapshot_path}"; then
      echo "app target smoke failed: ${name} marker not found: ${marker}" >&2
      echo "snapshot: ${snapshot_path}" >&2
      exit 1
    fi
  done

  echo "app target smoke passed: ${name}"
}

run_case_with_followup() {
  local name="$1"
  local input="$2"
  local followup_input="$3"
  local marker="$4"
  local snapshot_delay_ms="${5:-2500}"
  local followup_delay_ms="${6:-1000}"
  local case_dir="${LOG_DIR}/${name}"
  local snapshot_path="${case_dir}/snapshot.txt"
  local stdout_path="${case_dir}/stdout.log"
  local stderr_path="${case_dir}/stderr.log"

  mkdir -p "${case_dir}"
  rm -f "${snapshot_path}" "${stdout_path}" "${stderr_path}"

  MINIMAL_TERMINAL_SMOKE_INPUT="${input}" \
  MINIMAL_TERMINAL_SMOKE_FOLLOWUP_INPUT="${followup_input}" \
  MINIMAL_TERMINAL_SMOKE_SNAPSHOT_PATH="${snapshot_path}" \
  MINIMAL_TERMINAL_SMOKE_INPUT_DELAY_MS=500 \
  MINIMAL_TERMINAL_SMOKE_FOLLOWUP_INPUT_DELAY_MS="${followup_delay_ms}" \
  MINIMAL_TERMINAL_SMOKE_SNAPSHOT_DELAY_MS="${snapshot_delay_ms}" \
  MINIMAL_TERMINAL_SMOKE_EXIT=1 \
  "${APP_BINARY}" >"${stdout_path}" 2>"${stderr_path}" &
  local pid=$!

  local deadline=$((SECONDS + WAIT_SECONDS))
  while kill -0 "${pid}" >/dev/null 2>&1 && [[ "${SECONDS}" -lt "${deadline}" ]]; do
    sleep 0.2
  done

  if kill -0 "${pid}" >/dev/null 2>&1; then
    kill "${pid}" >/dev/null 2>&1 || true
    wait "${pid}" >/dev/null 2>&1 || true
    echo "app target smoke failed: ${name} did not exit within ${WAIT_SECONDS}s" >&2
    exit 1
  fi

  wait "${pid}"

  if [[ ! -f "${snapshot_path}" ]]; then
    echo "app target smoke failed: ${name} missing snapshot ${snapshot_path}" >&2
    exit 1
  fi

  if ! grep -Fq "${marker}" "${snapshot_path}"; then
    echo "app target smoke failed: ${name} marker not found: ${marker}" >&2
    echo "snapshot: ${snapshot_path}" >&2
    exit 1
  fi

  echo "app target smoke passed: ${name}"
}

run_case_with_resize_followup() {
  local name="$1"
  local input="$2"
  local resize="$3"
  local followup_input="$4"
  local marker="$5"
  local snapshot_delay_ms="${6:-2500}"
  local resize_delay_ms="${7:-1000}"
  local followup_delay_ms="${8:-1000}"
  local case_dir="${LOG_DIR}/${name}"
  local snapshot_path="${case_dir}/snapshot.txt"
  local stdout_path="${case_dir}/stdout.log"
  local stderr_path="${case_dir}/stderr.log"

  mkdir -p "${case_dir}"
  rm -f "${snapshot_path}" "${stdout_path}" "${stderr_path}"

  MINIMAL_TERMINAL_SMOKE_INPUT="${input}" \
  MINIMAL_TERMINAL_SMOKE_RESIZE="${resize}" \
  MINIMAL_TERMINAL_SMOKE_FOLLOWUP_INPUT="${followup_input}" \
  MINIMAL_TERMINAL_SMOKE_SNAPSHOT_PATH="${snapshot_path}" \
  MINIMAL_TERMINAL_SMOKE_INPUT_DELAY_MS=500 \
  MINIMAL_TERMINAL_SMOKE_RESIZE_DELAY_MS="${resize_delay_ms}" \
  MINIMAL_TERMINAL_SMOKE_FOLLOWUP_INPUT_DELAY_MS="${followup_delay_ms}" \
  MINIMAL_TERMINAL_SMOKE_SNAPSHOT_DELAY_MS="${snapshot_delay_ms}" \
  MINIMAL_TERMINAL_SMOKE_EXIT=1 \
  "${APP_BINARY}" >"${stdout_path}" 2>"${stderr_path}" &
  local pid=$!

  local deadline=$((SECONDS + WAIT_SECONDS))
  while kill -0 "${pid}" >/dev/null 2>&1 && [[ "${SECONDS}" -lt "${deadline}" ]]; do
    sleep 0.2
  done

  if kill -0 "${pid}" >/dev/null 2>&1; then
    kill "${pid}" >/dev/null 2>&1 || true
    wait "${pid}" >/dev/null 2>&1 || true
    echo "app target smoke failed: ${name} did not exit within ${WAIT_SECONDS}s" >&2
    exit 1
  fi

  wait "${pid}"

  if [[ ! -f "${snapshot_path}" ]]; then
    echo "app target smoke failed: ${name} missing snapshot ${snapshot_path}" >&2
    exit 1
  fi

  if ! grep -Fq "${marker}" "${snapshot_path}"; then
    echo "app target smoke failed: ${name} marker not found: ${marker}" >&2
    echo "snapshot: ${snapshot_path}" >&2
    exit 1
  fi

  echo "app target smoke passed: ${name}"
}

run_case_with_native_window_resize() {
  local name="$1"
  local input="$2"
  local resize="$3"
  local marker="$4"
  local snapshot_delay_ms="${5:-3500}"
  local case_dir="${LOG_DIR}/${name}"
  local snapshot_path="${case_dir}/snapshot.txt"
  local stdout_path="${case_dir}/stdout.log"
  local stderr_path="${case_dir}/stderr.log"

  mkdir -p "${case_dir}"
  rm -f "${snapshot_path}" "${stdout_path}" "${stderr_path}"

  MINIMAL_TERMINAL_SMOKE_INPUT="${input}" \
  MINIMAL_TERMINAL_SMOKE_NATIVE_WINDOW_RESIZE="${resize}" \
  MINIMAL_TERMINAL_SMOKE_SNAPSHOT_PATH="${snapshot_path}" \
  MINIMAL_TERMINAL_SMOKE_INPUT_DELAY_MS=500 \
  MINIMAL_TERMINAL_SMOKE_SNAPSHOT_DELAY_MS="${snapshot_delay_ms}" \
  MINIMAL_TERMINAL_SMOKE_EXIT=1 \
  "${APP_BINARY}" >"${stdout_path}" 2>"${stderr_path}" &
  local pid=$!

  local deadline=$((SECONDS + WAIT_SECONDS))
  while kill -0 "${pid}" >/dev/null 2>&1 && [[ "${SECONDS}" -lt "${deadline}" ]]; do
    sleep 0.2
  done

  if kill -0 "${pid}" >/dev/null 2>&1; then
    kill "${pid}" >/dev/null 2>&1 || true
    wait "${pid}" >/dev/null 2>&1 || true
    echo "app target smoke failed: ${name} did not exit within ${WAIT_SECONDS}s" >&2
    exit 1
  fi

  wait "${pid}"

  if [[ ! -f "${snapshot_path}" ]]; then
    echo "app target smoke failed: ${name} missing snapshot ${snapshot_path}" >&2
    exit 1
  fi

  if ! grep -Fq "${marker}" "${snapshot_path}"; then
    echo "app target smoke failed: ${name} marker not found: ${marker}" >&2
    echo "snapshot: ${snapshot_path}" >&2
    exit 1
  fi

  echo "app target smoke passed: ${name}"
}

run_case_with_two_followups() {
  local name="$1"
  local input="$2"
  local followup_input="$3"
  local second_followup_input="$4"
  local marker="$5"
  local snapshot_delay_ms="${6:-2500}"
  local followup_delay_ms="${7:-1000}"
  local second_followup_delay_ms="${8:-1000}"
  local case_dir="${LOG_DIR}/${name}"
  local snapshot_path="${case_dir}/snapshot.txt"
  local stdout_path="${case_dir}/stdout.log"
  local stderr_path="${case_dir}/stderr.log"

  mkdir -p "${case_dir}"
  rm -f "${snapshot_path}" "${stdout_path}" "${stderr_path}"

  MINIMAL_TERMINAL_SMOKE_INPUT="${input}" \
  MINIMAL_TERMINAL_SMOKE_FOLLOWUP_INPUT="${followup_input}" \
  MINIMAL_TERMINAL_SMOKE_SECOND_FOLLOWUP_INPUT="${second_followup_input}" \
  MINIMAL_TERMINAL_SMOKE_SNAPSHOT_PATH="${snapshot_path}" \
  MINIMAL_TERMINAL_SMOKE_INPUT_DELAY_MS=500 \
  MINIMAL_TERMINAL_SMOKE_FOLLOWUP_INPUT_DELAY_MS="${followup_delay_ms}" \
  MINIMAL_TERMINAL_SMOKE_SECOND_FOLLOWUP_INPUT_DELAY_MS="${second_followup_delay_ms}" \
  MINIMAL_TERMINAL_SMOKE_SNAPSHOT_DELAY_MS="${snapshot_delay_ms}" \
  MINIMAL_TERMINAL_SMOKE_EXIT=1 \
  "${APP_BINARY}" >"${stdout_path}" 2>"${stderr_path}" &
  local pid=$!

  local deadline=$((SECONDS + WAIT_SECONDS))
  while kill -0 "${pid}" >/dev/null 2>&1 && [[ "${SECONDS}" -lt "${deadline}" ]]; do
    sleep 0.2
  done

  if kill -0 "${pid}" >/dev/null 2>&1; then
    kill "${pid}" >/dev/null 2>&1 || true
    wait "${pid}" >/dev/null 2>&1 || true
    echo "app target smoke failed: ${name} did not exit within ${WAIT_SECONDS}s" >&2
    exit 1
  fi

  wait "${pid}"

  if [[ ! -f "${snapshot_path}" ]]; then
    echo "app target smoke failed: ${name} missing snapshot ${snapshot_path}" >&2
    exit 1
  fi

  if ! grep -Fq "${marker}" "${snapshot_path}"; then
    echo "app target smoke failed: ${name} marker not found: ${marker}" >&2
    echo "snapshot: ${snapshot_path}" >&2
    exit 1
  fi

  echo "app target smoke passed: ${name}"
}

run_case_with_three_followups() {
  local name="$1"
  local input="$2"
  local followup_input="$3"
  local second_followup_input="$4"
  local third_followup_input="$5"
  local marker="$6"
  local snapshot_delay_ms="${7:-2500}"
  local followup_delay_ms="${8:-1000}"
  local second_followup_delay_ms="${9:-1000}"
  local third_followup_delay_ms="${10:-1000}"
  local case_dir="${LOG_DIR}/${name}"
  local snapshot_path="${case_dir}/snapshot.txt"
  local stdout_path="${case_dir}/stdout.log"
  local stderr_path="${case_dir}/stderr.log"

  mkdir -p "${case_dir}"
  rm -f "${snapshot_path}" "${stdout_path}" "${stderr_path}"

  MINIMAL_TERMINAL_SMOKE_INPUT="${input}" \
  MINIMAL_TERMINAL_SMOKE_FOLLOWUP_INPUT="${followup_input}" \
  MINIMAL_TERMINAL_SMOKE_SECOND_FOLLOWUP_INPUT="${second_followup_input}" \
  MINIMAL_TERMINAL_SMOKE_THIRD_FOLLOWUP_INPUT="${third_followup_input}" \
  MINIMAL_TERMINAL_SMOKE_SNAPSHOT_PATH="${snapshot_path}" \
  MINIMAL_TERMINAL_SMOKE_INPUT_DELAY_MS=500 \
  MINIMAL_TERMINAL_SMOKE_FOLLOWUP_INPUT_DELAY_MS="${followup_delay_ms}" \
  MINIMAL_TERMINAL_SMOKE_SECOND_FOLLOWUP_INPUT_DELAY_MS="${second_followup_delay_ms}" \
  MINIMAL_TERMINAL_SMOKE_THIRD_FOLLOWUP_INPUT_DELAY_MS="${third_followup_delay_ms}" \
  MINIMAL_TERMINAL_SMOKE_SNAPSHOT_DELAY_MS="${snapshot_delay_ms}" \
  MINIMAL_TERMINAL_SMOKE_EXIT=1 \
  "${APP_BINARY}" >"${stdout_path}" 2>"${stderr_path}" &
  local pid=$!

  local deadline=$((SECONDS + WAIT_SECONDS))
  while kill -0 "${pid}" >/dev/null 2>&1 && [[ "${SECONDS}" -lt "${deadline}" ]]; do
    sleep 0.2
  done

  if kill -0 "${pid}" >/dev/null 2>&1; then
    kill "${pid}" >/dev/null 2>&1 || true
    wait "${pid}" >/dev/null 2>&1 || true
    echo "app target smoke failed: ${name} did not exit within ${WAIT_SECONDS}s" >&2
    exit 1
  fi

  wait "${pid}"

  if [[ ! -f "${snapshot_path}" ]]; then
    echo "app target smoke failed: ${name} missing snapshot ${snapshot_path}" >&2
    exit 1
  fi

  if ! grep -Fq "${marker}" "${snapshot_path}"; then
    echo "app target smoke failed: ${name} marker not found: ${marker}" >&2
    echo "snapshot: ${snapshot_path}" >&2
    exit 1
  fi

  echo "app target smoke passed: ${name}"
}

run_case_with_mouse_report() {
  local name="$1"
  local input="$2"
  local mouse_report="$3"
  local marker="$4"
  local snapshot_delay_ms="${5:-2500}"
  local mouse_report_delay_ms="${6:-1000}"
  local case_dir="${LOG_DIR}/${name}"
  local snapshot_path="${case_dir}/snapshot.txt"
  local stdout_path="${case_dir}/stdout.log"
  local stderr_path="${case_dir}/stderr.log"

  mkdir -p "${case_dir}"
  rm -f "${snapshot_path}" "${stdout_path}" "${stderr_path}"

  MINIMAL_TERMINAL_SMOKE_INPUT="${input}" \
  MINIMAL_TERMINAL_SMOKE_MOUSE_REPORT="${mouse_report}" \
  MINIMAL_TERMINAL_SMOKE_SNAPSHOT_PATH="${snapshot_path}" \
  MINIMAL_TERMINAL_SMOKE_INPUT_DELAY_MS=500 \
  MINIMAL_TERMINAL_SMOKE_MOUSE_REPORT_DELAY_MS="${mouse_report_delay_ms}" \
  MINIMAL_TERMINAL_SMOKE_SNAPSHOT_DELAY_MS="${snapshot_delay_ms}" \
  MINIMAL_TERMINAL_SMOKE_EXIT=1 \
  "${APP_BINARY}" >"${stdout_path}" 2>"${stderr_path}" &
  local pid=$!

  local deadline=$((SECONDS + WAIT_SECONDS))
  while kill -0 "${pid}" >/dev/null 2>&1 && [[ "${SECONDS}" -lt "${deadline}" ]]; do
    sleep 0.2
  done

  if kill -0 "${pid}" >/dev/null 2>&1; then
    kill "${pid}" >/dev/null 2>&1 || true
    wait "${pid}" >/dev/null 2>&1 || true
    echo "app target smoke failed: ${name} did not exit within ${WAIT_SECONDS}s" >&2
    exit 1
  fi

  wait "${pid}"

  if [[ ! -f "${snapshot_path}" ]]; then
    echo "app target smoke failed: ${name} missing snapshot ${snapshot_path}" >&2
    exit 1
  fi

  if ! grep -Fq "${marker}" "${snapshot_path}"; then
    echo "app target smoke failed: ${name} marker not found: ${marker}" >&2
    echo "snapshot: ${snapshot_path}" >&2
    exit 1
  fi

  echo "app target smoke passed: ${name}"
}

run_case_with_native_mouse_report() {
  local name="$1"
  local input="$2"
  local native_mouse_report="$3"
  local marker="$4"
  local snapshot_delay_ms="${5:-2500}"
  local case_dir="${LOG_DIR}/${name}"
  local snapshot_path="${case_dir}/snapshot.txt"
  local stdout_path="${case_dir}/stdout.log"
  local stderr_path="${case_dir}/stderr.log"

  mkdir -p "${case_dir}"
  rm -f "${snapshot_path}" "${stdout_path}" "${stderr_path}"

  MINIMAL_TERMINAL_SMOKE_INPUT="${input}" \
  MINIMAL_TERMINAL_SMOKE_NATIVE_MOUSE_REPORT="${native_mouse_report}" \
  MINIMAL_TERMINAL_SMOKE_SNAPSHOT_PATH="${snapshot_path}" \
  MINIMAL_TERMINAL_SMOKE_INPUT_DELAY_MS=500 \
  MINIMAL_TERMINAL_SMOKE_SNAPSHOT_DELAY_MS="${snapshot_delay_ms}" \
  MINIMAL_TERMINAL_SMOKE_EXIT=1 \
  "${APP_BINARY}" >"${stdout_path}" 2>"${stderr_path}" &
  local pid=$!

  local deadline=$((SECONDS + WAIT_SECONDS))
  while kill -0 "${pid}" >/dev/null 2>&1 && [[ "${SECONDS}" -lt "${deadline}" ]]; do
    sleep 0.2
  done

  if kill -0 "${pid}" >/dev/null 2>&1; then
    kill "${pid}" >/dev/null 2>&1 || true
    wait "${pid}" >/dev/null 2>&1 || true
    echo "app target smoke failed: ${name} did not exit within ${WAIT_SECONDS}s" >&2
    exit 1
  fi

  wait "${pid}"

  if [[ ! -f "${snapshot_path}" ]]; then
    echo "app target smoke failed: ${name} missing snapshot ${snapshot_path}" >&2
    exit 1
  fi

  if ! grep -Fq "${marker}" "${snapshot_path}"; then
    echo "app target smoke failed: ${name} marker not found: ${marker}" >&2
    echo "snapshot: ${snapshot_path}" >&2
    exit 1
  fi

  echo "app target smoke passed: ${name}"
}

run_case_with_native_key() {
  local name="$1"
  local input="$2"
  local native_key="$3"
  local marker="$4"
  local snapshot_delay_ms="${5:-2500}"
  local case_dir="${LOG_DIR}/${name}"
  local snapshot_path="${case_dir}/snapshot.txt"
  local stdout_path="${case_dir}/stdout.log"
  local stderr_path="${case_dir}/stderr.log"

  mkdir -p "${case_dir}"
  rm -f "${snapshot_path}" "${stdout_path}" "${stderr_path}"

  MINIMAL_TERMINAL_SMOKE_INPUT="${input}" \
  MINIMAL_TERMINAL_SMOKE_NATIVE_KEY="${native_key}" \
  MINIMAL_TERMINAL_SMOKE_SNAPSHOT_PATH="${snapshot_path}" \
  MINIMAL_TERMINAL_SMOKE_INPUT_DELAY_MS=500 \
  MINIMAL_TERMINAL_SMOKE_SNAPSHOT_DELAY_MS="${snapshot_delay_ms}" \
  MINIMAL_TERMINAL_SMOKE_EXIT=1 \
  "${APP_BINARY}" >"${stdout_path}" 2>"${stderr_path}" &
  local pid=$!

  local deadline=$((SECONDS + WAIT_SECONDS))
  while kill -0 "${pid}" >/dev/null 2>&1 && [[ "${SECONDS}" -lt "${deadline}" ]]; do
    sleep 0.2
  done

  if kill -0 "${pid}" >/dev/null 2>&1; then
    kill "${pid}" >/dev/null 2>&1 || true
    wait "${pid}" >/dev/null 2>&1 || true
    echo "app target smoke failed: ${name} did not exit within ${WAIT_SECONDS}s" >&2
    exit 1
  fi

  wait "${pid}"

  if [[ ! -f "${snapshot_path}" ]]; then
    echo "app target smoke failed: ${name} missing snapshot ${snapshot_path}" >&2
    exit 1
  fi

  if ! grep -Fq "${marker}" "${snapshot_path}"; then
    echo "app target smoke failed: ${name} marker not found: ${marker}" >&2
    echo "snapshot: ${snapshot_path}" >&2
    exit 1
  fi

  echo "app target smoke passed: ${name}"
}

run_case_with_native_key_required_markers() {
  local name="$1"
  local input="$2"
  local native_key="$3"
  local snapshot_delay_ms="$4"
  shift 4
  local case_dir="${LOG_DIR}/${name}"
  local snapshot_path="${case_dir}/snapshot.txt"
  local stdout_path="${case_dir}/stdout.log"
  local stderr_path="${case_dir}/stderr.log"

  mkdir -p "${case_dir}"
  rm -f "${snapshot_path}" "${stdout_path}" "${stderr_path}"

  MINIMAL_TERMINAL_SMOKE_INPUT="${input}" \
  MINIMAL_TERMINAL_SMOKE_NATIVE_KEY="${native_key}" \
  MINIMAL_TERMINAL_SMOKE_SNAPSHOT_PATH="${snapshot_path}" \
  MINIMAL_TERMINAL_SMOKE_INPUT_DELAY_MS=500 \
  MINIMAL_TERMINAL_SMOKE_SNAPSHOT_DELAY_MS="${snapshot_delay_ms}" \
  MINIMAL_TERMINAL_SMOKE_EXIT=1 \
  "${APP_BINARY}" >"${stdout_path}" 2>"${stderr_path}" &
  local pid=$!

  local deadline=$((SECONDS + WAIT_SECONDS))
  while kill -0 "${pid}" >/dev/null 2>&1 && [[ "${SECONDS}" -lt "${deadline}" ]]; do
    sleep 0.2
  done

  if kill -0 "${pid}" >/dev/null 2>&1; then
    kill "${pid}" >/dev/null 2>&1 || true
    wait "${pid}" >/dev/null 2>&1 || true
    echo "app target smoke failed: ${name} did not exit within ${WAIT_SECONDS}s" >&2
    exit 1
  fi

  wait "${pid}"

  if [[ ! -f "${snapshot_path}" ]]; then
    echo "app target smoke failed: ${name} missing snapshot ${snapshot_path}" >&2
    exit 1
  fi

  local marker
  for marker in "$@"; do
    if ! grep -Fq "${marker}" "${snapshot_path}"; then
      echo "app target smoke failed: ${name} marker not found: ${marker}" >&2
      echo "snapshot: ${snapshot_path}" >&2
      exit 1
    fi
  done

  echo "app target smoke passed: ${name}"
}

ran=0

run_case \
  "shell-home" \
  $'printf "shell-home:%s\\n" "$PWD"\n' \
  "shell-home:${HOME}" \
  1500
run_case \
  "shell-exit-notice" \
  $'exit\n' \
  "[Shell process exited]" \
  1500
run_case_with_native_window_resize \
  "native-window-resize" \
  $'sleep 2; printf "native-window-resize-after:%s\\n" "$(stty size)"\n' \
  "24x80" \
  "native-window-resize-after:24 80" \
  3500
run_case_with_native_key \
  "native-control-f5-key" \
  $'ready="native-key"; ready="${ready}-ready"; stty raw -echo; printf "\\n%s\\n" "$ready"; bytes="$(dd bs=1 count=7 2>/dev/null | od -An -tx1 | tr -d " \\n")"; stty sane; printf "\\nnative-control-f5-key:%s\\n" "$bytes"\n' \
  "control-f5" \
  "native-control-f5-key:1b5b31353b357e" \
  1500
run_case_with_native_key \
  "native-shift-option-up-key" \
  $'ready="native-key"; ready="${ready}-ready"; stty raw -echo; printf "\\n%s\\n" "$ready"; bytes="$(dd bs=1 count=6 2>/dev/null | od -An -tx1 | tr -d " \\n")"; stty sane; printf "\\nnative-shift-option-up-key:%s\\n" "$bytes"\n' \
  "shift-option-up" \
  "native-shift-option-up-key:1b5b313b3441" \
  1500
run_case_with_native_key \
  "native-control-option-right-key" \
  $'ready="native-key"; ready="${ready}-ready"; stty raw -echo; printf "\\n%s\\n" "$ready"; bytes="$(dd bs=1 count=6 2>/dev/null | od -An -tx1 | tr -d " \\n")"; stty sane; printf "\\nnative-control-option-right-key:%s\\n" "$bytes"\n' \
  "control-option-right" \
  "native-control-option-right-key:1b5b313b3743" \
  1500
run_case_with_native_key_required_markers \
  "native-up-modifier-matrix-key" \
  $'ready="native-key"; ready="${ready}-ready"; stty raw -echo; printf "\\n%s\\n" "$ready"; for modifier in 2 3 4 5 6 7 8; do bytes="$(dd bs=1 count=6 2>/dev/null | od -An -tx1 | tr -d " \\n")"; printf "\\nnative-up-modifier-matrix-key-${modifier}:%s\\n" "$bytes"; done; stty sane\n' \
  "shift-up,option-up,shift-option-up,control-up,shift-control-up,control-option-up,shift-control-option-up" \
  1500 \
  "native-up-modifier-matrix-key-2:1b5b313b3241" \
  "native-up-modifier-matrix-key-3:1b5b313b3341" \
  "native-up-modifier-matrix-key-4:1b5b313b3441" \
  "native-up-modifier-matrix-key-5:1b5b313b3541" \
  "native-up-modifier-matrix-key-6:1b5b313b3641" \
  "native-up-modifier-matrix-key-7:1b5b313b3741" \
  "native-up-modifier-matrix-key-8:1b5b313b3841"
run_case_with_native_key_required_markers \
  "native-f5-modifier-matrix-key" \
  $'ready="native-key"; ready="${ready}-ready"; stty raw -echo; printf "\\n%s\\n" "$ready"; for modifier in 2 3 4 5 6 7 8; do bytes="$(dd bs=1 count=7 2>/dev/null | od -An -tx1 | tr -d " \\n")"; printf "\\nnative-f5-modifier-matrix-key-${modifier}:%s\\n" "$bytes"; done; stty sane\n' \
  "shift-f5,option-f5,shift-option-f5,control-f5,shift-control-f5,control-option-f5,shift-control-option-f5" \
  1500 \
  "native-f5-modifier-matrix-key-2:1b5b31353b327e" \
  "native-f5-modifier-matrix-key-3:1b5b31353b337e" \
  "native-f5-modifier-matrix-key-4:1b5b31353b347e" \
  "native-f5-modifier-matrix-key-5:1b5b31353b357e" \
  "native-f5-modifier-matrix-key-6:1b5b31353b367e" \
  "native-f5-modifier-matrix-key-7:1b5b31353b377e" \
  "native-f5-modifier-matrix-key-8:1b5b31353b387e"
run_case_with_native_key_required_markers \
  "native-shift-function-f1-f4-key" \
  $'ready="native-key"; ready="${ready}-ready"; stty raw -echo; printf "\\n%s\\n" "$ready"; for key in f1 f2 f3 f4; do bytes="$(dd bs=1 count=6 2>/dev/null | od -An -tx1 | tr -d " \\n")"; printf "\\nnative-shift-function-f1-f4-key-${key}:%s\\n" "$bytes"; done; stty sane\n' \
  "shift-f1,shift-f2,shift-f3,shift-f4" \
  1500 \
  "native-shift-function-f1-f4-key-f1:1b5b313b3250" \
  "native-shift-function-f1-f4-key-f2:1b5b313b3251" \
  "native-shift-function-f1-f4-key-f3:1b5b313b3252" \
  "native-shift-function-f1-f4-key-f4:1b5b313b3253"
run_case_with_native_key_required_markers \
  "native-shift-function-f5-f12-key" \
  $'ready="native-key"; ready="${ready}-ready"; stty raw -echo; printf "\\n%s\\n" "$ready"; for key in f5 f6 f7 f8 f9 f10 f11 f12; do bytes="$(dd bs=1 count=7 2>/dev/null | od -An -tx1 | tr -d " \\n")"; printf "\\nnative-shift-function-f5-f12-key-${key}:%s\\n" "$bytes"; done; stty sane\n' \
  "shift-f5,shift-f6,shift-f7,shift-f8,shift-f9,shift-f10,shift-f11,shift-f12" \
  1500 \
  "native-shift-function-f5-f12-key-f5:1b5b31353b327e" \
  "native-shift-function-f5-f12-key-f6:1b5b31373b327e" \
  "native-shift-function-f5-f12-key-f7:1b5b31383b327e" \
  "native-shift-function-f5-f12-key-f8:1b5b31393b327e" \
  "native-shift-function-f5-f12-key-f9:1b5b32303b327e" \
  "native-shift-function-f5-f12-key-f10:1b5b32313b327e" \
  "native-shift-function-f5-f12-key-f11:1b5b32333b327e" \
  "native-shift-function-f5-f12-key-f12:1b5b32343b327e"
run_case_with_native_key_required_markers \
  "native-control-function-f1-f4-key" \
  $'ready="native-key"; ready="${ready}-ready"; stty raw -echo; printf "\\n%s\\n" "$ready"; for key in f1 f2 f3 f4; do bytes="$(dd bs=1 count=6 2>/dev/null | od -An -tx1 | tr -d " \\n")"; printf "\\nnative-control-function-f1-f4-key-${key}:%s\\n" "$bytes"; done; stty sane\n' \
  "control-f1,control-f2,control-f3,control-f4" \
  1500 \
  "native-control-function-f1-f4-key-f1:1b5b313b3550" \
  "native-control-function-f1-f4-key-f2:1b5b313b3551" \
  "native-control-function-f1-f4-key-f3:1b5b313b3552" \
  "native-control-function-f1-f4-key-f4:1b5b313b3553"
run_case_with_native_key_required_markers \
  "native-control-function-f5-f12-key" \
  $'ready="native-key"; ready="${ready}-ready"; stty raw -echo; printf "\\n%s\\n" "$ready"; for key in f5 f6 f7 f8 f9 f10 f11 f12; do bytes="$(dd bs=1 count=7 2>/dev/null | od -An -tx1 | tr -d " \\n")"; printf "\\nnative-control-function-f5-f12-key-${key}:%s\\n" "$bytes"; done; stty sane\n' \
  "control-f5,control-f6,control-f7,control-f8,control-f9,control-f10,control-f11,control-f12" \
  1500 \
  "native-control-function-f5-f12-key-f5:1b5b31353b357e" \
  "native-control-function-f5-f12-key-f6:1b5b31373b357e" \
  "native-control-function-f5-f12-key-f7:1b5b31383b357e" \
  "native-control-function-f5-f12-key-f8:1b5b31393b357e" \
  "native-control-function-f5-f12-key-f9:1b5b32303b357e" \
  "native-control-function-f5-f12-key-f10:1b5b32313b357e" \
  "native-control-function-f5-f12-key-f11:1b5b32333b357e" \
  "native-control-function-f5-f12-key-f12:1b5b32343b357e"
run_case_with_native_key_required_markers \
  "native-option-function-f1-f4-key" \
  $'ready="native-key"; ready="${ready}-ready"; stty raw -echo; printf "\\n%s\\n" "$ready"; for key in f1 f2 f3 f4; do bytes="$(dd bs=1 count=6 2>/dev/null | od -An -tx1 | tr -d " \\n")"; printf "\\nnative-option-function-f1-f4-key-${key}:%s\\n" "$bytes"; done; stty sane\n' \
  "option-f1,option-f2,option-f3,option-f4" \
  1500 \
  "native-option-function-f1-f4-key-f1:1b5b313b3350" \
  "native-option-function-f1-f4-key-f2:1b5b313b3351" \
  "native-option-function-f1-f4-key-f3:1b5b313b3352" \
  "native-option-function-f1-f4-key-f4:1b5b313b3353"
run_case_with_native_key_required_markers \
  "native-option-function-f5-f12-key" \
  $'ready="native-key"; ready="${ready}-ready"; stty raw -echo; printf "\\n%s\\n" "$ready"; for key in f5 f6 f7 f8 f9 f10 f11 f12; do bytes="$(dd bs=1 count=7 2>/dev/null | od -An -tx1 | tr -d " \\n")"; printf "\\nnative-option-function-f5-f12-key-${key}:%s\\n" "$bytes"; done; stty sane\n' \
  "option-f5,option-f6,option-f7,option-f8,option-f9,option-f10,option-f11,option-f12" \
  1500 \
  "native-option-function-f5-f12-key-f5:1b5b31353b337e" \
  "native-option-function-f5-f12-key-f6:1b5b31373b337e" \
  "native-option-function-f5-f12-key-f7:1b5b31383b337e" \
  "native-option-function-f5-f12-key-f8:1b5b31393b337e" \
  "native-option-function-f5-f12-key-f9:1b5b32303b337e" \
  "native-option-function-f5-f12-key-f10:1b5b32313b337e" \
  "native-option-function-f5-f12-key-f11:1b5b32333b337e" \
  "native-option-function-f5-f12-key-f12:1b5b32343b337e"
run_case_with_native_key_required_markers \
  "native-shift-option-function-f1-f4-key" \
  $'ready="native-key"; ready="${ready}-ready"; stty raw -echo; printf "\\n%s\\n" "$ready"; for key in f1 f2 f3 f4; do bytes="$(dd bs=1 count=6 2>/dev/null | od -An -tx1 | tr -d " \\n")"; printf "\\nnative-shift-option-function-f1-f4-key-${key}:%s\\n" "$bytes"; done; stty sane\n' \
  "shift-option-f1,shift-option-f2,shift-option-f3,shift-option-f4" \
  1500 \
  "native-shift-option-function-f1-f4-key-f1:1b5b313b3450" \
  "native-shift-option-function-f1-f4-key-f2:1b5b313b3451" \
  "native-shift-option-function-f1-f4-key-f3:1b5b313b3452" \
  "native-shift-option-function-f1-f4-key-f4:1b5b313b3453"
run_case_with_native_key_required_markers \
  "native-shift-option-function-f5-f12-key" \
  $'ready="native-key"; ready="${ready}-ready"; stty raw -echo; printf "\\n%s\\n" "$ready"; for key in f5 f6 f7 f8 f9 f10 f11 f12; do bytes="$(dd bs=1 count=7 2>/dev/null | od -An -tx1 | tr -d " \\n")"; printf "\\nnative-shift-option-function-f5-f12-key-${key}:%s\\n" "$bytes"; done; stty sane\n' \
  "shift-option-f5,shift-option-f6,shift-option-f7,shift-option-f8,shift-option-f9,shift-option-f10,shift-option-f11,shift-option-f12" \
  1500 \
  "native-shift-option-function-f5-f12-key-f5:1b5b31353b347e" \
  "native-shift-option-function-f5-f12-key-f6:1b5b31373b347e" \
  "native-shift-option-function-f5-f12-key-f7:1b5b31383b347e" \
  "native-shift-option-function-f5-f12-key-f8:1b5b31393b347e" \
  "native-shift-option-function-f5-f12-key-f9:1b5b32303b347e" \
  "native-shift-option-function-f5-f12-key-f10:1b5b32313b347e" \
  "native-shift-option-function-f5-f12-key-f11:1b5b32333b347e" \
  "native-shift-option-function-f5-f12-key-f12:1b5b32343b347e"
run_case_with_native_key_required_markers \
  "native-shift-control-function-f1-f4-key" \
  $'ready="native-key"; ready="${ready}-ready"; stty raw -echo; printf "\\n%s\\n" "$ready"; for key in f1 f2 f3 f4; do bytes="$(dd bs=1 count=6 2>/dev/null | od -An -tx1 | tr -d " \\n")"; printf "\\nnative-shift-control-function-f1-f4-key-${key}:%s\\n" "$bytes"; done; stty sane\n' \
  "shift-control-f1,shift-control-f2,shift-control-f3,shift-control-f4" \
  1500 \
  "native-shift-control-function-f1-f4-key-f1:1b5b313b3650" \
  "native-shift-control-function-f1-f4-key-f2:1b5b313b3651" \
  "native-shift-control-function-f1-f4-key-f3:1b5b313b3652" \
  "native-shift-control-function-f1-f4-key-f4:1b5b313b3653"
run_case_with_native_key_required_markers \
  "native-shift-control-function-f5-f12-key" \
  $'ready="native-key"; ready="${ready}-ready"; stty raw -echo; printf "\\n%s\\n" "$ready"; for key in f5 f6 f7 f8 f9 f10 f11 f12; do bytes="$(dd bs=1 count=7 2>/dev/null | od -An -tx1 | tr -d " \\n")"; printf "\\nnative-shift-control-function-f5-f12-key-${key}:%s\\n" "$bytes"; done; stty sane\n' \
  "shift-control-f5,shift-control-f6,shift-control-f7,shift-control-f8,shift-control-f9,shift-control-f10,shift-control-f11,shift-control-f12" \
  1500 \
  "native-shift-control-function-f5-f12-key-f5:1b5b31353b367e" \
  "native-shift-control-function-f5-f12-key-f6:1b5b31373b367e" \
  "native-shift-control-function-f5-f12-key-f7:1b5b31383b367e" \
  "native-shift-control-function-f5-f12-key-f8:1b5b31393b367e" \
  "native-shift-control-function-f5-f12-key-f9:1b5b32303b367e" \
  "native-shift-control-function-f5-f12-key-f10:1b5b32313b367e" \
  "native-shift-control-function-f5-f12-key-f11:1b5b32333b367e" \
  "native-shift-control-function-f5-f12-key-f12:1b5b32343b367e"
run_case_with_native_key_required_markers \
  "native-control-option-function-f1-f4-key" \
  $'ready="native-key"; ready="${ready}-ready"; stty raw -echo; printf "\\n%s\\n" "$ready"; for key in f1 f2 f3 f4; do bytes="$(dd bs=1 count=6 2>/dev/null | od -An -tx1 | tr -d " \\n")"; printf "\\nnative-control-option-function-f1-f4-key-${key}:%s\\n" "$bytes"; done; stty sane\n' \
  "control-option-f1,control-option-f2,control-option-f3,control-option-f4" \
  1500 \
  "native-control-option-function-f1-f4-key-f1:1b5b313b3750" \
  "native-control-option-function-f1-f4-key-f2:1b5b313b3751" \
  "native-control-option-function-f1-f4-key-f3:1b5b313b3752" \
  "native-control-option-function-f1-f4-key-f4:1b5b313b3753"
run_case_with_native_key_required_markers \
  "native-control-option-function-f5-f12-key" \
  $'ready="native-key"; ready="${ready}-ready"; stty raw -echo; printf "\\n%s\\n" "$ready"; for key in f5 f6 f7 f8 f9 f10 f11 f12; do bytes="$(dd bs=1 count=7 2>/dev/null | od -An -tx1 | tr -d " \\n")"; printf "\\nnative-control-option-function-f5-f12-key-${key}:%s\\n" "$bytes"; done; stty sane\n' \
  "control-option-f5,control-option-f6,control-option-f7,control-option-f8,control-option-f9,control-option-f10,control-option-f11,control-option-f12" \
  1500 \
  "native-control-option-function-f5-f12-key-f5:1b5b31353b377e" \
  "native-control-option-function-f5-f12-key-f6:1b5b31373b377e" \
  "native-control-option-function-f5-f12-key-f7:1b5b31383b377e" \
  "native-control-option-function-f5-f12-key-f8:1b5b31393b377e" \
  "native-control-option-function-f5-f12-key-f9:1b5b32303b377e" \
  "native-control-option-function-f5-f12-key-f10:1b5b32313b377e" \
  "native-control-option-function-f5-f12-key-f11:1b5b32333b377e" \
  "native-control-option-function-f5-f12-key-f12:1b5b32343b377e"
run_case_with_native_key_required_markers \
  "native-shift-control-option-function-f1-f4-key" \
  $'ready="native-key"; ready="${ready}-ready"; stty raw -echo; printf "\\n%s\\n" "$ready"; for key in f1 f2 f3 f4; do bytes="$(dd bs=1 count=6 2>/dev/null | od -An -tx1 | tr -d " \\n")"; printf "\\nnative-shift-control-option-function-f1-f4-key-${key}:%s\\n" "$bytes"; done; stty sane\n' \
  "shift-control-option-f1,shift-control-option-f2,shift-control-option-f3,shift-control-option-f4" \
  1500 \
  "native-shift-control-option-function-f1-f4-key-f1:1b5b313b3850" \
  "native-shift-control-option-function-f1-f4-key-f2:1b5b313b3851" \
  "native-shift-control-option-function-f1-f4-key-f3:1b5b313b3852" \
  "native-shift-control-option-function-f1-f4-key-f4:1b5b313b3853"
run_case_with_native_key_required_markers \
  "native-shift-control-option-function-f5-f12-key" \
  $'ready="native-key"; ready="${ready}-ready"; stty raw -echo; printf "\\n%s\\n" "$ready"; for key in f5 f6 f7 f8 f9 f10 f11 f12; do bytes="$(dd bs=1 count=7 2>/dev/null | od -An -tx1 | tr -d " \\n")"; printf "\\nnative-shift-control-option-function-f5-f12-key-${key}:%s\\n" "$bytes"; done; stty sane\n' \
  "shift-control-option-f5,shift-control-option-f6,shift-control-option-f7,shift-control-option-f8,shift-control-option-f9,shift-control-option-f10,shift-control-option-f11,shift-control-option-f12" \
  1500 \
  "native-shift-control-option-function-f5-f12-key-f5:1b5b31353b387e" \
  "native-shift-control-option-function-f5-f12-key-f6:1b5b31373b387e" \
  "native-shift-control-option-function-f5-f12-key-f7:1b5b31383b387e" \
  "native-shift-control-option-function-f5-f12-key-f8:1b5b31393b387e" \
  "native-shift-control-option-function-f5-f12-key-f9:1b5b32303b387e" \
  "native-shift-control-option-function-f5-f12-key-f10:1b5b32313b387e" \
  "native-shift-control-option-function-f5-f12-key-f11:1b5b32333b387e" \
  "native-shift-control-option-function-f5-f12-key-f12:1b5b32343b387e"
run_case_with_native_key_required_markers \
  "native-control-option-navigation-family-key" \
  $'ready="native-key"; ready="${ready}-ready"; stty raw -echo; printf "\\n%s\\n" "$ready"; for key in up down right left home end page-up page-down delete; do bytes="$(dd bs=1 count=6 2>/dev/null | od -An -tx1 | tr -d " \\n")"; printf "\\nnative-control-option-navigation-family-key-${key}:%s\\n" "$bytes"; done; stty sane\n' \
  "control-option-up,control-option-down,control-option-right,control-option-left,control-option-home,control-option-end,control-option-page-up,control-option-page-down,control-option-delete" \
  1500 \
  "native-control-option-navigation-family-key-up:1b5b313b3741" \
  "native-control-option-navigation-family-key-down:1b5b313b3742" \
  "native-control-option-navigation-family-key-right:1b5b313b3743" \
  "native-control-option-navigation-family-key-left:1b5b313b3744" \
  "native-control-option-navigation-family-key-home:1b5b313b3748" \
  "native-control-option-navigation-family-key-end:1b5b313b3746" \
  "native-control-option-navigation-family-key-page-up:1b5b353b377e" \
  "native-control-option-navigation-family-key-page-down:1b5b363b377e" \
  "native-control-option-navigation-family-key-delete:1b5b333b377e"
run_case_with_native_key_required_markers \
  "native-shift-control-navigation-family-key" \
  $'ready="native-key"; ready="${ready}-ready"; stty raw -echo; printf "\\n%s\\n" "$ready"; for key in up down right left home end page-up page-down delete; do bytes="$(dd bs=1 count=6 2>/dev/null | od -An -tx1 | tr -d " \\n")"; printf "\\nnative-shift-control-navigation-family-key-${key}:%s\\n" "$bytes"; done; stty sane\n' \
  "shift-control-up,shift-control-down,shift-control-right,shift-control-left,shift-control-home,shift-control-end,shift-control-page-up,shift-control-page-down,shift-control-delete" \
  1500 \
  "native-shift-control-navigation-family-key-up:1b5b313b3641" \
  "native-shift-control-navigation-family-key-down:1b5b313b3642" \
  "native-shift-control-navigation-family-key-right:1b5b313b3643" \
  "native-shift-control-navigation-family-key-left:1b5b313b3644" \
  "native-shift-control-navigation-family-key-home:1b5b313b3648" \
  "native-shift-control-navigation-family-key-end:1b5b313b3646" \
  "native-shift-control-navigation-family-key-page-up:1b5b353b367e" \
  "native-shift-control-navigation-family-key-page-down:1b5b363b367e" \
  "native-shift-control-navigation-family-key-delete:1b5b333b367e"
run_case_with_native_key_required_markers \
  "native-shift-option-navigation-family-key" \
  $'ready="native-key"; ready="${ready}-ready"; stty raw -echo; printf "\\n%s\\n" "$ready"; for key in up down right left home end page-up page-down delete; do bytes="$(dd bs=1 count=6 2>/dev/null | od -An -tx1 | tr -d " \\n")"; printf "\\nnative-shift-option-navigation-family-key-${key}:%s\\n" "$bytes"; done; stty sane\n' \
  "shift-option-up,shift-option-down,shift-option-right,shift-option-left,shift-option-home,shift-option-end,shift-option-page-up,shift-option-page-down,shift-option-delete" \
  1500 \
  "native-shift-option-navigation-family-key-up:1b5b313b3441" \
  "native-shift-option-navigation-family-key-down:1b5b313b3442" \
  "native-shift-option-navigation-family-key-right:1b5b313b3443" \
  "native-shift-option-navigation-family-key-left:1b5b313b3444" \
  "native-shift-option-navigation-family-key-home:1b5b313b3448" \
  "native-shift-option-navigation-family-key-end:1b5b313b3446" \
  "native-shift-option-navigation-family-key-page-up:1b5b353b347e" \
  "native-shift-option-navigation-family-key-page-down:1b5b363b347e" \
  "native-shift-option-navigation-family-key-delete:1b5b333b347e"
run_case_with_native_key_required_markers \
  "native-shift-control-option-navigation-family-key" \
  $'ready="native-key"; ready="${ready}-ready"; stty raw -echo; printf "\\n%s\\n" "$ready"; for key in up down right left home end page-up page-down delete; do bytes="$(dd bs=1 count=6 2>/dev/null | od -An -tx1 | tr -d " \\n")"; printf "\\nnative-shift-control-option-navigation-family-key-${key}:%s\\n" "$bytes"; done; stty sane\n' \
  "shift-control-option-up,shift-control-option-down,shift-control-option-right,shift-control-option-left,shift-control-option-home,shift-control-option-end,shift-control-option-page-up,shift-control-option-page-down,shift-control-option-delete" \
  1500 \
  "native-shift-control-option-navigation-family-key-up:1b5b313b3841" \
  "native-shift-control-option-navigation-family-key-down:1b5b313b3842" \
  "native-shift-control-option-navigation-family-key-right:1b5b313b3843" \
  "native-shift-control-option-navigation-family-key-left:1b5b313b3844" \
  "native-shift-control-option-navigation-family-key-home:1b5b313b3848" \
  "native-shift-control-option-navigation-family-key-end:1b5b313b3846" \
  "native-shift-control-option-navigation-family-key-page-up:1b5b353b387e" \
  "native-shift-control-option-navigation-family-key-page-down:1b5b363b387e" \
  "native-shift-control-option-navigation-family-key-delete:1b5b333b387e"
run_case_with_native_key_required_markers \
  "native-shift-navigation-family-key" \
  $'ready="native-key"; ready="${ready}-ready"; stty raw -echo; printf "\\n%s\\n" "$ready"; for key in up down right left home end page-up page-down delete; do bytes="$(dd bs=1 count=6 2>/dev/null | od -An -tx1 | tr -d " \\n")"; printf "\\nnative-shift-navigation-family-key-${key}:%s\\n" "$bytes"; done; stty sane\n' \
  "shift-up,shift-down,shift-right,shift-left,shift-home,shift-end,shift-page-up,shift-page-down,shift-delete" \
  1500 \
  "native-shift-navigation-family-key-up:1b5b313b3241" \
  "native-shift-navigation-family-key-down:1b5b313b3242" \
  "native-shift-navigation-family-key-right:1b5b313b3243" \
  "native-shift-navigation-family-key-left:1b5b313b3244" \
  "native-shift-navigation-family-key-home:1b5b313b3248" \
  "native-shift-navigation-family-key-end:1b5b313b3246" \
  "native-shift-navigation-family-key-page-up:1b5b353b327e" \
  "native-shift-navigation-family-key-page-down:1b5b363b327e" \
  "native-shift-navigation-family-key-delete:1b5b333b327e"
run_case_with_native_key_required_markers \
  "native-control-navigation-family-key" \
  $'ready="native-key"; ready="${ready}-ready"; stty raw -echo; printf "\\n%s\\n" "$ready"; for key in up down right left home end page-up page-down delete; do bytes="$(dd bs=1 count=6 2>/dev/null | od -An -tx1 | tr -d " \\n")"; printf "\\nnative-control-navigation-family-key-${key}:%s\\n" "$bytes"; done; stty sane\n' \
  "control-up,control-down,control-right,control-left,control-home,control-end,control-page-up,control-page-down,control-delete" \
  1500 \
  "native-control-navigation-family-key-up:1b5b313b3541" \
  "native-control-navigation-family-key-down:1b5b313b3542" \
  "native-control-navigation-family-key-right:1b5b313b3543" \
  "native-control-navigation-family-key-left:1b5b313b3544" \
  "native-control-navigation-family-key-home:1b5b313b3548" \
  "native-control-navigation-family-key-end:1b5b313b3546" \
  "native-control-navigation-family-key-page-up:1b5b353b357e" \
  "native-control-navigation-family-key-page-down:1b5b363b357e" \
  "native-control-navigation-family-key-delete:1b5b333b357e"
run_case_with_native_key_required_markers \
  "native-option-non-word-navigation-key" \
  $'ready="native-key"; ready="${ready}-ready"; stty raw -echo; printf "\\n%s\\n" "$ready"; for key in up down home end page-up page-down delete; do bytes="$(dd bs=1 count=6 2>/dev/null | od -An -tx1 | tr -d " \\n")"; printf "\\nnative-option-non-word-navigation-key-${key}:%s\\n" "$bytes"; done; stty sane\n' \
  "option-up,option-down,option-home,option-end,option-page-up,option-page-down,option-delete" \
  1500 \
  "native-option-non-word-navigation-key-up:1b5b313b3341" \
  "native-option-non-word-navigation-key-down:1b5b313b3342" \
  "native-option-non-word-navigation-key-home:1b5b313b3348" \
  "native-option-non-word-navigation-key-end:1b5b313b3346" \
  "native-option-non-word-navigation-key-page-up:1b5b353b337e" \
  "native-option-non-word-navigation-key-page-down:1b5b363b337e" \
  "native-option-non-word-navigation-key-delete:1b5b333b337e"
ran=1

run_case_with_mouse_report \
  "mouse-sgr-report" \
  $'stty raw -echo; printf "\\033[?1000h\\033[?1006h"; bytes="$(dd bs=1 count=9 2>/dev/null | od -An -tx1 | tr -d " \\n")"; stty sane; printf "\\nmouse-sgr-report:%s\\n" "$bytes"\n' \
  "left-press" \
  "mouse-sgr-report:1b5b3c303b333b324d" \
  1500 \
  1000
run_case_with_native_mouse_report \
  "native-mouse-sgr-report" \
  $'stty raw -echo; printf "\\033[?1000h\\033[?1006h"; bytes="$(dd bs=1 count=9 2>/dev/null | od -An -tx1 | tr -d " \\n")"; stty sane; printf "\\nnative-mouse-sgr-report:%s\\n" "$bytes"\n' \
  "left-press" \
  "native-mouse-sgr-report:1b5b3c30" \
  1500
ran=1

if command -v fzf >/dev/null 2>&1; then
  fzf_path="$(command -v fzf)"
  run_case "fzf-filter" "printf \"alpha\\nbeta\\n\" | ${fzf_path} --filter alpha"$'\n' "alpha"
  run_case_with_followup \
    "fzf-interactive" \
    "printf \"alpha\\nbeta\\n\" | ${fzf_path}"$'\n' \
    "b" \
    "▌ beta" \
    1500 \
    1000
  run_case \
    "fzf-preview" \
    "printf \"alpha\\nbeta\\n\" | ${fzf_path} --query beta --preview \"printf preview:{}\""$'\n' \
    "preview:beta" \
    5000
  run_case_with_two_followups \
    "fzf-select" \
    "selected=\"\$(printf \"alpha\\nbeta\\n\" | ${fzf_path})\"; printf \"fzf-select:%s\\n\" \"\$selected\""$'\n' \
    "b" \
    $'\r' \
    "fzf-select:beta" \
    1500 \
    1000 \
    700
  run_case_with_two_followups \
    "fzf-multi-select" \
    "selected=\"\$(printf \"alpha\\nbeta\\n\" | ${fzf_path} -m)\"; printf \"fzf-multi:%s\\n\" \"\$selected\""$'\n' \
    "b" \
    $'\t\r' \
    "fzf-multi:beta" \
    1500 \
    1000 \
    700
  run_case_with_three_followups \
    "fzf-shell-ctrl-t" \
    "tmpdir=\"\$(mktemp -d /tmp/minimal-terminal-fzf-shell.XXXXXX)\"; touch \"\$tmpdir/phase-fzf-shell-target\"; cd \"\$tmpdir\"; source /opt/homebrew/opt/fzf/shell/key-bindings.zsh; printf \"fzf-shell-ready\\n\""$'\n' \
    $'printf "fzf-shell:%s\\n" \024' \
    "phase-fzf-shell-target"$'\r' \
    $'\r' \
    "fzf-shell:phase-fzf-shell-target" \
    2200 \
    900 \
    1200 \
    700
  run_case_with_three_followups \
    "fzf-shell-alt-c" \
    "tmpdir=\"\$(mktemp -d /tmp/minimal-terminal-fzf-alt-c.XXXXXX)\"; mkdir -p \"\$tmpdir/phase-fzf-alt-c-target\"; cd \"\$tmpdir\"; source /opt/homebrew/opt/fzf/shell/key-bindings.zsh; printf \"fzf-alt-c-ready\\n\""$'\n' \
    $'\ec' \
    "phase-fzf-alt-c-target"$'\r' \
    $'printf "fzf-alt-c:%s\\n" "$(basename "$PWD")"\r' \
    "fzf-alt-c:phase-fzf-alt-c-target" \
    2200 \
    900 \
    1200 \
    700
  run_case_with_three_followups \
    "fzf-shell-ctrl-r" \
    "print -s 'printf \"fzf-history-ok\\\\n\"'; source /opt/homebrew/opt/fzf/shell/key-bindings.zsh; printf \"fzf-history-ready\\n\""$'\n' \
    $'\022' \
    "fzf-history-ok"$'\r' \
    $'\r' \
    "fzf-history-ok" \
    2200 \
    900 \
    1200 \
    700
  ran=1
else
  echo "app target smoke skipped: fzf not found"
fi

if command -v vim >/dev/null 2>&1; then
  vim_path="$(command -v vim)"
  run_case_with_followup \
    "vim-edit-write-quit" \
    "tmp=\"/tmp/minimal-terminal-vim-smoke-\$\$.txt\"; rm -f \"\$tmp\"; ${vim_path} --clean -Nu NONE -n \"\$tmp\"; printf \"vim-workflow-ok:%s\\n\" \"\$(cat \"\$tmp\")\"; rm -f \"\$tmp\""$'\n' \
    $'ihello from vim\e:wq\r' \
    "vim-workflow-ok:hello from vim" \
    2500 \
    1200
  run_case_with_mouse_report \
    "vim-mouse-left-press" \
    "tmp=\"/tmp/minimal-terminal-vim-mouse-smoke.txt\"; rm -f \"\$tmp\"; ${vim_path} --clean -Nu NONE -n -c 'set mouse=a ttymouse=sgr' -c 'nnoremap <LeftMouse> :call writefile([\"vim-mouse-ok\"], \"/tmp/minimal-terminal-vim-mouse-smoke.txt\")<CR>:qa!<CR>'; cat \"\$tmp\" 2>/dev/null; rm -f \"\$tmp\""$'\n' \
    "left-press" \
    "vim-mouse-ok" \
    2500 \
    1800
  run_case_with_followup \
    "vim-window-split" \
    "tmp=\"/tmp/minimal-terminal-vim-split-smoke.txt\"; rm -f \"\$tmp\"; ${vim_path} --clean -Nu NONE -n; printf \"vim-split-count:%s\\n\" \"\$(cat \"\$tmp\")\"; rm -f \"\$tmp\""$'\n' \
    $'\027s:call writefile([string(winnr(\047$\047))], "/tmp/minimal-terminal-vim-split-smoke.txt")\r:qall!\r' \
    "vim-split-count:2" \
    3200 \
    1400
  run_case_with_resize_followup \
    "vim-resize-redraw" \
    "tmp=\"/tmp/minimal-terminal-vim-resize-smoke.txt\"; rm -f \"\$tmp\"; ${vim_path} --clean -Nu NONE -n; printf \"vim-resize-result:%s\\n\" \"\$(cat \"\$tmp\")\"; rm -f \"\$tmp\""$'\n' \
    "24x80" \
    $':call writefile([printf("lines=%d columns=%d", &lines, &columns)], "/tmp/minimal-terminal-vim-resize-smoke.txt")\r:qall!\r' \
    "vim-resize-result:lines=24 columns=80" \
    3600 \
    1000 \
    1200
  ran=1
else
  echo "app target smoke skipped: vim not found"
fi

if command -v less >/dev/null 2>&1; then
  less_path="$(command -v less)"
  run_case_with_followup \
    "less-basic-quit" \
    "printf \"one\\ntwo\\nthree\\n\" | ${less_path}; printf \"less-basic-ok\\n\""$'\n' \
    "q" \
    "less-basic-ok" \
    1500 \
    1000
  run_case_with_mouse_report \
    "less-mouse-wheel-down" \
    "seq -f \"less-mouse-line-%03g\" 1 120 | ${less_path} --mouse --wheel-lines=10"$'\n' \
    "wheel-down-5" \
    "less-mouse-line-045" \
    2200 \
    1200
  run_case_with_followup \
    "less-search" \
    "seq -f \"less-search-line-%03g\" 1 120 | ${less_path}"$'\n' \
    $'/less-search-line-080\r' \
    "less-search-line-080" \
    3200 \
    1800
  run_case \
    "less-follow" \
    "tmp=\"/tmp/minimal-terminal-less-follow-\$\$.log\"; rm -f \"\$tmp\"; printf \"less-follow-line-001\\n\" > \"\$tmp\"; (sleep 0.2; printf \"less-follow-line-002\\nless-follow-marker\\n\" >> \"\$tmp\"; sleep 10; rm -f \"\$tmp\") & ${less_path} +F \"\$tmp\""$'\n' \
    "less-follow-marker" \
    5000
  ran=1
else
  echo "app target smoke skipped: less not found"
fi

repo_path="$(pwd)"
head_sha="$(git -C "${repo_path}" rev-parse --short HEAD)"
run_case \
  "git-log" \
  "git -C \"${repo_path}\" --no-pager log --oneline -1 --no-color"$'\n' \
  "${head_sha}" \
  8000
run_case_with_followup \
  "git-pager-quit" \
  "git -C \"${repo_path}\" log --oneline --graph --decorate -100 --color=never | less; printf \"git-pager-quit-ok\\n\""$'\n' \
  "q" \
  "git-pager-quit-ok" \
  1500 \
  1000
run_case_with_two_followups \
  "git-pager-page-quit" \
  "git -C \"${repo_path}\" log --oneline --graph --decorate -100 --color=never | less; printf \"git-pager-page-quit-ok\\n\""$'\n' \
  " " \
  "q" \
  "git-pager-page-quit-ok" \
  1500 \
  1000 \
  700
run_case_with_followup \
  "git-pager-search-quit" \
  "git -C \"${repo_path}\" log --oneline --graph --decorate -100 --color=never | less; printf \"git-pager-search-ok\\n\""$'\n' \
  $'/Implement\rq' \
  "git-pager-search-ok" \
  1500 \
  1000
run_case_with_followup \
  "git-pager-horizontal-quit" \
  "git -C \"${repo_path}\" log --pretty=format:\"%H %s\" -100 --color=never | less -S; printf \"git-pager-horizontal-ok\\n\""$'\n' \
  $'\e[Cq' \
  "git-pager-horizontal-ok" \
  1500 \
  1000
run_case_with_followup \
  "git-pager-mark-quit" \
  "git -C \"${repo_path}\" log --oneline --graph --decorate -100 --color=never | less; printf \"git-pager-mark-ok\\n\""$'\n' \
  $'ma\047aq' \
  "git-pager-mark-ok" \
  1500 \
  1000
ran=1

if command -v tmux >/dev/null 2>&1; then
  tmux_path="$(command -v tmux)"
  run_case "tmux-version" "${tmux_path} -V"$'\n' "tmux "
  run_case_with_followup \
    "tmux-attached-session" \
    "tmux_socket=\"minimal-terminal-app-smoke-\$\$\"; ${tmux_path} -L \"\$tmux_socket\" new-session -s minimal-terminal-smoke 'printf \"tmux-pane-ready\\n\"; read -r line'; printf \"tmux-workflow-ok\\n\"; ${tmux_path} -L \"\$tmux_socket\" kill-server >/dev/null 2>&1 || true"$'\n' \
    $'exit\r' \
    "tmux-workflow-ok" \
    2500 \
    1200
  run_case_with_followup \
    "tmux-split-pane" \
    "tmux_socket=\"minimal-terminal-app-smoke-\$\$\"; ${tmux_path} -L \"\$tmux_socket\" new-session -s minimal-terminal-smoke 'printf \"tmux-top-ready\\n\"; read -r line' \\; set-hook -g pane-exited 'kill-session' \\; split-window -v 'printf \"tmux-bottom-ready\\n\"; read -r line' \\; select-pane -D; printf \"tmux-split-ok\\n\"; ${tmux_path} -L \"\$tmux_socket\" kill-server >/dev/null 2>&1 || true"$'\n' \
    $'exit\r' \
    "tmux-split-ok" \
    3000 \
    1200
  run_case \
    "tmux-pane-resize" \
    "tmux_socket=\"minimal-terminal-app-smoke-\$\$\"; ${tmux_path} -L \"\$tmux_socket\" new-session -d -s minimal-terminal-resize 'sleep 30'; ${tmux_path} -L \"\$tmux_socket\" split-window -v 'sleep 30'; ${tmux_path} -L \"\$tmux_socket\" select-pane -D; before=\"\$(${tmux_path} -L \"\$tmux_socket\" display-message -p '#{pane_height}')\"; ${tmux_path} -L \"\$tmux_socket\" resize-pane -D 2; after=\"\$(${tmux_path} -L \"\$tmux_socket\" display-message -p '#{pane_height}')\"; ${tmux_path} -L \"\$tmux_socket\" kill-server >/dev/null 2>&1 || true; if [ \"\$after\" -gt \"\$before\" ]; then printf \"tmux-pane-resize-ok:%s>%s\\n\" \"\$after\" \"\$before\"; else printf \"tmux-pane-resize-failed:%s<=%s\\n\" \"\$after\" \"\$before\"; exit 1; fi"$'\n' \
    "tmux-pane-resize-ok" \
    2500
  tmux_copy_mode_script="$(pwd)/${LOG_DIR}/tmux-copy-mode-helper.sh"
  cat >"${tmux_copy_mode_script}" <<'TMUX_COPY_MODE_SCRIPT'
#!/usr/bin/env bash
set -euo pipefail

tmux_socket="minimal-terminal-app-smoke-$$"
out="/tmp/minimal-terminal-tmux-copy-mode-$$.txt"
rm -f "${out}"

"${TMUX_PATH}" -L "${tmux_socket}" new-session -d -s minimal-terminal-copy 'printf "alpha\ntmux-copy-source\nsecond-line\n"; sleep 30'
for _ in $(seq 1 30); do
  "${TMUX_PATH}" -L "${tmux_socket}" capture-pane -p -t minimal-terminal-copy:0.0 | grep -Fq "tmux-copy-source" && break
  sleep 0.1
done

"${TMUX_PATH}" -L "${tmux_socket}" copy-mode -t minimal-terminal-copy:0.0
sleep 0.1
"${TMUX_PATH}" -L "${tmux_socket}" send-keys -t minimal-terminal-copy:0.0 -X search-backward "tmux-copy-source"
sleep 0.1
"${TMUX_PATH}" -L "${tmux_socket}" send-keys -t minimal-terminal-copy:0.0 -X start-of-line
"${TMUX_PATH}" -L "${tmux_socket}" send-keys -t minimal-terminal-copy:0.0 -X begin-selection
"${TMUX_PATH}" -L "${tmux_socket}" send-keys -t minimal-terminal-copy:0.0 -X end-of-line
"${TMUX_PATH}" -L "${tmux_socket}" send-keys -t minimal-terminal-copy:0.0 -X copy-selection-and-cancel
sleep 0.1
"${TMUX_PATH}" -L "${tmux_socket}" save-buffer "${out}" 2>/dev/null || true
printf "tmux-copy-mode:%s\n" "$(cat "${out}" 2>/dev/null)"
rm -f "${out}"
"${TMUX_PATH}" -L "${tmux_socket}" kill-server >/dev/null 2>&1 || true
TMUX_COPY_MODE_SCRIPT
  chmod +x "${tmux_copy_mode_script}"
  run_case \
    "tmux-copy-mode" \
    "TMUX_PATH=\"${tmux_path}\" \"${tmux_copy_mode_script}\""$'\n' \
    "tmux-copy-mode:tmux-copy-source" \
    7000
  run_case_with_mouse_report \
    "tmux-mouse-wheel" \
    "tmux_socket=\"minimal-terminal-app-smoke-\$\$\"; ${tmux_path} -L \"\$tmux_socket\" new-session -s minimal-terminal-mouse 'for i in \$(seq 1 120); do if [ \"\$i\" -le 20 ]; then printf \"tmux-mouse-line-%03d tmux-mouse-scroll-marker\\n\" \"\$i\"; else printf \"tmux-mouse-line-%03d\\n\" \"\$i\"; fi; done; sleep 30' \\; set-option -g destroy-unattached on \\; set-option -g mouse on"$'\n' \
    "wheel-up-20" \
    "tmux-mouse-scroll-marker" \
    3500 \
    1800
  if command -v vim >/dev/null 2>&1; then
    vim_path="$(command -v vim)"
    run_case_with_followup \
      "tmux-vim-edit-write-quit" \
      "tmp=\"/tmp/minimal-terminal-tmux-vim-smoke-\$\$.txt\"; tmux_socket=\"minimal-terminal-app-smoke-\$\$\"; rm -f \"\$tmp\"; ${tmux_path} -L \"\$tmux_socket\" new-session -s minimal-terminal-nested \"${vim_path} --clean -Nu NONE -n \\\"\$tmp\\\"\"; printf \"tmux-vim-workflow-ok:%s\\n\" \"\$(cat \"\$tmp\")\"; rm -f \"\$tmp\"; ${tmux_path} -L \"\$tmux_socket\" kill-server >/dev/null 2>&1 || true"$'\n' \
      $'ihello from tmux vim\e:wq\r' \
      "tmux-vim-workflow-ok:hello from tmux vim" \
      3000 \
      1400
    tmux_split_vim_script="${repo_path}/${LOG_DIR}/tmux-split-vim-resize.sh"
    cat >"${tmux_split_vim_script}" <<EOF
#!/usr/bin/env bash
set -euo pipefail
tmp="/tmp/minimal-terminal-tmux-split-vim-smoke-\$\$.txt"
resize_out="/tmp/minimal-terminal-tmux-split-vim-resize-\$\$.txt"
tmux_socket="minimal-terminal-app-smoke-\$\$"
rm -f "\${tmp}" "\${resize_out}"
(
  sleep 1.0
  before="\$(${tmux_path} -L "\${tmux_socket}" display-message -p -t minimal-terminal-split-vim:0.1 '#{pane_height}' 2>/dev/null || true)"
  ${tmux_path} -L "\${tmux_socket}" resize-pane -t minimal-terminal-split-vim:0.1 -D 2
  after="\$(${tmux_path} -L "\${tmux_socket}" display-message -p -t minimal-terminal-split-vim:0.1 '#{pane_height}' 2>/dev/null || true)"
  if [ -n "\${before}" ] && [ -n "\${after}" ] && [ "\${after}" != "\${before}" ]; then
    printf "resize-ok:%s->%s\n" "\${before}" "\${after}" >"\${resize_out}"
  else
    printf "resize-failed:%s->%s\n" "\${before}" "\${after}" >"\${resize_out}"
  fi
) &
${tmux_path} -L "\${tmux_socket}" new-session -s minimal-terminal-split-vim 'printf "tmux-top-ready\n"; read -r line' \; set-hook -g pane-exited 'kill-session' \; split-window -v "${vim_path} --clean -Nu NONE -n \"\${tmp}\"" \; select-pane -t minimal-terminal-split-vim:0.1
resize_result="\$(cat "\${resize_out}" 2>/dev/null)"
case "\${resize_result}" in
  resize-ok:*) printf "tmux-split-vim-resize-ok:%s:%s\n" "\$(cat "\${tmp}")" "\${resize_result}" ;;
  *) printf "tmux-split-vim-resize-failed:%s:%s\n" "\$(cat "\${tmp}" 2>/dev/null)" "\${resize_result}"; exit 1 ;;
esac
rm -f "\${tmp}" "\${resize_out}" "\${BASH_SOURCE[0]}"
${tmux_path} -L "\${tmux_socket}" kill-server >/dev/null 2>&1 || true
EOF
    chmod +x "${tmux_split_vim_script}"
    run_case_with_followup \
      "tmux-split-vim-resize" \
      "${tmux_split_vim_script}"$'\n' \
      $'ihello from split tmux vim\e:wq\r' \
      "tmux-split-vim-resize-ok:hello from split tmux vim:resize-ok" \
      4500 \
      2100
  else
    echo "app target smoke skipped: tmux vim targets require vim"
  fi
  ran=1
else
  echo "app target smoke skipped: tmux not found"
fi

if command -v htop >/dev/null 2>&1; then
  htop_path="$(command -v htop)"
  run_case "htop-version" "${htop_path} --version"$'\n' "htop"
  run_case_with_required_markers \
    "htop-runtime" \
    "${htop_path}"$'\n' \
    3000 \
    "Mem[" \
    "Tasks:" \
    "Load average:" \
    "PID USER" \
    "Command" \
    "F1Help  F2Setup" \
    "F10Quit"
  run_case_with_followup \
    "htop-quit" \
    "${htop_path}; printf \"htop-quit-ok\\n\""$'\n' \
    "q" \
    "htop-quit-ok" \
    2000 \
    1200
  run_case_with_followup \
    "htop-f10-quit" \
    "${htop_path}; printf \"htop-f10-ok\\n\""$'\n' \
    $'\e[21~' \
    "htop-f10-ok" \
    2000 \
    1200
  run_case_with_mouse_report \
    "htop-mouse-setup" \
    "${htop_path}"$'\n' \
    "left-click:28:10" \
    "[Setup]" \
    2000 \
    1200
  run_case_with_followup \
    "htop-f1-help-quit" \
    "${htop_path}; printf \"htop-f1-ok\\n\""$'\n' \
    $'\eOPqq' \
    "htop-f1-ok" \
    2500 \
    1200
  run_case_with_followup \
    "htop-f5-tree" \
    "${htop_path}"$'\n' \
    $'\e[15~' \
    "├─" \
    2500 \
    1200
  run_case_with_followup \
    "htop-f2-setup" \
    "${htop_path}"$'\n' \
    $'\eOQ' \
    "[Setup]" \
    2500 \
    1200
  run_case_with_followup \
    "htop-setup-save" \
    "tmpdir=\"\$(mktemp -d /tmp/minimal-terminal-htop-setup.XXXXXX)\"; rc=\"\$tmpdir/htoprc\"; HTOPRC=\"\$rc\" ${htop_path}; if [ -s \"\$rc\" ]; then printf \"htop-setup-save-ok\\n\"; else printf \"htop-setup-save-missing\\n\"; exit 1; fi; rm -rf \"\$tmpdir\""$'\n' \
    $'\eOQ\e[21~q' \
    "htop-setup-save-ok" \
    3000 \
    1200
  run_case_with_followup \
    "htop-setup-toggle-tree" \
    "tmpdir=\"\$(mktemp -d /tmp/minimal-terminal-htop-toggle.XXXXXX)\"; rc=\"\$tmpdir/htoprc\"; HTOPRC=\"\$rc\" ${htop_path}; if grep -Eq '(^|[.])tree_view=1$' \"\$rc\"; then printf \"htop-setup-toggle-tree-ok\\n\"; else printf \"htop-setup-toggle-tree-missing\\n\"; cat \"\$rc\" 2>/dev/null; rm -rf \"\$tmpdir\"; exit 1; fi; rm -rf \"\$tmpdir\""$'\n' \
    $'\eOQ\eOC\eOB \e[21~q' \
    "htop-setup-toggle-tree-ok" \
    3000 \
    1200
  ran=1
else
  echo "app target smoke skipped: htop not found"
fi

if command -v claude >/dev/null 2>&1; then
  claude_path="$(command -v claude)"
  run_case "claude-version" "${claude_path} --version"$'\n' "Claude Code"
  run_case "claude-help" "${claude_path} --help"$'\n' "Usage: claude"
  ran=1
else
  echo "app target smoke skipped: claude not found"
fi

if command -v codex-cli >/dev/null 2>&1; then
  codex_cli_path="$(command -v codex-cli)"
  run_case "codex-cli-version" "${codex_cli_path} --version"$'\n' "codex-cli"
  run_case "codex-cli-help" "${codex_cli_path} --help"$'\n' "Commands:"
  ran=1
elif command -v codex >/dev/null 2>&1; then
  codex_path="$(command -v codex)"
  run_case "codex-version" "${codex_path} --version"$'\n' "codex-cli"
  run_case "codex-help" "${codex_path} --help"$'\n' "Commands:"
  ran=1
else
  echo "app target smoke skipped: codex/codex-cli not found"
fi

if command -v vttest >/dev/null 2>&1; then
  vttest_path="$(command -v vttest)"
  run_case "vttest-version" "${vttest_path} -V"$'\n' "VT100 test program"
  run_case "vttest-menu" "${vttest_path}"$'\n' "VT100 test program"
  run_case_with_followup \
    "vttest-cursor-movement" \
    "${vttest_path}"$'\n' \
    $'1\r' \
    "The screen should be cleared" \
    1500 \
    1000
  run_case_with_followup \
    "vttest-screen-features" \
    "${vttest_path}"$'\n' \
    $'2\r' \
    "Test of WRAP AROUND mode setting" \
    1500 \
    1000
  run_case_with_followup \
    "vttest-character-sets" \
    "${vttest_path}"$'\n' \
    $'3\r' \
    "These are the installed character sets" \
    1500 \
    1000
  run_case_with_followup \
    "vttest-double-sized" \
    "${vttest_path}"$'\n' \
    $'4\r' \
    "Double-width-and-height line" \
    1500 \
    1000
  run_case_with_followup \
    "vttest-dsr" \
    "${vttest_path}"$'\n' \
    $'6\r3\r' \
    "Report is: <27> [ 5 ; 1 R  -- OK" \
    2500 \
    1000
  ran=1
else
  echo "app target smoke skipped: vttest not found"
fi

if [[ "${ran}" -eq 0 ]]; then
  echo "app target smoke skipped: no targets available"
fi
