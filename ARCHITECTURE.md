# Architecture

이 프로젝트는 macOS에서 안정적으로 동작하는 Rust 기반 터미널 에뮬레이터를 목표로 한다.

아키텍처는 엄격한 hexagonal architecture를 그대로 적용하기보다, hexagonal architecture의 장점을 가져온 modular monolith 구조를 따른다. 핵심 목표는 터미널 코어와 OS/UI 의존 코드를 분리하여 안정성, 테스트 용이성, 유지보수성을 확보하는 것이다.

## Architecture Style

권장 구조는 hexagonal-inspired modular monolith이다.

- 터미널 상태와 escape sequence 처리는 OS와 UI를 모르는 순수 Rust 로직으로 유지한다.
- PTY, macOS 창, 클립보드, 렌더러 같은 외부 의존성은 어댑터 계층에 둔다.
- 초기 MVP에서는 trait와 추상화를 과도하게 만들지 않고, 실제로 테스트와 교체 가능성이 필요한 경계에만 둔다.
- 크레이트는 역할 기준으로 분리하되, 런타임 구조는 단순하게 유지한다.

## Recommended Codebase Layout

```text
minimal-terminal-app/
├── crates/
│   ├── terminal-core/
│   │   ├── src/
│   │   │   ├── grid.rs
│   │   │   ├── cell.rs
│   │   │   ├── cursor.rs
│   │   │   ├── scrollback.rs
│   │   │   ├── parser.rs
│   │   │   ├── state.rs
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   │
│   ├── terminal-pty/
│   │   ├── src/
│   │   │   ├── pty.rs
│   │   │   ├── process.rs
│   │   │   ├── macos.rs
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   │
│   ├── terminal-renderer/
│   │   ├── src/
│   │   │   ├── layout.rs
│   │   │   ├── glyph.rs
│   │   │   ├── renderer.rs
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   │
│   └── terminal-app/
│       ├── src/
│       │   ├── app.rs
│       │   ├── window.rs
│       │   ├── input.rs
│       │   ├── clipboard.rs
│       │   ├── config.rs
│       │   └── main.rs
│       └── Cargo.toml
│
├── Cargo.toml
├── GOAL.md
├── ARCHITECTURE.md
└── README.md
```

## Crate Responsibilities

### terminal-core

터미널 에뮬레이터의 핵심 도메인 로직을 담당한다.

- ANSI/VT escape sequence 처리
- terminal grid 관리
- cell 상태 관리
- cursor 상태 관리
- scrollback 관리
- terminal state update

이 크레이트는 macOS, PTY, GUI, 클립보드, 렌더링 API에 의존하지 않아야 한다. 순수 Rust 로직으로 유지하여 단위 테스트를 집중적으로 작성할 수 있어야 한다.

### terminal-pty

OS의 PTY와 셸 프로세스 실행을 담당한다.

- macOS PTY 생성
- 기본 셸 실행
- PTY 입력 쓰기
- PTY 출력 읽기
- 창 크기 변경 시 PTY resize 전달
- 셸 프로세스 종료 상태 감지

macOS 전용 구현은 이 크레이트 안에 격리한다.

### terminal-renderer

`terminal-core`의 상태를 화면에 그릴 수 있는 형태로 변환하고 렌더링한다.

- grid layout 계산
- glyph 배치
- cursor 렌더링
- selection 렌더링
- viewport 계산

렌더러는 terminal state를 직접 변경하지 않아야 한다. core state를 읽고 화면 표현으로 변환하는 역할에 집중한다.

### terminal-app

실제 macOS 앱의 조립부 역할을 한다.

- 앱 실행
- 창 생성
- 이벤트 루프
- 키보드 입력 처리
- 마우스 selection 처리
- 클립보드 연동
- 설정 로딩
- PTY, core, renderer 연결

이 계층은 외부 이벤트를 내부 command로 변환하고, 각 모듈을 연결하는 역할을 맡는다.

## Dependency Direction

의존성 방향은 아래 원칙을 따른다.

```text
terminal-app
    ├── terminal-pty
    ├── terminal-renderer
    └── terminal-core

terminal-renderer
    └── terminal-core

terminal-pty
    └── OS APIs

terminal-core
    └── no OS/UI dependency
```

`terminal-core`는 가장 안쪽 계층이다. 다른 크레이트가 core를 사용할 수는 있지만, core가 다른 앱 계층을 알아서는 안 된다.

## Runtime Flow

터미널 출력 흐름은 다음과 같다.

```text
PTY output bytes
    -> parser
    -> terminal state update
    -> render snapshot
    -> window redraw
```

사용자 입력 흐름은 다음과 같다.

```text
keyboard input
    -> key encoding
    -> PTY write
    -> shell process
```

창 크기 변경 흐름은 다음과 같다.

```text
window resize
    -> terminal rows/cols calculation
    -> terminal state resize
    -> PTY resize
    -> redraw
```

## Hexagonal Mapping

hexagonal architecture 관점에서는 다음과 같이 볼 수 있다.

### Domain

- `terminal-core`

### Ports

초기 MVP에서 고려할 수 있는 port는 다음과 같다.

- `PtySession`
- `TerminalInput`
- `TerminalOutput`
- `Renderer`
- `Clipboard`

단, 초기부터 모든 port를 trait로 만들 필요는 없다. 테스트가 필요하거나 구현 교체 가능성이 실제로 생기는 지점부터 추상화한다.

### Adapters

- macOS PTY adapter
- macOS window adapter
- macOS clipboard adapter
- concrete renderer

## Stability Principles

안정성 우선 개발을 위해 다음 원칙을 따른다.

- `terminal-core`는 panic 없이 동작해야 한다.
- 잘못된 escape sequence는 무시하거나 안전하게 복구해야 한다.
- PTY 프로세스가 종료되어도 앱 프로세스는 유지되어야 한다.
- 대량 출력 상황에서 UI 이벤트 루프를 장시간 블로킹하지 않아야 한다.
- core state 변경은 가능한 한 명시적인 command 또는 update API를 통해 수행한다.
- 렌더러는 terminal state를 변경하지 않는다.
- OS와 UI 의존 코드는 core 밖에 격리한다.

## Testing Strategy

초기 테스트는 `terminal-core`에 집중한다.

- parser 단위 테스트
- cursor 이동 테스트
- grid write 테스트
- scrollback 테스트
- resize 테스트
- 잘못된 escape sequence 처리 테스트

PTY와 앱 계층은 통합 테스트 또는 수동 테스트 시나리오로 검증한다.

## Initial Implementation Guidance

초기 MVP에서는 다음 순서로 구현한다.

1. Cargo workspace 구성
2. `terminal-core`의 grid, cell, cursor, state 구현
3. ANSI escape sequence 기본 처리
4. `terminal-pty`에서 macOS PTY와 기본 셸 실행 구현
5. 단일 창 앱에서 PTY 출력 표시
6. 키보드 입력을 PTY로 전달
7. resize, scrollback, selection, copy/paste 추가
8. 안정성 기준에 맞춘 테스트와 오류 처리 보강
