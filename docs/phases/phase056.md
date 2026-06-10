# Phase 056 - tmux Attached Session App Smoke

## 목표

`tmux`는 version smoke만으로는 alternate screen, pane input, session 종료 후 shell 복원 여부를 확인할 수 없다. 앱 내부 PTY에서 attached tmux session을 실행하고 pane 입력으로 정상 종료되는 최소 workflow를 자동화한다.

## 범위

1. `scripts/run-app-target-smokes.sh`에 `tmux-attached-session` target을 추가한다.
2. 앱 내부 PTY에서 `tmux new-session`을 attached mode로 실행한다.
3. follow-up input으로 pane의 `read`를 종료하고 tmux client가 shell로 복귀하는지 marker로 확인한다.
4. matrix, smoke test, known gap 문서를 새 evidence에 맞게 갱신한다.

## 비범위

- tmux pane split, resize, mouse mode, nested `vim` workflow는 자동화하지 않는다.
- tmux 설정 파일이나 plugin 환경을 검증하지 않는다.

## Acceptance Criteria

- [done] `tmux-attached-session` app target smoke가 추가되어 있다.
- [done] 현재 local environment에서 `tmux-attached-session` app target smoke가 통과한다.
- [done] tmux app readiness 문서가 version-only evidence에서 attached workflow evidence로 갱신되어 있다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## 결과

- `tmux -L <socket> new-session` attached workflow를 app 내부 PTY에서 실행한다.
- follow-up `exit` 입력으로 pane process가 종료되고 shell marker `tmux-workflow-ok`가 출력되는지 확인한다.
