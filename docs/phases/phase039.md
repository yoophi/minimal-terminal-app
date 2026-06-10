# Phase 039: fzf Interactive App Smoke

## Purpose

Phase 039의 목적은 `fzf` non-interactive filter smoke를 넘어 native app 내부 PTY에서 interactive navigation key workflow를 자동 확인하는 것이다.

Phase 033은 `fzf --filter`만 확인했다. 이 phase에서는 `fzf` UI가 뜬 뒤 follow-up key input을 보내 selection 이동과 Enter workflow를 snapshot으로 검증한다.

## Scope

Phase 039에서 다룰 작업:

1. smoke harness에 follow-up input과 delay 추가
2. `scripts/run-app-target-smokes.sh`에 `fzf-interactive` target 추가
3. app 내부 PTY에서 `fzf` UI redraw와 selection movement marker 확인
4. matrix와 known gaps 갱신

## Proposed Work Breakdown

### Step 1. Add Follow-up Smoke Input

- 기존 `MINIMAL_TERMINAL_SMOKE_INPUT`은 command 시작에 사용한다.
- `MINIMAL_TERMINAL_SMOKE_FOLLOWUP_INPUT`은 별도 delay 후 PTY에 쓴다.

완료 기준:

- full-screen TUI가 그려진 뒤 key input을 보낼 수 있다.

### Step 2. Add fzf Interactive Target

- `printf 'alpha\nbeta\n' | fzf` 실행 후 Down/Enter를 보낸다.
- snapshot에서 `beta`와 fzf UI marker를 확인한다.

완료 기준:

- local environment에서 `fzf-interactive` app target smoke가 통과한다.

## Non-goals

- 모든 fzf key binding을 인증하지 않는다.
- mouse interaction을 인증하지 않는다.
- GUI focus 기반 keyboard automation을 추가하지 않는다.

## Acceptance Criteria

- smoke harness가 follow-up input을 지원한다. `done`
- `fzf-interactive` app target smoke가 추가되어 있다. `done`
- 설치된 환경에서 fzf UI redraw와 navigation marker가 확인된다. `done`
- matrix와 known gaps가 갱신되어 있다. `done`
- `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다. `done`

## Implementation Update - 2026-06-11

Status: implementation complete for fzf interactive query redraw evidence.

구현된 내용:

- `MINIMAL_TERMINAL_SMOKE_FOLLOWUP_INPUT`과 `MINIMAL_TERMINAL_SMOKE_FOLLOWUP_INPUT_DELAY_MS`를 추가했다.
- `scripts/run-app-target-smokes.sh`에 follow-up input case helper를 추가했다.
- `fzf-interactive` target을 추가해 fzf UI가 뜬 뒤 query input `b`를 보내고 `beta` redraw marker를 확인한다.
- compatibility matrix, smoke test 문서, known gaps, README를 갱신했다.

검증:

- `scripts/run-app-target-smokes.sh`
