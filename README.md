# Minimal Terminal

macOS에서 사용할 안정성 우선의 터미널 에뮬레이터를 개발하는 Rust 프로젝트입니다.

현재 단계는 macOS 네이티브 AppKit 창에서 login shell, terminal grid/cursor, ANSI/SGR style, scrollback, selection/copy, Korean IME composition, 그리고 1차 TUI compatibility 확장을 포함한 MVP 기반 코드입니다.

## Goals

- 개발 언어: Rust
- 대상 플랫폼: macOS
- 우선순위: 안정성
- 초기 목표: 단일 창 터미널 에뮬레이터 MVP

## Requirements

- macOS
- Rust toolchain

## Run

개발 빌드 확인:

```bash
cargo test
```

호환성 core regression과 문서 consistency 확인:

```bash
scripts/run-compatibility-core.sh
```

로컬 macOS 앱 smoke 확인:

```bash
scripts/run-app-smoke.sh
```

macOS `.app` 번들 생성:

```bash
scripts/bundle-macos-app.sh
```

앱 실행:

```bash
open 'target/debug/Minimal Terminal.app'
```

## Logging

macOS Console.app과 `log` CLI를 통한 로그 확인 방법은 [docs/LOGGING.md](docs/LOGGING.md)를 참고합니다.

## Project Layout

```text
crates/
├── terminal-core/     # terminal grid, cursor, and parser state
└── terminal-app/      # macOS native AppKit application

docs/
├── phases/
│   ├── phase001.md
│   ├── phase002.md
│   ├── phase003.md
│   ├── phase004.md
│   ├── phase005.md
│   ├── phase006.md
│   ├── phase007.md
│   ├── phase008.md
│   ├── phase009.md
│   └── ...
├── compatibility/
│   ├── csi.md
│   ├── standards-and-tests.md
│   ├── test-strategy.md
│   ├── app-readiness.md
│   ├── matrix.md
│   ├── smoke-tests.md
│   ├── known-gaps.md
│   └── regression-runner.md
└── LOGGING.md

scripts/
├── bundle-macos-app.sh
├── check-compatibility-docs.sh
├── run-app-smoke.sh
└── run-compatibility-core.sh

GOAL.md               # project goal and MVP scope
ARCHITECTURE.md       # proposed architecture
```

## Current Phase Status

- Phase 001-004: AppKit scaffold, PTY pipeline, terminal core, `vte` parser, and SGR style rendering are complete.
- Phase 005: Korean IME composition and text input stability are implemented.
- Phase 006: Selection, copy, and scrollback UX are implemented for the current visible snapshot.
- Phase 007: First TUI compatibility expansion is implemented.
- Phase 008: Compatibility matrix, smoke test protocol, known gap tracking, CSI/reference documentation, regression runner, and core compatibility evidence tests are implemented. GUI/runtime smoke scenarios remain manual.
- Phase 009: Compatibility documentation consistency checks are implemented.
- Phase 010-015: App logic tests, DSR, cursor style, SGR mouse reporting, combined scrollback selection, and local app smoke automation are implemented.
- Phase 016-021: TUI replay fixtures, app smoke unknown resolution, xterm/vttest compatibility expansion, representative app certification tracking, secondary device attributes, and function/modified key encoding are implemented.
