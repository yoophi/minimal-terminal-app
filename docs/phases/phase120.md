# Phase 120 - App Target Smoke Wait Budget Stabilization

## Purpose

긴 follow-up 입력과 snapshot delay를 함께 쓰는 app target smoke가 기능은 통과해도 기본 종료 제한에 가까워져 간헐적으로 timeout 되는 문제를 줄인다.

## Scope

- `scripts/run-app-target-smokes.sh`의 기본 `WAIT_SECONDS`를 12초로 늘린다.
- `tmux-split-vim-resize`처럼 helper script, follow-up 입력, snapshot delay를 모두 사용하는 target이 안정적으로 종료될 시간을 확보한다.
- smoke 문서와 compatibility evidence에 timeout budget 변경을 기록한다.

## Proposed Work Breakdown

1. app target smoke 기본 wait budget을 8초에서 12초로 조정한다.
2. 전체 app target smoke suite를 실행해 target들이 통과하는지 확인한다.
3. README와 compatibility 문서에 Phase 120 결과를 반영한다.

## Acceptance Criteria

- [done] `scripts/run-app-target-smokes.sh` 기본 wait budget이 12초다.
- [done] 전체 app target smoke suite가 통과한다.
- [done] 관련 문서에 wait budget stabilization이 기록되어 있다.

## Non-goals

- 개별 target의 command workflow 변경
- smoke hook의 event injection 구조 변경
- app command smoke 기본 timeout 변경

## Result

상태: 구현 완료.

`tmux-split-vim-resize`를 포함한 전체 `scripts/run-app-target-smokes.sh` suite가 local verification environment에서 통과한다.
