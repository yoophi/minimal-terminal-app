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
- Phase 016-117: TUI replay fixtures, app smoke unknown resolution, xterm/vttest compatibility expansion, representative app certification tracking, secondary device attributes, function/modified key encoding, exhaustive modifier key tests, application keypad mode, OSC 52 clipboard write/query policy, legacy/modifier-aware mouse encoding, mode-gated SGR mouse runtime evidence, selection drag autoscroll, OSC title updates, vttest menu replay/app runtime evidence, htop runtime layout/AppKit cell width rendering/quit/F10/F1 help/F5 tree/F2 setup/elevated layout marker evidence, fzf interactive redraw/preview/selection/multi-select/shell Ctrl-T/Alt-C/Ctrl-R evidence and preview smoke timing stabilization, less direct quit/search/follow and mouse wheel evidence, git pager quit/page navigation/search/horizontal scroll/mark evidence with repository-path-stable app smokes, vim edit/write/quit/mouse left press/window split key chord/resize redraw evidence, tmux attached session, split-pane, pane resize, copy mode, mouse wheel, nested vim, and split-pane nested vim resize evidence with repository-path-stable helper script execution, claude/codex version and help output evidence, Tab/Esc input fixes, absolute scrollback selection, app command smoke automation, app target command smokes, representative CLI version app smokes, 8-bit C1 CSI introducer evidence, 8-bit C1 SS2/SS3 single shift support, 8-bit C1 OSC/ST support, DEC Supplemental UPSS alias support, DEC Special Graphics G0-G3 locking/single/right-side shift charset support, British/Dutch/French-Canadian/German/Finnish/French/Greek/Hebrew/Italian/DEC Supplemental Graphics/DEC Technical/DEC Cyrillic/DEC Greek Supplemental/DEC Hebrew Supplemental/DEC Turkish Supplemental/ISO Greek Supplemental/ISO Hebrew Supplemental/ISO Latin-Cyrillic Supplemental/ISO Latin-5 Supplemental/ISO Latin-1 Supplemental/ISO Latin-2 Supplemental/JIS-Katakana/JIS-Roman/Norwegian-Danish/Portuguese/Russian/SCS/Spanish/Swedish/Swiss/Turkish charset support, and `codex` CLI version smoke fallback are implemented.
