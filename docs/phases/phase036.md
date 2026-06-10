# Phase 036: vttest App Runtime Menu Smoke

## Purpose

Phase 036의 목적은 `vttest` 시작 메뉴를 native app 내부 PTY에서 실제로 실행해 runtime snapshot evidence를 확보하는 것이다.

Phase 028은 vttest 시작 메뉴 replay fixture를 core test로 고정했다. 이 phase에서는 replay가 아니라 app 내부 login shell에서 `/opt/homebrew/bin/vttest`를 실행하고, terminal buffer snapshot에서 시작 메뉴 marker를 확인한다.

## Scope

Phase 036에서 다룰 작업:

1. `scripts/run-app-target-smokes.sh`에 `vttest-menu` target 추가
2. `vttest`가 설치된 경우 app-internal snapshot에서 시작 메뉴 marker 확인
3. matrix와 known gaps 갱신

## Proposed Work Breakdown

### Step 1. Add vttest Runtime Target

- `vttest` 절대 경로를 사용해 PATH 차이를 줄인다.
- snapshot marker는 `VT100 test program`으로 검증한다.
- target이 없으면 skip으로 기록한다.

완료 기준:

- local environment에서 `vttest-menu` app target smoke가 통과한다.

### Step 2. Update Gap Status

- vttest 시작 메뉴 runtime evidence를 matrix에 연결한다.
- full interactive menu result는 known gap으로 유지한다.

완료 기준:

- replay evidence와 app runtime menu evidence가 구분되어 있다.

## Non-goals

- vttest 전체 menu suite를 자동 실행하지 않는다.
- cursor/erase/scrolling/reporting/charset 세부 실패를 이 phase에서 모두 분해하지 않는다.
- GUI keyboard automation을 추가하지 않는다.

## Acceptance Criteria

- `vttest-menu` app target smoke가 추가되어 있다. `done`
- 설치된 환경에서 vttest 시작 메뉴가 app 내부 PTY snapshot으로 검증된다. `done`
- matrix와 known gaps가 갱신되어 있다. `done`
- `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다. `done`

## Implementation Update - 2026-06-11

Status: implementation complete for vttest start menu runtime evidence.

구현된 내용:

- `scripts/run-app-target-smokes.sh`에 `vttest-menu` target을 추가했다.
- local environment에서 app 내부 PTY로 `vttest` 시작 메뉴 snapshot smoke가 통과했다.
- compatibility matrix, smoke test 문서, known gaps, README를 갱신했다.

검증:

- `scripts/run-app-target-smokes.sh`
