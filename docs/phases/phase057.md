# Phase 057 - tmux Split Pane App Smoke

## 목표

Phase 056은 attached tmux session의 최소 실행과 shell 복원을 확인했다. Phase 057은 tmux가 split-pane을 만들 때 발생하는 pane redraw, active pane input, session 정리 경로를 app 내부 PTY smoke로 고정한다.

## 범위

1. `scripts/run-app-target-smokes.sh`에 `tmux-split-pane` target을 추가한다.
2. 앱 내부 PTY에서 attached tmux session을 시작하고 vertical split pane을 생성한다.
3. active pane에 follow-up input을 보내 종료한 뒤 tmux session이 정리되고 shell marker가 출력되는지 확인한다.
4. matrix, smoke test, known gap 문서를 새 evidence에 맞게 갱신한다.

## 비범위

- tmux pane resize, mouse mode, copy mode, nested `vim` workflow는 자동화하지 않는다.
- tmux 사용자 설정과 plugin 환경을 검증하지 않는다.

## Acceptance Criteria

- [done] `tmux-split-pane` app target smoke가 추가되어 있다.
- [done] 현재 local environment에서 `tmux-split-pane` app target smoke가 통과한다.
- [done] tmux 문서가 split-pane workflow evidence를 설명한다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## 결과

- `tmux new-session`, `split-window -v`, active pane input, session cleanup, shell restore marker를 app 내부 PTY snapshot으로 확인한다.
