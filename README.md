# Minimal Terminal

macOS에서 사용할 안정성 우선의 터미널 에뮬레이터를 개발하는 Rust 프로젝트입니다.

현재 단계는 macOS 네이티브 AppKit 기반의 빈 창 앱을 실행하는 최소 기반 코드입니다.

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
cargo check
```

macOS `.app` 번들 생성:

```bash
scripts/bundle-macos-app.sh
```

앱 실행:

```bash
open 'target/debug/Minimal Terminal.app'
```

## Project Layout

```text
crates/
└── terminal-app/      # macOS native AppKit application

scripts/
└── bundle-macos-app.sh

GOAL.md               # project goal and MVP scope
ARCHITECTURE.md       # proposed architecture
```
