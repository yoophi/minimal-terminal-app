# Phase 145 - fzf Ctrl-T Smoke Selection Stabilization

## 목적

Phase 143 검증 중 기존 `fzf-shell-ctrl-t` target이 query timing에 따라 첫 후보인 `alpha-file`을 선택하는 failure를 보였다. 이 phase는 Ctrl-T widget workflow가 query 입력 timing에 덜 의존하도록 안정화한다.

## 범위

- `fzf-shell-ctrl-t` target의 임시 파일 후보를 `phase-fzf-shell-target` 하나로 줄인다.
- Ctrl-T widget이 file path를 shell command line에 삽입하고 marker command가 실행되는 기존 evidence는 유지한다.

## 완료 기준

- [done] `fzf-shell-ctrl-t` app target smoke가 `fzf-shell:phase-fzf-shell-target` marker를 안정적으로 관측한다.
- [done] `scripts/run-app-target-smokes.sh` 전체 suite가 통과한다.

## 비범위

- 사용자별 shell/plugin 설정이 포함된 fzf integration workflow
- fzf 자체 query/filter 알고리즘 검증
