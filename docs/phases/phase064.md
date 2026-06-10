# Phase 064 - fzf Shell Ctrl-R App Smoke

## 목표

Phase 062와 Phase 063에서 fzf zsh Ctrl-T file widget과 Alt-C directory widget을 자동화했다. 남은 주요 fzf shell integration인 Ctrl-R history widget을 app 내부 PTY에서 실행하고 선택된 history command가 shell에서 실행되는지 확인한다.

## 범위

1. `scripts/run-app-target-smokes.sh`에 `fzf-shell-ctrl-r` target을 추가한다.
2. zsh history에 marker command를 `print -s`로 주입하고 fzf key binding을 source한다.
3. follow-up input으로 Ctrl-R sequence, query/Enter, final Enter를 보낸다.
4. shell marker command가 실행되어 `fzf-history-ok`가 출력되는지 확인한다.
5. matrix, smoke test, known gap 문서를 새 evidence에 맞게 갱신한다.

## 비범위

- 사용자별 shell history 파일, duplicate history policy, fzf history sort toggle은 자동화하지 않는다.
- shell plugin manager integration은 검증하지 않는다.

## Acceptance Criteria

- [done] `fzf-shell-ctrl-r` app target smoke가 추가되어 있다.
- [done] 현재 local environment에서 `fzf-shell-ctrl-r` app target smoke가 통과한다.
- [done] fzf shell integration 문서가 Ctrl-T, Alt-C, Ctrl-R evidence를 구분해 설명한다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## 결과

- app 내부 PTY에서 zsh history에 `printf "fzf-history-ok\n"`를 추가하고 Ctrl-R widget으로 선택한 뒤 shell marker `fzf-history-ok`를 확인한다.
