# Phase 038: htop App Runtime Snapshot Smoke

## Purpose

Phase 038의 목적은 `htop` version smoke를 넘어 native app 내부 PTY에서 full-screen `htop` redraw snapshot을 자동 확인하는 것이다.

Phase 034는 `htop --version`만 확인했다. 이 phase에서는 실제 `htop`을 실행하고 terminal buffer snapshot에서 full-screen runtime marker를 확인한다.

## Scope

Phase 038에서 다룰 작업:

1. `scripts/run-app-target-smokes.sh`에 `htop-runtime` target 추가
2. app 내부 PTY에서 `htop` 실행 후 snapshot marker 확인
3. matrix와 known gaps 갱신

## Proposed Work Breakdown

### Step 1. Add htop Runtime Target

- `htop` 절대 경로를 사용한다.
- snapshot marker는 `Load average`로 검증한다.
- target이 없으면 skip으로 기록한다.

완료 기준:

- local environment에서 `htop-runtime` app target smoke가 통과한다.

### Step 2. Update Remaining Gap

- `htop` redraw evidence를 matrix에 연결한다.
- function key/quit workflow는 known gap으로 유지한다.

완료 기준:

- full-screen redraw snapshot과 interactive key workflow가 문서에서 구분되어 있다.

## Non-goals

- `htop` function key workflow를 인증하지 않는다.
- GUI keyboard automation을 추가하지 않는다.
- mouse interaction을 인증하지 않는다.

## Acceptance Criteria

- `htop-runtime` app target smoke가 추가되어 있다. `done`
- 설치된 환경에서 `htop` full-screen snapshot marker가 확인된다. `done`
- matrix와 known gaps가 갱신되어 있다. `done`
- `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다. `done`

## Implementation Update - 2026-06-11

Status: implementation complete for htop runtime redraw snapshot evidence.

구현된 내용:

- `scripts/run-app-target-smokes.sh`에 `htop-runtime` target을 추가했다.
- target별 snapshot delay override를 지원하도록 smoke runner를 확장했다.
- local environment에서 app 내부 PTY로 `htop` full-screen redraw snapshot smoke가 통과했다.
- compatibility matrix, smoke test 문서, known gaps, README를 갱신했다.

검증:

- `scripts/run-app-target-smokes.sh`
