# Phase 058 - vim Edit Write Quit App Smoke

## 목표

`vim`은 replay fixture evidence만으로는 실제 app 내부 PTY에서 normal/insert mode 전환, typed input, file write, alternate screen 종료 후 shell 복원을 충분히 확인할 수 없다. 최소 edit/write/quit workflow를 자동화한다.

## 범위

1. `scripts/run-app-target-smokes.sh`에 `vim-edit-write-quit` target을 추가한다.
2. 앱 내부 PTY에서 `vim --clean -Nu NONE -n <tempfile>`을 실행한다.
3. follow-up input으로 insert mode 진입, 텍스트 입력, `Esc`, `:wq`를 수행한다.
4. 종료 후 shell marker가 저장된 파일 내용을 출력하는지 확인한다.
5. matrix, smoke test, app readiness, known gap 문서를 새 evidence에 맞게 갱신한다.

## 비범위

- vim mouse workflow, 복잡한 key chord, plugin 환경, search highlight, resize workflow는 자동화하지 않는다.
- `tmux` 안의 `vim` nested workflow는 별도 phase로 남긴다.

## Acceptance Criteria

- [done] `vim-edit-write-quit` app target smoke가 추가되어 있다.
- [done] 현재 local environment에서 `vim-edit-write-quit` app target smoke가 통과한다.
- [done] vim app readiness 문서가 replay-only evidence에서 app-internal edit/write/quit evidence로 갱신되어 있다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## 결과

- `/usr/bin/vim` 9.1 환경에서 app 내부 PTY로 tempfile을 열고 `hello from vim`을 저장한 뒤 shell marker `vim-workflow-ok:hello from vim`을 확인한다.
