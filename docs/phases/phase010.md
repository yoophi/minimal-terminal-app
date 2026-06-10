# Phase 010: App Logic Unit Test Expansion

## Purpose

Phase 010의 목적은 AppKit에 직접 묶이지 않은 앱 로직을 unit test 가능한 모듈로 분리하고, Phase 005-007에서 구현한 input, paste, selection 동작의 자동 테스트 범위를 넓히는 것이다.

현재 `terminal-app`에는 input encoder와 selection test가 있지만, paste wrapping 같은 일부 로직은 `terminal_view.rs` private 함수로 남아 있어 직접 테스트하기 어렵다.

## Scope

Phase 010에서 다룰 작업:

1. bracketed paste byte wrapping을 테스트 가능한 module로 분리
2. input encoder test 확대
3. selection extraction test 확대
4. compatibility matrix의 runtime behavior evidence 보강

## Proposed Work Breakdown

### Step 1. Extract Paste Logic

- `crates/terminal-app/src/paste.rs`를 추가한다.
- bracketed paste wrapper를 순수 함수로 옮긴다.
- `TerminalView`는 새 module 함수를 호출한다.

완료 기준:

- bracketed paste wrapper unit test가 있다.
- 기존 paste 동작이 유지된다.

### Step 2. Expand Input Tests

추가할 test:

- forward delete
- Backspace/Delete 기본 경로
- Ctrl-C / Ctrl-D
- Option-left / Option-right
- Command 조합 무시
- application cursor mode arrow key
- IME confirmed UTF-8 text

완료 기준:

- input encoder의 주요 modifier 조합이 테스트된다.

### Step 3. Expand Selection Tests

추가할 test:

- 빈 줄 포함 multi-line selection
- selection end column이 line width를 넘어가는 경우
- combining mark
- wide character boundary
- styled text여도 plain text만 추출되는지

완료 기준:

- visible snapshot 기준 selection/copy edge case가 테스트된다.

## Non-goals

- AppKit pasteboard 자체를 unit test하지 않는다.
- cross-scrollback selection은 Phase 014로 분리한다.
- GUI click/drag 자동화는 이 phase 범위가 아니다.

## Risks

### AppKit Coupling

`terminal_view.rs`에 로직이 계속 남아 있으면 unit test가 어렵다.

대응:

- 순수 함수로 분리할 수 있는 로직부터 작게 이동한다.

## Acceptance Criteria

- paste wrapper가 module 단위로 테스트된다.
- input encoder test coverage가 Phase 005/007 동작을 포괄한다.
- selection test coverage가 Phase 006 edge case를 포괄한다.
- `cargo test`가 통과한다.
- `docs/compatibility/matrix.md` evidence가 필요한 경우 갱신되어 있다.

