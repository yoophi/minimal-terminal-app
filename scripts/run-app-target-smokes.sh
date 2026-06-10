#!/usr/bin/env bash
set -euo pipefail

APP_DIR="target/debug/Minimal Terminal.app"
APP_BINARY="${APP_DIR}/Contents/MacOS/terminal-app"
LOG_DIR="target/debug/app-target-smokes"
WAIT_SECONDS="${WAIT_SECONDS:-8}"

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
  local case_dir="${LOG_DIR}/${name}"
  local snapshot_path="${case_dir}/snapshot.txt"
  local stdout_path="${case_dir}/stdout.log"
  local stderr_path="${case_dir}/stderr.log"

  mkdir -p "${case_dir}"
  rm -f "${snapshot_path}" "${stdout_path}" "${stderr_path}"

  MINIMAL_TERMINAL_SMOKE_INPUT="${input}" \
  MINIMAL_TERMINAL_SMOKE_SNAPSHOT_PATH="${snapshot_path}" \
  MINIMAL_TERMINAL_SMOKE_INPUT_DELAY_MS=500 \
  MINIMAL_TERMINAL_SMOKE_SNAPSHOT_DELAY_MS=2500 \
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

ran=0

if command -v fzf >/dev/null 2>&1; then
  run_case "fzf-filter" $'printf "alpha\\nbeta\\n" | fzf --filter alpha\n' "alpha"
  ran=1
else
  echo "app target smoke skipped: fzf not found"
fi

head_sha="$(git rev-parse --short HEAD)"
run_case "git-log" $'git log --oneline -1 --no-color\n' "${head_sha}"
ran=1

if [[ "${ran}" -eq 0 ]]; then
  echo "app target smoke skipped: no targets available"
fi
