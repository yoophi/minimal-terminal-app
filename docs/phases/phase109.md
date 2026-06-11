# Phase 109 - tmux Mouse Wheel App Smoke

## 목표

app 내부 PTY에서 `tmux` mouse mode wheel workflow를 자동 smoke로 고정한다.

## 범위

- `scripts/run-app-target-smokes.sh`에 `tmux-mouse-wheel` target을 추가한다.
- 별도 tmux socket에서 `mouse on`을 켠 attached session을 실행한다.
- pane에 120줄 output을 만들고 smoke hook의 `wheel-down-5` mouse reports를 보낸다.
- snapshot에서 history scroll marker `tmux-mouse-line-080`을 확인한다.
- compatibility matrix, smoke test 문서, app readiness, known gaps, README를 새 evidence에 맞게 갱신한다.

## 제외 범위

- tmux mouse drag selection
- pane resize를 mouse drag로 수행하는 workflow
- nested TUI 내부 mouse event forwarding
- native GUI `NSEvent` 합성부터 PTY write까지의 end-to-end mouse 검증

## 완료 기준

- `scripts/run-app-target-smokes.sh`에서 `tmux-mouse-wheel`이 통과한다.
- app snapshot에 `tmux-mouse-line-080` marker가 남는다.
- compatibility 문서가 tmux mouse mode wheel evidence를 반영한다.

## 결과

상태: 구현 완료.

- local verification environment에서 app 내부 PTY로 tmux mouse mode를 켜고 smoke hook wheel reports 뒤 pane history가 이동하는 것을 확인했다.
- `tmux`의 남은 대표 gap은 split-pane 내부 nested TUI resize/interaction과 native GUI mouse end-to-end 범위로 좁혔다.
