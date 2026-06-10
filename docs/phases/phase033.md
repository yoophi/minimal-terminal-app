# Phase 033: App Target Command Smoke

## Purpose

Phase 033의 목적은 Phase 032 smoke harness를 사용해 로컬에 있는 대표 command target을 native app 내부 PTY에서 자동 확인하는 것이다.

Phase 017과 Phase 019는 `fzf`, `git log`를 command-level로 확인했지만 앱 내부 PTY를 통한 snapshot evidence는 없었다. 이 phase에서는 non-interactive target부터 자동화한다.

## Scope

Phase 033에서 다룰 작업:

1. app 내부 `fzf --filter` smoke 추가
2. app 내부 `git log --oneline` smoke 추가
3. target이 설치되어 있지 않으면 명시적으로 skip
4. matrix와 known gaps 갱신

## Proposed Work Breakdown

### Step 1. Add Target Script

- `scripts/run-app-target-smokes.sh`를 추가한다.
- Phase 032 smoke env를 사용해 target별 command를 앱 내부 PTY에 주입한다.
- snapshot에서 expected marker를 확인한다.

완료 기준:

- local 환경에서 `fzf`와 `git` target smoke가 통과하거나, 미설치 target은 skip으로 기록된다.

### Step 2. Update Evidence

- `matrix.md`의 `fzf`, `git log` 근거를 app-internal smoke로 갱신한다.
- interactive navigation/pager workflow는 known gap으로 유지한다.

완료 기준:

- supported/partial 표기가 실제 evidence 범위와 일치한다.

## Non-goals

- `fzf` interactive navigation을 인증하지 않는다.
- `git` pager scroll/quit workflow를 인증하지 않는다.
- `htop` full-screen redraw와 function key workflow를 인증하지 않는다.

## Acceptance Criteria

- app target smoke script가 추가되어 있다. `done`
- `fzf --filter`가 설치된 환경에서 앱 내부 PTY snapshot으로 검증된다. `done`
- `git log --oneline`이 앱 내부 PTY snapshot으로 검증된다. `done`
- matrix와 known gaps가 갱신되어 있다. `done`
- `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다. `done`

## Implementation Update - 2026-06-11

Status: implementation complete.

구현된 내용:

- `scripts/run-app-target-smokes.sh`를 추가했다.
- local environment에서 `fzf-filter`와 `git-log` app-internal snapshot smoke가 통과했다.
- compatibility matrix, smoke test 문서, known gaps, README를 갱신했다.

검증:

- `scripts/run-app-target-smokes.sh`
