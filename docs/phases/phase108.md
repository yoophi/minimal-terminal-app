# Phase 108 - tmux Copy Mode App Smoke

## 목표

app 내부 PTY에서 `tmux` copy mode workflow를 자동 smoke로 고정한다.

## 범위

- `scripts/run-app-target-smokes.sh`에 `tmux-copy-mode` target을 추가한다.
- 별도 tmux socket에서 detached session과 pane output을 만든다.
- `copy-mode`, `search-backward`, selection command, `copy-selection-and-cancel`을 실행한다.
- `save-buffer` 결과가 `tmux-copy-source`인지 확인한다.
- compatibility matrix, smoke test 문서, app readiness, known gaps, README를 새 evidence에 맞게 갱신한다.

## 제외 범위

- attached tmux client에서 keyboard prefix로 copy mode를 직접 조작하는 workflow
- tmux mouse mode
- system clipboard integration
- scrollback이 큰 pane의 copy 성능

## 완료 기준

- `scripts/run-app-target-smokes.sh`에서 `tmux-copy-mode`가 통과한다.
- app snapshot에 `tmux-copy-mode:tmux-copy-source` marker가 남는다.
- compatibility 문서가 tmux copy mode evidence를 반영한다.

## 결과

상태: 구현 완료.

- local verification environment에서 app 내부 PTY로 tmux copy mode selection을 실행하고 tmux buffer 저장 결과를 확인했다.
- `tmux`의 남은 대표 gap은 mouse mode와 split-pane 내부 nested TUI resize/interaction으로 좁혔다.
