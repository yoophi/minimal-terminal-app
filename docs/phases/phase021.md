# Phase 021: Function and Modified Key Encoding

## Purpose

Phase 021의 목적은 대표 TUI가 기대하는 function key와 modifier-aware key sequence gap을 줄이는 것이다.

Phase 019 app readiness에서 `vim`, `tmux`, agent-style CLI의 key encoding risk가 남아 있었다. 이 phase에서는 앱 입력 레이어에서 자동 테스트 가능한 F1-F12와 Shift/Option/Control modifier navigation/function key encoding을 구현한다.

## Scope

Phase 021에서 다룰 작업:

1. F1-F12 key encoding 추가
2. Shift/Option/Control modifier가 붙은 navigation/function key encoding 추가
3. 기존 shell 편집 UX에 쓰는 Option 단독 word navigation 유지
4. input unit test와 compatibility 문서 갱신

## Proposed Work Breakdown

### Step 1. Encode Function Keys

- F1-F4는 SS3 sequence를 사용한다.
- F5-F12는 tilde CSI sequence를 사용한다.

완료 기준:

- input test가 F1, F2, F5, F12 encoding을 검증한다.

### Step 2. Encode Modified Keys

- Shift/Option/Control modifier parameter를 xterm-style CSI parameter로 계산한다.
- modified arrow, delete, page, function key를 검증한다.

완료 기준:

- input test가 modified navigation/function key encoding을 검증한다.

### Step 3. Preserve Existing Shell UX

- unmodified arrow key는 기존 shell line-editor control byte를 유지한다.
- Option 단독 Left/Right/Backspace word navigation을 유지한다.
- Command 조합은 앱 shortcut을 위해 계속 예약한다.

완료 기준:

- 기존 input test가 그대로 통과한다.

## Non-goals

- application keypad mode와 numeric keypad 전체를 구현하지 않는다.
- macOS 키보드 레이아웃별 모든 modifier variant를 보증하지 않는다.
- `vim`, `tmux`, `emacs` 전체 workflow 성공을 이 phase에서 선언하지 않는다.

## Risks

### Shell Editing Regression

기존 unmodified arrow와 Option word navigation은 shell 사용성에 직접 영향을 준다.

대응:

- 기존 shell 편집 sequence는 유지한다.
- modifier가 붙은 special key에만 xterm-style sequence를 추가한다.

## Acceptance Criteria

- F1-F12 key encoding test가 있다. `done`
- Shift/Option/Control modified navigation/function key encoding test가 있다. `done`
- 기존 Option word navigation test가 통과한다. `done`
- matrix와 known gaps가 갱신되어 있다. `done`
- `scripts/run-compatibility-core.sh`와 `cargo test`가 통과한다. `done`

## Implementation Update - 2026-06-11

Status: implementation complete for function keys and selected modifier-aware key encoding.

구현된 내용:

- F1-F12 encoding을 추가했다.
- Shift/Option/Control modifier parameter 계산을 추가했다.
- modified navigation/function key encoding을 추가했다.
- 기존 Option 단독 Left/Right/Backspace word navigation은 유지했다.
- matrix를 `partially supported`로 갱신하고 application keypad와 full modifier coverage를 known gap으로 남겼다.

검증:

- `cargo test -p terminal-app input::tests`
- `scripts/run-compatibility-core.sh`
- `cargo test`
