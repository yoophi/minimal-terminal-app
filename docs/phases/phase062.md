# Phase 062 - fzf Shell Ctrl-T App Smoke

## 목표

기존 fzf smoke는 직접 `fzf` 실행, selection, multi-select, preview를 확인했다. shell integration gap을 줄이기 위해 zsh key binding을 source한 뒤 Ctrl-T widget으로 파일 경로를 command line에 삽입하고 실행하는 workflow를 자동화한다.

## 범위

1. smoke hook에 `MINIMAL_TERMINAL_SMOKE_THIRD_FOLLOWUP_INPUT`과 delay 환경 변수를 추가한다.
2. `scripts/run-app-target-smokes.sh`에 3단계 follow-up helper를 추가한다.
3. `fzf-shell-ctrl-t` target을 추가한다.
4. app 내부 PTY에서 zsh fzf key binding을 source하고, command prefix 입력, Ctrl-T, query/selection, final Enter를 단계별로 수행한다.
5. matrix, smoke test, known gap 문서를 새 evidence에 맞게 갱신한다.

## 비범위

- Ctrl-R history integration, Alt-C directory change widget은 자동화하지 않는다.
- shell plugin manager나 사용자별 fzf 설정 파일은 검증하지 않는다.

## Acceptance Criteria

- [done] 세 번째 follow-up smoke input이 지원된다.
- [done] `fzf-shell-ctrl-t` app target smoke가 추가되어 있다.
- [done] 현재 local environment에서 `fzf-shell-ctrl-t` app target smoke가 통과한다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## 결과

- app 내부 PTY에서 `/opt/homebrew/opt/fzf/shell/key-bindings.zsh`를 source한 뒤 Ctrl-T widget으로 `phase-fzf-shell-target` 경로를 command line에 삽입하고 shell marker `fzf-shell:phase-fzf-shell-target`를 확인한다.
