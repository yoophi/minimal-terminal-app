# Phase 004: vte Adapter, Style Model, and ANSI Compatibility Foundation

## Purpose

Phase 004의 목적은 Phase 003에서 만든 hand-written ANSI/VT parser와 단색 grid renderer를 더 안정적인 터미널 호환성 기반으로 교체하는 것이다.

Phase 003는 terminal core, cursor, input, resize, scrollback의 최소 구조를 만들었다. 하지만 parser 범위가 제한적이고 SGR style sequence를 무시하기 때문에 실제 shell prompt, color output, TUI 앱을 충분히 표현하지 못한다.

Phase 004에서는 Rust 기반 오픈소스 parser인 `vte` crate를 도입하고, 현재 `terminal-core` 구조를 유지한 상태에서 parser adapter, style model, fixture 기반 테스트, AppKit style-aware rendering을 연결한다.

## Scope

Phase 004에서 다룰 핵심 작업은 다음과 같다.

1. `vte` crate 도입 `done`
2. 기존 hand-written parser를 `vte` adapter로 교체 `done`
3. SGR style model 추가 `done`
4. parser fixture/golden test 추가 `done`
5. AppKit style-aware rendering 연결 `done`

이 단계는 "ANSI/VT100 완전 준수"를 한 번에 달성하는 단계가 아니다. 대신 직접 만든 parser의 불확실성을 줄이고, 이후 xterm 호환성 확장을 반복 가능하게 만드는 기반 단계다.

## Why vte

터미널 parser는 직접 구현하기 쉬워 보이지만, 실제로는 escape state, CSI parameter, OSC, charset designation, DEC private mode, malformed sequence recovery 같은 세부 동작이 많다.

`vte` crate를 사용하는 이유:

- Rust 생태계에서 검증된 terminal escape parser를 재사용할 수 있다.
- parser state machine을 직접 유지하는 부담을 줄일 수 있다.
- `terminal-core`의 grid/state logic과 parser logic을 분리할 수 있다.
- 향후 xterm compatibility를 fixture 기반으로 점진 확장하기 쉽다.

`vte`는 "화면 상태 전체"를 대신 관리하는 라이브러리가 아니라, escape sequence를 해석해 callback으로 전달하는 parser 역할에 가깝다. 따라서 현재 구조에서는 `vte`를 `terminal-core` 내부 parser adapter로 쓰고, grid/cursor/scrollback/state는 우리 코드가 계속 소유하는 방식이 적절하다.

## Recommended Extra Work

질문에서 제안한 1, 2번 외에 함께 진행하기를 권장하는 작업은 다음이다.

### 1. Parser Adapter Boundary

`vte` callback을 `TerminalState`에 직접 붙이지 말고, 내부 action/event로 한 번 변환하는 adapter 경계를 둔다.

권장 형태:

```text
vte::Perform callback
        ↓
ParserAdapter
        ↓
TerminalAction
        ↓
TerminalState
```

이 경계를 두면 parser를 교체하거나 특정 sequence 동작을 테스트할 때 AppKit/PTY와 무관하게 검증할 수 있다.

### 2. Terminal Fixture Test Harness

단위 테스트만으로는 실제 terminal sequence 호환성을 검증하기 어렵다. Phase 004에서는 parser fixture/golden test를 같이 도입하는 것이 좋다.

예상 fixture:

- plain text
- newline/carriage return
- cursor movement
- clear line/screen
- SGR basic colors
- 256-color SGR
- truecolor SGR
- OSC skip
- malformed escape recovery
- alternate screen enter/exit

fixture는 input byte와 expected grid snapshot을 함께 기록한다. 이렇게 해야 나중에 parser를 확장할 때 prompt redraw, color, cursor 이동이 깨졌는지 빠르게 알 수 있다.

### 3. Style Model First, Theme Later

Phase 004에서는 theme system을 만들기보다 cell이 어떤 style을 가지는지만 정확히 모델링한다.

권장 model:

- foreground color
- background color
- bold
- italic
- underline
- inverse
- reset/default style
- 8-color, bright 8-color
- 256-color
- RGB truecolor

theme palette나 사용자 설정 UI는 후속 phase로 분리한다. 지금은 parser가 받은 style attribute를 cell에 안정적으로 저장하고 renderer에 전달하는 것이 중요하다.

### 4. Style Span Snapshot

AppKit renderer가 cell 단위로 각각 draw call을 호출하면 성능과 구조가 나빠질 수 있다. `TerminalSnapshot`에서 같은 style이 연속된 구간을 style span으로 묶는 API를 추가하는 것이 좋다.

권장 형태:

```text
StyledLine
  └─ StyledSpan { text, style }
```

이 구조는 AppKit `NSAttributedString` 렌더링으로 연결하기 쉽고, 나중에 selection/copy 구현에도 도움이 된다.

### 5. Compatibility Matrix

ANSI/VT100/xterm 호환성은 "완료"라고 선언하기 어렵다. Phase 004부터는 문서에 compatibility matrix를 두고 sequence별 상태를 추적하는 방식이 좋다.

예상 상태:

- supported
- partially supported
- parsed but ignored
- ignored safely
- not supported

이 matrix는 이후 Phase 005, Phase 006에서 TUI 앱 호환성을 올릴 때 회고와 반복 작업의 기준이 된다.

### 6. Regression Runtime Commands

테스트 코드와 별도로 실제 shell에서 실행할 smoke command를 문서화한다.

예상 command:

```sh
printf '\033[31mred\033[0m normal\n'
printf '\033[1mbold\033[0m \033[4munderline\033[0m\n'
printf '\033[38;5;196mcolor-196\033[0m\n'
printf '\033[38;2;255;120;0mtruecolor\033[0m\n'
```

자동 테스트가 잡지 못하는 AppKit rendering 문제를 빠르게 확인할 수 있다.

## Proposed Work Breakdown

### Step 1. Document and dependency decision

- Phase 004 문서 작성
- `vte` crate 도입 방침 확정
- `alacritty_terminal`은 즉시 통합하지 않고 별도 spike 후보로 남김

완료 기준:

- Phase 004 scope와 non-goals가 문서화되어 있다.
- `vte` 사용 방식이 parser adapter라는 점이 명확하다.

결과:

- `docs/phases/phase004.md`를 추가했다.
- Phase 003 문서의 후속 작업 항목에서 Phase 004를 명시했다.
- `alacritty_terminal`은 즉시 통합하지 않고 후속 spike 후보로 남겼다.

### Step 2. Introduce vte adapter

- `terminal-core`에 `vte` dependency 추가
- 기존 `parser.rs`를 adapter 구조로 전환
- `vte::Perform` callback을 내부 action으로 매핑
- Phase 003에서 통과하던 parser/grid tests 유지

완료 기준:

- `cargo test`가 기존 Phase 003 테스트를 모두 통과한다.
- plain text, CR/LF, backspace, tab, cursor movement, clear line/screen 동작이 유지된다.

결과:

- `terminal-core`에 `vte = "0.15.0"`을 추가했다.
- 기존 hand-written parser state machine을 `vte::Parser` 기반 adapter로 교체했다.
- `vte::Perform` callback을 기존 `Action` enum으로 변환한다.
- `TerminalState::append_bytes`가 byte stream을 직접 parser에 전달하도록 바꿨다.
- 기존 Phase 003 테스트가 통과한다.

### Step 3. Add SGR style model

- `style.rs` 추가
- `Cell`에 style 추가
- `TerminalState`에 current style 추가
- SGR reset/basic style 처리
- 8-color, bright color, 256-color, truecolor 처리

완료 기준:

- SGR sequence가 cell style로 저장된다.
- reset 이후 기본 style로 복귀한다.
- style 처리가 text/cursor/scrollback을 깨지 않는다.

결과:

- `Color`, `Style`, `StyledLine`, `StyledSpan` model을 추가했다.
- `Cell`이 character와 함께 `Style`을 저장한다.
- `TerminalState`가 current style을 유지하고 SGR sequence를 누적 적용한다.
- `TerminalSnapshot`이 기존 `lines`와 함께 `styled_lines`를 제공한다.
- 8-color, bright 8-color, 256-color, truecolor foreground/background를 처리한다.
- bold, italic, underline, inverse, reset을 처리한다.

### Step 4. Add parser fixture/golden tests

- fixture 기반 테스트 구조 추가
- 주요 ANSI/SGR sequence snapshot 검증
- malformed escape sequence recovery 검증
- alternate screen 회귀 테스트 유지

완료 기준:

- 새 parser adapter가 fixture로 검증된다.
- 향후 xterm compatibility 확장 시 회귀 테스트를 추가할 수 있다.

결과:

- `crates/terminal-core/tests/fixtures.rs`를 추가했다.
- plain text, carriage return, cursor movement, clear line, OSC skip, malformed escape recovery, SGR style, alternate screen을 golden snapshot으로 검증한다.

### Step 5. Connect AppKit style-aware rendering

- `TerminalSnapshot`에 styled line/span API 추가
- AppKit renderer에서 `NSAttributedString` 또는 style span 기반 draw 연결
- foreground/background/bold/underline 최소 표시
- cursor rendering과 style rendering 충돌 방지

완료 기준:

- shell prompt color가 화면에 표시된다.
- `printf` 기반 SGR smoke command가 색상/스타일을 표시한다.
- 단색 fallback path가 유지된다.

결과:

- AppKit renderer가 `TerminalSnapshot.styled_lines`를 span 단위로 렌더링한다.
- foreground/background color, bold font fallback, underline, inverse를 반영한다.
- ANSI 16-color, 256-color cube/grayscale, truecolor RGB를 `NSColor`로 변환한다.
- `styled_lines`가 비어 있는 fallback snapshot에서는 기존 단색 렌더링을 유지한다.

## Compatibility Matrix

Phase 004 완료 시점의 terminal sequence 상태:

| Area | Status | Notes |
| --- | --- | --- |
| Plain printable text | supported | UTF-8 byte stream을 `vte` parser로 처리한다. |
| CR/LF/Tab/Backspace | supported | 기존 Phase 003 동작 유지. |
| Cursor movement `A/B/C/D/G/H/f` | supported | fixture와 단위 테스트로 검증. |
| Clear line/screen `J/K` | supported | mode `0`, `1`, `2`, screen `3` 처리. |
| Save/restore cursor | supported | `ESC 7/8`, `CSI s/u` 처리. |
| Alternate screen `?47/?1047/?1049` | supported | main screen 저장/복원. |
| OSC | ignored safely | title 등 OSC payload는 화면을 깨지 않도록 skip. |
| Charset designation | ignored safely | 현재 glyph set 전환은 하지 않는다. |
| SGR reset/basic attributes | supported | reset, bold, italic, underline, inverse. |
| SGR 8/16 color | supported | indexed color로 저장하고 AppKit palette로 렌더링. |
| SGR 256 color | supported | indexed color, color cube, grayscale 렌더링. |
| SGR truecolor | supported | RGB foreground/background 렌더링. |
| DEC private modes beyond alternate screen | parsed but ignored | bracketed paste/cursor visibility 등은 상태를 깨지 않도록 ignore. |
| Full xterm compatibility | not supported | 후속 phase에서 matrix를 확장한다. |

## Runtime Verification

2026-06-10에 다음 절차로 런타임 검증을 진행했다.

```sh
cargo test
scripts/bundle-macos-app.sh
open target/debug/Minimal\ Terminal.app
```

앱에서 실행한 smoke command:

```sh
printf '\033[31mred\033[0m normal\n'
printf '\033[1mbold\033[0m \033[4munderline\033[0m\n'
printf '\033[38;5;196mcolor-196\033[0m \033[38;2;255;120;0mtruecolor\033[0m\n'
```

확인 결과:

- `red`가 red ANSI color로 표시된다.
- `color-196`이 256-color red 계열로 표시된다.
- `truecolor`가 RGB truecolor로 표시된다.
- underline text가 밑줄로 표시된다.
- command 실행 후 shell prompt가 계속 표시된다.

## Non-goals

Phase 004에서 하지 않을 작업:

- ANSI/VT100/xterm 완전 준수 선언
- 모든 DEC private mode 지원
- full-screen TUI 앱 완전 호환성 보장
- GPU renderer 도입
- theme preference UI
- selection/copy 완성
- IME composition UI
- tab/split session management

## Risks

### Parser replacement risk

기존 hand-written parser를 `vte` adapter로 바꾸면 Phase 003에서 되던 동작이 깨질 수 있다.

대응:

- 기존 terminal-core 테스트를 먼저 고정한다.
- adapter 교체 후 기존 테스트가 통과하는지 확인한다.
- fixture/golden test를 추가해 회귀 범위를 넓힌다.

### Style rendering risk

style-aware rendering은 AppKit text drawing, font fallback, line height 계산에 영향을 줄 수 있다.

대응:

- 먼저 core style snapshot을 테스트한다.
- 그 다음 renderer를 연결한다.
- AppKit smoke command로 실제 색상 표시를 확인한다.

### Scope creep risk

ANSI/VT100 완전 준수는 범위가 크다. Phase 004에서 이를 목표로 잡으면 완료 기준이 흐려진다.

대응:

- Phase 004의 완료 기준은 `vte` adapter, SGR model, fixture test, style-aware rendering으로 제한한다.
- xterm compatibility expansion은 matrix를 만들고 후속 phase로 반복한다.

## Acceptance Criteria

Phase 004의 완료 기준:

- `vte` crate가 `terminal-core` parser path에 통합되어 있다. `done`
- 기존 hand-written parser dependency가 제거되거나 adapter 내부로 대체되어 있다. `done`
- Phase 003의 terminal-core 테스트가 계속 통과한다. `done`
- SGR style model이 cell에 저장된다. `done`
- 기본 foreground/background/bold/underline style이 snapshot으로 노출된다. `done`
- parser fixture/golden test가 추가되어 있다. `done`
- AppKit renderer가 style span을 사용해 색상/스타일을 표시한다. `done`
- `cargo test`가 통과한다. `done`
- 실제 앱에서 SGR smoke command가 색상/스타일을 화면에 표시한다. `done`

## Suggested Priority

권장 작업 순서:

1. `vte` adapter boundary 설계와 기존 테스트 고정
2. `vte` crate 도입 및 기존 parser 교체
3. SGR style model 추가
4. fixture/golden test 추가
5. style span snapshot 추가
6. AppKit style-aware rendering 연결
7. compatibility matrix 업데이트

이 순서가 좋은 이유는 parser 교체와 style rendering을 한 번에 섞지 않기 위해서다. 먼저 parser 교체가 기존 동작을 보존하는지 확인하고, 그 다음 style state와 rendering을 확장한다.

## Follow-up Candidates

Phase 004 이후 확정된 진행 순서:

1. Phase 005: Korean IME and Text Editing Stability
2. Phase 006: Selection, Copy, and Scrollback UX
3. Phase 007: TUI Compatibility Expansion
4. Phase 008: Terminal Compatibility Test Matrix

후속 후보:

- configurable theme/palette
- configurable Option/Command key policy
- `alacritty_terminal` 통합 가능성 별도 spike
