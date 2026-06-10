# Phase 024: Legacy Mouse Encoding

## Purpose

Phase 024의 목적은 SGR mouse mode가 켜져 있지 않은 TUI도 mouse reporting을 받을 수 있도록 legacy X10-style mouse encoding을 추가하는 것이다.

Phase 013에서 SGR mouse reporting을 구현했지만 legacy mouse encoding은 gap으로 남아 있었다. 이 phase에서는 기존 mouse mode tracking을 유지하면서 app input layer에서 SGR 또는 legacy encoding을 선택한다.

## Scope

Phase 024에서 다룰 작업:

1. legacy mouse report encoder 추가
2. SGR mouse mode 여부에 따라 SGR/legacy report 선택
3. press/release/drag/wheel encoding test 추가
4. compatibility 문서 갱신

## Proposed Work Breakdown

### Step 1. Encode Legacy Mouse Reports

- report prefix는 `ESC [ M`이다.
- code, column, row는 classic mouse byte encoding을 사용한다.
- release는 legacy release code를 사용한다.

완료 기준:

- mouse unit test가 press, release, wheel legacy encoding을 검증한다.

### Step 2. Select Encoding in App Layer

- `mouse_reporting`이 꺼져 있으면 native selection/scrollback을 유지한다.
- `mouse_reporting`이 켜져 있고 `sgr_mouse`가 켜져 있으면 SGR report를 보낸다.
- `mouse_reporting`이 켜져 있고 `sgr_mouse`가 꺼져 있으면 legacy report를 보낸다.

완료 기준:

- app input layer가 두 encoding path를 모두 사용할 수 있다.

## Non-goals

- modifier-aware mouse report를 구현하지 않는다.
- 모든 xterm mouse mode variant를 구현하지 않는다.
- `vim`/`less` mouse runtime smoke 통과를 이 phase에서 선언하지 않는다.

## Risks

### Native Selection Regression

mouse reporting이 꺼져 있을 때는 앱 native selection이 계속 동작해야 한다.

대응:

- 기존처럼 `mouse_reporting`이 꺼져 있으면 mouse report를 보내지 않는다.
- legacy fallback은 `mouse_reporting`이 켜져 있는 경우에만 사용한다.

## Acceptance Criteria

- legacy mouse press/release/wheel encoding test가 있다. `done`
- TerminalView가 SGR/legacy mouse report를 mode에 따라 선택한다. `done`
- matrix와 known gaps가 갱신되어 있다. `done`
- `scripts/run-compatibility-core.sh`와 `cargo test`가 통과한다. `done`

## Implementation Update - 2026-06-11

Status: implementation complete for legacy mouse encoding.

구현된 내용:

- `legacy_mouse_report` encoder를 추가했다.
- TerminalView가 `mouse_reporting`과 `sgr_mouse` mode에 따라 SGR 또는 legacy report를 선택한다.
- legacy mouse press, release, wheel test를 추가했다.
- matrix와 known gaps를 갱신했다.

검증:

- `cargo test -p terminal-app mouse::tests`
- `scripts/run-compatibility-core.sh`
- `cargo test`
