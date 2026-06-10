# Phase 046: fzf Multi-select App Smoke

## Purpose

Phase 046의 목적은 app 내부 PTY에서 `fzf -m` multi-select workflow가 query, Tab selection, Enter accept 순서로 동작하는지 자동 확인하는 것이다.

Phase 043은 단일 Enter selection을 확인했다. 이 phase에서는 multi-select mode에서 selection toggle key path까지 smoke coverage를 넓힌다.

## Scope

Phase 046에서 다룰 작업:

1. `scripts/run-app-target-smokes.sh`에 `fzf-multi-select` target 추가
2. app 내부 PTY에서 `printf "alpha\nbeta\n" | fzf -m` 실행
3. follow-up `b` 입력 후 두 번째 follow-up Tab+Enter 입력
4. shell marker `fzf-multi:beta` 확인
5. matrix와 known gaps 갱신

## Proposed Work Breakdown

### Step 1. Add fzf Multi-select Target

- `selected="$(printf "alpha\nbeta\n" | fzf -m)"; printf "fzf-multi:%s\n" "$selected"`를 실행한다.
- first follow-up input `b`를 보낸다.
- second follow-up input Tab+Enter를 보낸다.
- 선택 결과 marker `fzf-multi:beta`가 출력되는지 확인한다.

완료 기준:

- local environment에서 `fzf-multi-select` app target smoke가 통과한다.

### Step 2. Update Evidence

- `fzf` matrix row에 multi-select workflow evidence를 연결한다.
- Environment-dependent App Smoke Target gap에서 `fzf` multi-select 항목을 정리한다.

완료 기준:

- fzf filter, interactive redraw, Enter selection, multi-select evidence가 문서에서 구분되어 있다.

## Non-goals

- `fzf`의 모든 key binding을 인증하지 않는다.
- preview window와 shell integration을 인증하지 않는다.
- GUI focus 기반 keyboard automation을 추가하지 않는다.

## Acceptance Criteria

- [done] `fzf-multi-select` app target smoke가 추가되어 있다.
- [done] 설치된 환경에서 fzf multi-select marker가 확인된다.
- [done] matrix와 known gaps가 갱신되어 있다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## Implementation Update - 2026-06-11

상태: fzf multi-select workflow evidence 구현 완료.

구현된 내용:

- `scripts/run-app-target-smokes.sh`에 `fzf-multi-select` target을 추가했다.
- app 내부 PTY에서 `fzf -m`에 query `b`, Tab, Enter를 순서대로 보내 선택 결과 marker `fzf-multi:beta`를 확인한다.
- compatibility matrix, smoke test 문서, known gaps, README를 갱신했다.

검증:

- `scripts/run-compatibility-core.sh`
- `cargo test`
- `scripts/run-app-smoke.sh`
- `scripts/run-app-command-smoke.sh`
- `scripts/run-app-target-smokes.sh`
