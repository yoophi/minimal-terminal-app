# Phase 013: Mouse Reporting

## Purpose

Phase 013의 목적은 TUI 프로그램이 요청하는 mouse reporting mode를 처리하고, AppKit mouse event를 terminal mouse report로 encode해 PTY로 전달하는 것이다.

현재 앱의 mouse drag는 native selection에만 쓰인다. `vim`, `less`, multiplexer 등은 mouse reporting이 켜져 있으면 mouse press/release/wheel event를 PTY로 받기를 기대할 수 있다.

## Scope

Phase 013에서 다룰 작업:

1. 지원할 mouse mode 범위 정의
2. parser에서 mouse mode enable/disable 처리
3. `TerminalModes`에 mouse reporting state 추가
4. AppKit mouse event를 SGR mouse report로 encode
5. native selection과 TUI mouse reporting 충돌 정책 정의
6. test와 smoke 추가

## Proposed Work Breakdown

### Step 1. Define Mouse Mode Scope

우선순위:

1. SGR mouse encoding
2. press/release
3. wheel
4. drag

완료 기준:

- 첫 구현 범위와 후속 범위가 문서화되어 있다.

### Step 2. Parse Mouse Modes

- DEC private mouse mode sequence를 parser에서 처리한다.
- 지원하지 않는 mode는 safely ignored 또는 known gap으로 기록한다.

완료 기준:

- parser/state test가 mouse mode on/off를 검증한다.

### Step 3. Encode AppKit Mouse Events

- mouseDown/mouseUp/scrollWheel을 terminal coordinate로 변환한다.
- active mouse reporting mode에서 PTY report byte를 생성한다.
- SGR mouse format을 우선 적용한다.

완료 기준:

- mouse report encoding unit test가 있다.

### Step 4. Conflict Policy

- mouse reporting off: native selection
- mouse reporting on: PTY mouse report
- native selection override modifier가 필요한지 검토

완료 기준:

- 정책이 문서와 코드에 반영되어 있다.

## Non-goals

- 모든 legacy mouse encoding을 한 번에 지원하지 않는다.
- pixel-perfect mouse hit testing을 목표로 하지 않는다.

## Risks

### Selection Regression

mouse reporting이 native selection을 깨뜨릴 수 있다.

대응:

- mouse mode가 꺼져 있을 때 기존 selection test와 smoke를 유지한다.

## Acceptance Criteria

- mouse mode parser/state test가 있다. `done`
- mouse report encoding test가 있다. `done`
- AppKit mouse event가 active mouse mode에서 PTY로 전달된다. `done`
- native selection 기본 동작이 유지된다. `done`
- `vim` 또는 `less` mouse smoke 결과가 기록되어 있다. `deferred`
- matrix와 known gaps가 갱신되어 있다. `done`

## Implementation Update - 2026-06-10

Status: implementation complete for SGR mouse reporting. Runtime app smoke evidence remains tracked as a known gap.

구현된 내용:

- parser가 DEC private mouse mode `?1000`, `?1002`, `?1003`, `?1006` on/off를 인식한다.
- `TerminalModes`에 mouse reporting과 SGR mouse 상태를 추가했다.
- SGR mouse press, release, drag, wheel encoding helper와 unit test를 추가했다.
- AppKit mouseDown, mouseDragged, mouseUp, scrollWheel이 active SGR mouse mode에서 PTY report를 보낸다.
- SGR mouse reporting이 꺼져 있으면 기존 native selection/scrollback 동작을 유지한다.
- matrix와 known gaps를 갱신했다.

검증:

- `scripts/run-compatibility-core.sh`
- `cargo test`
