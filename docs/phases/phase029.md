# Phase 029: Tab and Escape Key Input

## Purpose

Phase 029의 목적은 AppKit text input path로 빠져 PTY에 전달되지 않던 Tab과 Escape key 입력을 직접 control byte로 전달하는 것이다.

`BUGS.md`에 기록된 "tab key 입력되지 않음", "esc key 입력되지 않음" 증상은 TUI와 shell 사용성에 직접 영향을 준다. 이 phase에서는 Tab/Esc를 direct key encoding path로 고정한다.

## Scope

Phase 029에서 다룰 작업:

1. Tab keycode를 `TAB` byte로 인코딩
2. Escape keycode를 `ESC` byte로 인코딩
3. TerminalView text input 우회 목록에 Tab/Esc 추가
4. input test와 compatibility 문서 갱신

## Proposed Work Breakdown

### Step 1. Encode Tab/Esc

- Tab: `0x09`
- Escape: `0x1b`

완료 기준:

- input unit test가 Tab/Esc encoding을 검증한다.

### Step 2. Avoid NSTextInputClient Command Routing

- Tab/Esc는 IME/text composition 경로로 보내지 않는다.
- 기존 Return, Backspace, navigation key 처리와 같은 direct key path를 사용한다.

완료 기준:

- `should_use_text_input`이 Tab/Esc를 제외한다.

## Non-goals

- 모든 AppKit command selector를 구현하지 않는다.
- Option/Command 기반 application shortcut 정책을 변경하지 않는다.
- GUI 수동 입력 smoke를 자동화하지 않는다.

## Risks

### IME/Text Input Regression

Tab/Esc를 text input path에서 제외하면 composition command와 충돌할 수 있다.

대응:

- printable text와 IME 입력은 기존 text input path를 유지한다.
- Tab/Esc 두 key만 direct control byte로 보낸다.

## Acceptance Criteria

- Tab key가 `0x09`로 인코딩된다. `done`
- Escape key가 `0x1b`로 인코딩된다. `done`
- Tab/Esc가 text input path를 우회한다. `done`
- matrix와 README가 갱신되어 있다. `done`
- `scripts/run-compatibility-core.sh`와 `cargo test`가 통과한다. `done`

## Implementation Update - 2026-06-11

Status: implementation complete.

구현된 내용:

- `input.rs`에 Tab/Esc keycode encoding을 추가했다.
- `TerminalView::should_use_text_input`에서 Tab/Esc를 제외했다.
- `input::tests::encodes_tab_and_escape_as_control_bytes`를 추가했다.
- compatibility matrix와 README를 갱신했다.

검증:

- `cargo test -p terminal-app encodes_tab_and_escape_as_control_bytes`
- `scripts/run-compatibility-core.sh`
- `cargo test`
