# Phase 003: Terminal Core, Cursor, and Input Semantics

## Purpose

Phase 003의 목적은 Phase 002에서 만든 "PTY to screen pipeline"을 실제 터미널 에뮬레이터 구조로 발전시키는 것이다.

Phase 002는 login shell output을 화면에 표시하는 최소 경로를 검증했다. 그러나 현재 구조는 문자열 블록 렌더링에 가깝고, cursor row/column, terminal grid, ANSI/VT state가 없다. 따라서 입력 커서, 특수키, line editing, TUI 앱을 안정적으로 처리하려면 terminal core가 먼저 필요하다.

## Scope

Phase 003에서 다룰 작업은 다음과 같다.

- `terminal-core` crate 생성
- terminal grid/cell/cursor 모델 구현
- ANSI/VT escape sequence parser 도입
- shell output을 grid update로 변환
- cursor position 렌더링
- keyboard input encoder 구현
- Backspace, Enter, Ctrl-C, arrow keys 처리
- 창 크기 변경 시 rows/cols 계산
- PTY resize 전달
- scrollback model 초안

## Why Cursor Belongs Here

입력 커서는 단순히 문자열 끝에 그리는 UI 장식이 아니다. 터미널에서 cursor는 terminal state의 일부다.

cursor 위치는 다음 이벤트에 의해 바뀐다.

- printable text 출력
- carriage return
- newline
- backspace
- ANSI cursor movement
- clear line/screen
- shell line editing
- prompt redraw
- terminal resize
- TUI application control sequence

Phase 002의 `TerminalBuffer`는 문자열 블록만 가지고 있기 때문에 cursor row/column을 안정적으로 알 수 없다. 이 상태에서 커서를 표시하면 실제 shell 상태와 화면 cursor가 어긋날 가능성이 높다.

따라서 cursor는 Phase 003에서 terminal grid와 함께 구현한다.

## Input Semantics

Phase 002의 입력 처리는 `event.characters()`를 PTY로 전달하는 수준이다. printable text와 Enter 확인에는 충분할 수 있지만, 터미널 앱으로는 부족하다.

Phase 003에서는 input encoder를 별도 책임으로 둔다.

예상 처리 대상:

- printable UTF-8 text
- Enter
- Backspace/Delete
- Tab
- Ctrl-C
- Ctrl-D
- arrow keys
- Home/End
- PageUp/PageDown
- Option/Command 조합
- IME composition

이 작업은 PTY writer에 직접 붙이기보다 `terminal-app/input.rs` 또는 별도 input adapter로 분리하는 것이 좋다.

## Proposed Module Shape

초기 구조:

```text
crates/
├── terminal-core/
│   └── src/
│       ├── cell.rs
│       ├── cursor.rs
│       ├── grid.rs
│       ├── parser.rs
│       ├── scrollback.rs
│       ├── state.rs
│       └── lib.rs
└── terminal-app/
    └── src/
        ├── input.rs
        ├── terminal_view.rs
        └── pty.rs
```

`terminal-core`는 AppKit, PTY, macOS API를 몰라야 한다.

## Acceptance Criteria

Phase 003의 최소 완료 기준:

- shell prompt가 grid 기반으로 표시된다.
- printable text 입력 후 cursor가 예상 위치로 이동한다.
- Backspace가 화면과 shell 양쪽에서 일관되게 동작한다.
- Enter 입력 후 새 prompt가 정상적으로 표시된다.
- Ctrl-C가 foreground shell process로 전달된다.
- arrow key escape sequence가 shell line editor에 전달된다.
- window resize 시 PTY size와 grid size가 동기화된다.
- `cargo test`로 terminal-core parser/grid/cursor 기본 동작을 검증한다.

## Non-goals

Phase 003에서 하지 않을 수 있는 작업:

- 완전한 xterm 호환성
- full-screen TUI 앱 완전 지원
- GPU renderer
- selection/copy/paste 완성
- theme system
- tabs/splits

## Notes from Phase 002

Phase 002에서 확인한 중요한 교훈:

- PTY reader는 UI를 직접 만지면 안 된다.
- AppKit drawing은 실제 런타임 검증이 필요하다.
- process 기준 log filter는 시스템 로그가 너무 많이 섞인다.
- subsystem/category 기반 logging은 디버깅에 유용하다.
- 문자열 블록 렌더링은 빠르게 확인하기 좋지만, cursor와 ANSI를 정확히 처리하기에는 부족하다.

