# Phase 022: Application Keypad Mode

## Purpose

Phase 022의 목적은 application keypad mode와 numeric keypad input gap을 줄이는 것이다.

Phase 021에서 function key와 일부 modifier-aware key encoding을 구현했지만, application keypad mode는 아직 남아 있었다. `tmux`, `vim`, `vttest` 같은 TUI는 keypad mode를 사용할 수 있으므로 core mode와 app input encoding을 함께 구현한다.

## Scope

Phase 022에서 다룰 작업:

1. `ESC =`, `ESC >` parser action 추가
2. `TerminalModes`에 application keypad mode 추가
3. app input layer에 keypad SS3 encoding 추가
4. compatibility evidence와 문서 갱신

## Proposed Work Breakdown

### Step 1. Parse Keypad Mode

- `ESC =`: application keypad on
- `ESC >`: application keypad off

완료 기준:

- parser test가 두 mode sequence를 검증한다.

### Step 2. Track Mode in Core

- `TerminalModes`가 application keypad 상태를 노출한다.
- alternate screen 진입 시 transient mode를 초기화한다.

완료 기준:

- state test와 compatibility test가 mode tracking을 검증한다.

### Step 3. Encode Keypad Input

- keypad 0-9
- keypad decimal
- keypad enter
- keypad plus/minus/multiply/divide/equals

완료 기준:

- app input test가 representative keypad encoding을 검증한다.

## Non-goals

- 모든 macOS keyboard layout의 keypad variant를 보증하지 않는다.
- keypad 관련 runtime app workflow 통과를 이 phase에서 선언하지 않는다.
- full xterm keyboard coverage 완료를 선언하지 않는다.

## Risks

### Normal Keypad Regression

application keypad mode가 꺼져 있을 때 숫자 keypad는 일반 입력처럼 동작해야 한다.

대응:

- application keypad mode가 켜진 경우에만 SS3 keypad encoding을 우선 적용한다.
- mode가 꺼진 경우 기존 `characters()` 기반 입력 경로를 유지한다.

## Acceptance Criteria

- `ESC =`, `ESC >` parser test가 있다. `done`
- `TerminalModes`가 application keypad mode를 추적한다. `done`
- app input layer가 representative keypad SS3 sequence를 보낸다. `done`
- matrix와 known gaps가 갱신되어 있다. `done`
- `scripts/run-compatibility-core.sh`와 `cargo test`가 통과한다. `done`

## Implementation Update - 2026-06-11

Status: implementation complete.

구현된 내용:

- `SetApplicationKeypad` parser action을 추가했다.
- `TerminalModes.application_keypad`를 추가했다.
- TerminalView가 application keypad mode에서 keypad input을 SS3 sequence로 보낸다.
- input test와 compatibility evidence를 추가했다.
- matrix와 known gaps를 갱신했다.

검증:

- `cargo test -p terminal-core application_keypad`
- `cargo test -p terminal-app encodes_application_keypad_keys_for_tui_modes`
- `scripts/run-compatibility-core.sh`
- `cargo test`
