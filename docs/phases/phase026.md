# Phase 026: Modifier-aware Mouse Reporting

## Purpose

Phase 026의 목적은 mouse reporting에서 Shift, Option, Control modifier를 legacy 및 SGR mouse report에 반영하는 것이다.

Phase 024에서 legacy mouse encoding을 구현했지만 modifier-aware report는 gap으로 남아 있었다. 이 phase에서는 macOS modifier flag를 xterm mouse modifier bit로 변환해 mouse press/release/drag/wheel report code에 합성한다.

## Scope

Phase 026에서 다룰 작업:

1. SGR mouse report에 modifier bit 반영
2. legacy mouse report에 modifier bit 반영
3. AppKit `NSEventModifierFlags`를 xterm mouse modifier bit로 변환
4. unit test와 compatibility 문서 갱신

## Proposed Work Breakdown

### Step 1. Extend Mouse Encoders

- Shift: `4`
- Option: Meta bit `8`
- Control: `16`

완료 기준:

- SGR와 legacy mouse encoder test가 modifier bit를 검증한다.

### Step 2. Wire AppKit Flags

- mouse event에서 modifier flag를 읽는다.
- `Option`은 xterm Meta modifier bit로 인코딩한다.

완료 기준:

- TerminalView helper test가 AppKit flag를 xterm modifier bit로 변환하는지 검증한다.

## Non-goals

- `vim`/`less` runtime mouse workflow 통과를 이 phase에서 선언하지 않는다.
- 모든 platform-specific modifier behavior를 보증하지 않는다.
- 새로운 mouse mode family를 추가하지 않는다.

## Risks

### Modifier Semantics Mismatch

macOS Option key는 텍스트 입력에서는 Meta/Option 정책과 충돌할 수 있다.

대응:

- mouse report에서만 Option을 xterm Meta bit로 사용한다.
- keyboard Option word navigation 정책은 변경하지 않는다.

## Acceptance Criteria

- SGR mouse modifier encoding test가 있다. `done`
- legacy mouse modifier encoding test가 있다. `done`
- AppKit modifier flag mapping test가 있다. `done`
- matrix와 known gaps가 갱신되어 있다. `done`
- `scripts/run-compatibility-core.sh`와 `cargo test`가 통과한다. `done`

## Implementation Update - 2026-06-11

Status: implementation complete for modifier-aware mouse reports.

구현된 내용:

- `sgr_mouse_report`와 `legacy_mouse_report`가 modifier mask를 받도록 확장했다.
- Shift/Option/Control modifier constants를 추가했다.
- TerminalView가 mouse event modifier flag를 xterm mouse modifier bit로 변환한다.
- modifier-aware mouse tests와 compatibility 문서를 갱신했다.

검증:

- `cargo test -p terminal-app`
- `scripts/run-compatibility-core.sh`
- `cargo test`
