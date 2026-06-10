# Phase 063 - fzf Shell Alt-C App Smoke

## 목표

Phase 062에서 zsh Ctrl-T file widget을 자동화했다. 남은 shell integration gap을 줄이기 위해 Alt-C directory widget을 app 내부 PTY에서 실행하고, 선택된 directory로 shell cwd가 변경되는지 확인한다.

## 범위

1. `scripts/run-app-target-smokes.sh`에 `fzf-shell-alt-c` target을 추가한다.
2. 임시 directory 아래 `phase-fzf-alt-c-target` directory를 만들고 zsh fzf key binding을 source한다.
3. follow-up input으로 Alt-C sequence `ESC c`, query/Enter, cwd marker command를 보낸다.
4. shell marker가 선택된 directory basename을 출력하는지 확인한다.
5. matrix, smoke test, known gap 문서를 새 evidence에 맞게 갱신한다.

## 비범위

- Ctrl-R history widget과 사용자별 shell/plugin 설정은 자동화하지 않는다.
- directory preview, multi-select, shell completion integration은 검증하지 않는다.

## Acceptance Criteria

- [done] `fzf-shell-alt-c` app target smoke가 추가되어 있다.
- [done] 현재 local environment에서 `fzf-shell-alt-c` app target smoke가 통과한다.
- [done] fzf shell integration 문서가 Ctrl-T와 Alt-C evidence를 구분해 설명한다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## 결과

- app 내부 PTY에서 `/opt/homebrew/opt/fzf/shell/key-bindings.zsh`를 source한 뒤 Alt-C widget으로 `phase-fzf-alt-c-target` directory를 선택하고 shell marker `fzf-alt-c:phase-fzf-alt-c-target`를 확인한다.
