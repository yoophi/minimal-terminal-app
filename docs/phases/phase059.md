# Phase 059 - tmux Nested vim App Smoke

## 목표

`tmux` 안의 `vim`은 alternate screen과 terminal mode가 중첩되는 대표 workflow다. tmux 단독 session과 vim 단독 edit smoke를 넘어, nested TUI 실행 후 저장과 shell 복원이 되는지 자동화한다.

## 범위

1. `scripts/run-app-target-smokes.sh`에 `tmux-vim-edit-write-quit` target을 추가한다.
2. 앱 내부 PTY에서 attached tmux session을 열고 그 안에서 clean vim을 실행한다.
3. follow-up input으로 vim insert, write, quit를 수행한다.
4. vim과 tmux가 종료된 뒤 shell marker가 저장된 파일 내용을 출력하는지 확인한다.
5. matrix, smoke test, app readiness, known gap 문서를 새 evidence에 맞게 갱신한다.

## 비범위

- tmux 안의 vim mouse workflow, pane resize, split-pane 내부 vim, plugin 환경은 자동화하지 않는다.
- 장시간 편집 session과 복잡한 key chord는 별도 gap으로 남긴다.

## Acceptance Criteria

- [done] `tmux-vim-edit-write-quit` app target smoke가 추가되어 있다.
- [done] 현재 local environment에서 `tmux-vim-edit-write-quit` app target smoke가 통과한다.
- [done] `tmux` 안의 `vim` 문서가 replay-only evidence에서 nested app-internal evidence로 갱신되어 있다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## 결과

- app 내부 PTY에서 tmux attached session을 열고 그 안의 vim tempfile에 `hello from tmux vim`을 저장한 뒤 shell marker `tmux-vim-workflow-ok:hello from tmux vim`을 확인한다.
