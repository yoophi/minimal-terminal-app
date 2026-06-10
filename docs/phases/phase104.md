# Phase 104 - tmux Pane Resize App Smoke

## 목표

app 내부 PTY에서 `tmux` pane resize workflow를 자동 smoke로 고정한다.

## 범위

- `scripts/run-app-target-smokes.sh`에 `tmux-pane-resize` target을 추가한다.
- app 내부 shell에서 별도 tmux socket으로 detached session을 만든다.
- vertical split pane을 만들고 active pane의 `#{pane_height}`를 읽는다.
- `resize-pane -D 2` 실행 후 active pane 높이가 증가했는지 비교한다.
- compatibility matrix, smoke test 문서, known gaps, README를 새 evidence에 맞게 갱신한다.

## 제외 범위

- attached tmux client에서 실제 keyboard prefix로 resize 명령을 입력하는 workflow
- tmux mouse mode
- tmux copy mode
- split-pane 내부 nested TUI resize 검증

## 완료 기준

- `scripts/run-app-target-smokes.sh`에서 `tmux-pane-resize`가 통과한다.
- resize 전후 pane height 비교 marker `tmux-pane-resize-ok`가 app snapshot에 남는다.
- `docs/compatibility/matrix.md`, `docs/compatibility/smoke-tests.md`, `docs/compatibility/known-gaps.md`가 새 evidence를 반영한다.

## 결과

상태: 구현 완료.

- local verification environment에서 app 내부 PTY로 detached tmux split pane을 만들고 resize 후 active pane height 증가를 확인했다.
- `tmux`의 남은 대표 gap은 mouse mode, copy mode, split-pane 내부 nested TUI resize/interaction으로 좁혔다.
