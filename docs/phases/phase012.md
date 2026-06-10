# Phase 012: Cursor Style Sequences

## Purpose

Phase 012의 목적은 cursor shape/style sequence를 처리하고 AppKit renderer에 반영하는 것이다.

현재 cursor renderer는 고정 block cursor만 그린다. editor와 TUI는 `CSI Ps SP q` 같은 sequence로 block, bar, underline, blinking/steady cursor를 요청할 수 있다.

## Scope

Phase 012에서 다룰 작업:

1. cursor style model 정의
2. parser에서 `CSI Ps SP q` 처리
3. `TerminalModes` 또는 별도 cursor mode에 style 저장
4. AppKit cursor renderer 갱신
5. parser/state test와 runtime smoke 추가

## Proposed Work Breakdown

### Step 1. Model Cursor Style

- block
- bar
- underline
- blinking 여부
- steady 여부

완료 기준:

- style enum 또는 struct가 정의되어 있다.

### Step 2. Parse Cursor Style

- `CSI Ps SP q` sequence를 인식한다.
- xterm cursor style parameter mapping을 문서화한다.
- 알 수 없는 parameter는 안전하게 무시하거나 기본값으로 처리한다.

완료 기준:

- parser test가 주요 style parameter를 검증한다.

### Step 3. Render Cursor Shape

- block cursor
- vertical bar cursor
- underline cursor
- blinking은 MVP에서는 steady로 처리하거나 non-goal로 명확히 둔다.

완료 기준:

- AppKit renderer가 mode에 따라 cursor rect를 다르게 그린다.

## Non-goals

- 정밀한 blink timer 구현은 후속 작업으로 분리할 수 있다.
- 모든 terminal-specific cursor style extension을 구현하지 않는다.

## Risks

### Visual Regression

cursor shape 변경이 기존 block cursor 표시를 깨뜨릴 수 있다.

대응:

- 기본값은 현재 block cursor와 동일하게 유지한다.
- runtime smoke에서 shell input cursor를 확인한다.

## Acceptance Criteria

- cursor style parser/state test가 있다.
- renderer가 최소 block/bar/underline을 구분한다.
- `docs/compatibility/matrix.md`의 Cursor style row가 갱신되어 있다.
- `docs/compatibility/known-gaps.md`의 cursor style gap이 제거되거나 제한이 명확히 낮춰져 있다.
- `cargo test`와 `scripts/run-compatibility-core.sh`가 통과한다.

