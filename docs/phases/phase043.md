# Phase 043: fzf Enter Selection App Smoke

## Purpose

Phase 043의 목적은 app 내부 PTY에서 `fzf` interactive UI에 진입한 뒤 query 입력과 Enter selection이 순서대로 처리되는 workflow를 자동 확인하는 것이다.

Phase 039는 query 입력 후 filtered redraw만 확인했다. 이 phase에서는 Enter로 선택 결과가 shell command substitution에 반환되는 경로까지 smoke coverage를 넓힌다.

## Scope

Phase 043에서 다룰 작업:

1. `scripts/run-app-target-smokes.sh`에 `fzf-select` target 추가
2. app 내부 PTY에서 `printf "alpha\nbeta\n" | fzf` 실행
3. follow-up `b` 입력 후 두 번째 follow-up Enter 입력
4. shell marker `fzf-select:beta` 확인
5. matrix와 known gaps 갱신

## Proposed Work Breakdown

### Step 1. Add fzf Selection Target

- `selected="$(printf "alpha\nbeta\n" | fzf)"; printf "fzf-select:%s\n" "$selected"`를 실행한다.
- first follow-up input `b`를 보낸다.
- second follow-up input Enter를 보낸다.
- 선택 결과 marker `fzf-select:beta`가 출력되는지 확인한다.

완료 기준:

- local environment에서 `fzf-select` app target smoke가 통과한다.

### Step 2. Update Evidence

- `fzf` matrix row에 Enter selection workflow evidence를 연결한다.
- Environment-dependent App Smoke Target gap에서 `fzf` Enter selection 항목을 정리한다.

완료 기준:

- fzf filter, interactive redraw, Enter selection evidence가 문서에서 구분되어 있다.

## Non-goals

- `fzf`의 모든 key binding을 인증하지 않는다.
- multi-select, preview window, shell integration을 인증하지 않는다.
- GUI focus 기반 keyboard automation을 추가하지 않는다.

## Acceptance Criteria

- [done] `fzf-select` app target smoke가 추가되어 있다.
- [done] 설치된 환경에서 fzf Enter selection marker가 확인된다.
- [done] matrix와 known gaps가 갱신되어 있다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## Implementation Update - 2026-06-11

상태: fzf Enter selection workflow evidence 구현 완료.

구현된 내용:

- `scripts/run-app-target-smokes.sh`에 `fzf-select` target을 추가했다.
- app 내부 PTY에서 `fzf`에 query `b`와 Enter를 순서대로 보내 선택 결과 marker `fzf-select:beta`를 확인한다.
- compatibility matrix, smoke test 문서, known gaps, README를 갱신했다.

검증:

- `scripts/run-compatibility-core.sh`
- `cargo test`
- `scripts/run-app-smoke.sh`
- `scripts/run-app-command-smoke.sh`
- `scripts/run-app-target-smokes.sh`
