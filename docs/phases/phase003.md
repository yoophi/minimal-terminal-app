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

## Progress

2026-06-10에 Phase 003의 첫 수직 조각을 구현했다.

완료한 작업:

- `terminal-core` crate 생성
- AppKit/PTY와 분리된 순수 Rust terminal state 추가
- grid/cell/cursor 기본 모델 추가
- 최소 ANSI/VT CSI parser 추가
- `terminal-app`의 `TerminalBuffer`를 `terminal-core::TerminalState` wrapper로 전환
- `TerminalView`가 `TerminalSnapshot`을 렌더링하도록 변경
- cursor 위치에 흰색 block cursor 렌더링
- terminal-core 단위 테스트 추가
- 실제 앱 재실행 후 grid 기반 출력과 block cursor 표시 확인
- `terminal-app/src/input.rs` input encoder 추가
- Enter, Backspace, Delete, Ctrl-C, Ctrl-D, arrow key, Home/End, PageUp/PageDown byte sequence 분리
- AppKit view bounds에서 rows/cols 계산
- terminal-core resize API 추가
- `TIOCSWINSZ`를 통한 PTY resize 전달
- scrollback 기본 저장 모델 추가

현재 `terminal-core`가 처리하는 동작:

- printable text
- carriage return
- newline
- backspace/delete
- tab
- 기본 scrolling
- CSI cursor movement: `A`, `B`, `C`, `D`, `G`, `H`, `f`
- CSI clear line: `K`
- CSI clear screen: `2J`
- SGR style sequence `m` 무시
- resize 시 visible grid 보존 및 cursor clamp
- scroll 시 overflow line을 scrollback에 저장

이번 단계의 의도는 완전한 terminal emulator를 만드는 것이 아니라, Phase 002의 문자열 버퍼를 테스트 가능한 grid/cursor 기반으로 교체하는 것이다.

런타임 검증:

```text
echo phase003-grid; echo cursor-check
phase003-grid
cursor-check
```

확인된 상태:

- shell prompt가 grid 기반 snapshot으로 표시된다.
- paste한 command echo가 화면에 표시된다.
- Enter 후 command output과 새 prompt가 표시된다.
- cursor 위치에 block cursor가 표시된다.
- input encoder 단위 테스트로 Enter, Backspace, arrow key byte sequence를 검증했다.
- Backspace 런타임 검증: `echo backspaceX` 입력 후 Backspace로 `X`를 지우고 `backspace` 출력 확인.
- Ctrl-C 런타임 검증: `sleep 5` 실행 중 `Ctrl-C` 입력 시 `^C`, `INT`, 새 prompt 표시 확인.
- terminal-core 단위 테스트로 resize와 scrollback 길이를 검증했다.
- view resize 시 grid rows/cols와 PTY window size가 함께 갱신된다.

남은 표시 문제:

- zsh prompt redraw 과정에서 command line이 한 번 중복되어 보일 수 있다.
- prompt 오른쪽 정렬 영역과 cursor movement sequence는 아직 완전히 해석하지 못한다.
- OSC/charset escape는 skip하도록 보강했지만, ANSI/VT parser 범위는 여전히 제한적이다.

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

- printable UTF-8 text `done`
- Enter `done`
- Backspace/Delete `done`
- Tab
- Ctrl-C `done`
- Ctrl-D `done`
- arrow keys `done`
- Home/End `done`
- PageUp/PageDown `done`
- Option/Command 조합
- IME composition

이 작업은 PTY writer에 직접 붙이기보다 `terminal-app/input.rs` 또는 별도 input adapter로 분리하는 것이 좋다. 현재는 `terminal-app/src/input.rs`에 최소 input encoder를 추가했다.

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

- shell prompt가 grid 기반으로 표시된다. `done`
- printable text 입력 후 cursor가 예상 위치로 이동한다. `done`
- Backspace가 화면과 shell 양쪽에서 일관되게 동작한다. `done`
- Enter 입력 후 새 prompt가 정상적으로 표시된다. `done`
- Ctrl-C가 foreground shell process로 전달된다. `done`
- arrow key escape sequence가 shell line editor에 전달된다. `implemented`
- window resize 시 PTY size와 grid size가 동기화된다. `done`
- `cargo test`로 terminal-core parser/grid/cursor 기본 동작을 검증한다. `done`

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

## Remaining Work

다음 작업은 Phase 003 안에서 계속 진행한다.

- arrow key, Home/End, PageUp/PageDown의 실제 login shell 런타임 검증
- Tab, Option/Command 조합, IME composition 처리 방침 결정
- scrollback UI 노출 방식 결정
- Unicode width 처리 방침 결정
- ANSI parser 범위 확장

## Prioritization Notes

Phase 003 잔여 작업은 다음 순서로 처리했다.

1. Input encoder: 사용자가 가장 먼저 체감하는 기본 조작이며, Backspace/Ctrl-C/arrow key는 shell 사용의 최소 조건이다.
2. Resize synchronization: terminal grid와 PTY가 다른 크기를 사용하면 prompt redraw와 line editing이 계속 어긋난다.
3. Scrollback model: UI 노출 전이라도 core가 scroll된 line을 잃지 않는 구조가 필요하다.
4. Unicode width and ANSI expansion: 정확한 렌더링 품질에 중요하지만 parser/grid 정책에 더 큰 영향을 주므로 다음 반복에서 별도로 다룬다.
