# Phase 144 - fzf Alt-C Smoke Selection Stabilization

## 목적

Phase 143 검증 중 기존 `fzf-shell-alt-c` target이 query timing에 따라 첫 후보인 `alpha-dir`를 선택하는 failure를 보였다. 이 phase는 Alt-C widget workflow가 query 입력 timing에 덜 의존하도록 안정화한다.

## 범위

- `fzf-shell-alt-c` target의 임시 디렉터리 후보를 `phase-fzf-alt-c-target` 하나로 줄인다.
- Alt-C widget이 directory를 선택하고 shell cwd marker를 출력하는 기존 evidence는 유지한다.

## 완료 기준

- [done] `fzf-shell-alt-c` app target smoke가 `fzf-alt-c:phase-fzf-alt-c-target` marker를 안정적으로 관측한다.
- [done] `scripts/run-app-target-smokes.sh` 전체 suite가 통과한다.

## 비범위

- 사용자별 shell/plugin 설정이 포함된 fzf integration workflow
- fzf 자체 query/filter 알고리즘 검증
