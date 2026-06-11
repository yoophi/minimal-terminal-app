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

ran=0

run_case_with_mouse_report \
  "mouse-sgr-report" \
  $'stty raw -echo; printf "\\033[?1000h\\033[?1006h"; bytes="$(dd bs=1 count=9 2>/dev/null | od -An -tx1 | tr -d " \\n")"; stty sane; printf "\\nmouse-sgr-report:%s\\n" "$bytes"\n' \
  "left-press" \
  "mouse-sgr-report:1b5b3c303b333b324d" \
  1500 \
  1000
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
  run_case_with_followup \
    "fzf-preview" \
    "printf \"alpha\\nbeta\\n\" | ${fzf_path} --preview \"printf preview:{}\""$'\n' \
    "b" \
    "preview:beta" \
    1500 \
    1000
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
    "tmpdir=\"\$(mktemp -d /tmp/minimal-terminal-fzf-shell.XXXXXX)\"; touch \"\$tmpdir/alpha-file\" \"\$tmpdir/phase-fzf-shell-target\"; cd \"\$tmpdir\"; source /opt/homebrew/opt/fzf/shell/key-bindings.zsh; printf \"fzf-shell-ready\\n\""$'\n' \
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
    "tmpdir=\"\$(mktemp -d /tmp/minimal-terminal-fzf-alt-c.XXXXXX)\"; mkdir -p \"\$tmpdir/alpha-dir\" \"\$tmpdir/phase-fzf-alt-c-target\"; cd \"\$tmpdir\"; source /opt/homebrew/opt/fzf/shell/key-bindings.zsh; printf \"fzf-alt-c-ready\\n\""$'\n' \
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
    "tmp=\"/tmp/minimal-terminal-less-follow-\$\$.log\"; rm -f \"\$tmp\"; printf \"less-follow-line-001\\n\" > \"\$tmp\"; (sleep 1; printf \"less-follow-line-002\\nless-follow-marker\\n\" >> \"\$tmp\"; sleep 10; rm -f \"\$tmp\") & ${less_path} +F \"\$tmp\""$'\n' \
    "less-follow-marker" \
    3500
  ran=1
else
  echo "app target smoke skipped: less not found"
fi

head_sha="$(git rev-parse --short HEAD)"
run_case_with_followup \
  "git-log" \
  "git --no-pager log --oneline -1 --no-color" \
  $'\r' \
  "${head_sha}" \
  5000 \
  1000
run_case_with_followup \
  "git-pager-quit" \
  $'git log --oneline --graph --decorate -100 --color=never | less; printf "git-pager-quit-ok\\n"\n' \
  "q" \
  "git-pager-quit-ok" \
  1500 \
  1000
run_case_with_two_followups \
  "git-pager-page-quit" \
  $'git log --oneline --graph --decorate -100 --color=never | less; printf "git-pager-page-quit-ok\\n"\n' \
  " " \
  "q" \
  "git-pager-page-quit-ok" \
  1500 \
  1000 \
  700
run_case_with_followup \
  "git-pager-search-quit" \
  $'git log --oneline --graph --decorate -100 --color=never | less; printf "git-pager-search-ok\\n"\n' \
  $'/Implement\rq' \
  "git-pager-search-ok" \
  1500 \
  1000
run_case_with_followup \
  "git-pager-horizontal-quit" \
  $'git log --pretty=format:"%H %s" -100 --color=never | less -S; printf "git-pager-horizontal-ok\\n"\n' \
  $'\e[Cq' \
  "git-pager-horizontal-ok" \
  1500 \
  1000
run_case_with_followup \
  "git-pager-mark-quit" \
  $'git log --oneline --graph --decorate -100 --color=never | less; printf "git-pager-mark-ok\\n"\n' \
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
  run_case \
    "tmux-copy-mode" \
    "tmux_socket=\"minimal-terminal-app-smoke-\$\$\"; out=\"/tmp/minimal-terminal-tmux-copy-mode-\$\$.txt\"; rm -f \"\$out\"; ${tmux_path} -L \"\$tmux_socket\" new-session -d -s minimal-terminal-copy 'printf \"alpha\\ntmux-copy-source\\nsecond-line\\n\"; sleep 30'; ${tmux_path} -L \"\$tmux_socket\" copy-mode -t minimal-terminal-copy:0.0; ${tmux_path} -L \"\$tmux_socket\" send-keys -t minimal-terminal-copy:0.0 -X search-backward \"tmux-copy-source\"; ${tmux_path} -L \"\$tmux_socket\" send-keys -t minimal-terminal-copy:0.0 -X start-of-line; ${tmux_path} -L \"\$tmux_socket\" send-keys -t minimal-terminal-copy:0.0 -X begin-selection; ${tmux_path} -L \"\$tmux_socket\" send-keys -t minimal-terminal-copy:0.0 -X end-of-line; ${tmux_path} -L \"\$tmux_socket\" send-keys -t minimal-terminal-copy:0.0 -X copy-selection-and-cancel; ${tmux_path} -L \"\$tmux_socket\" save-buffer \"\$out\" 2>/dev/null || true; printf \"tmux-copy-mode:%s\\n\" \"\$(cat \"\$out\" 2>/dev/null)\"; rm -f \"\$out\"; ${tmux_path} -L \"\$tmux_socket\" kill-server >/dev/null 2>&1 || true"$'\n' \
    "tmux-copy-mode:tmux-copy-source" \
    3500
  run_case_with_mouse_report \
    "tmux-mouse-wheel" \
    "tmux_socket=\"minimal-terminal-app-smoke-\$\$\"; ${tmux_path} -L \"\$tmux_socket\" new-session -s minimal-terminal-mouse 'for i in \$(seq 1 120); do if [ \"\$i\" -ge 80 ] && [ \"\$i\" -le 93 ]; then printf \"tmux-mouse-line-%03d tmux-mouse-scroll-marker\\n\" \"\$i\"; else printf \"tmux-mouse-line-%03d\\n\" \"\$i\"; fi; done; sleep 30' \\; set-option -g destroy-unattached on \\; set-option -g mouse on"$'\n' \
    "wheel-down-20" \
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
    tmux_split_vim_script="${LOG_DIR}/tmux-split-vim-resize.sh"
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
    "Tasks:" \
    "Load average:" \
    "PID USER" \
    "Command" \
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
  run_case "vttest-menu" "${vttest_path}"$'\n' "VT100 test program"
  ran=1
else
  echo "app target smoke skipped: vttest not found"
fi

if [[ "${ran}" -eq 0 ]]; then
  echo "app target smoke skipped: no targets available"
fi
