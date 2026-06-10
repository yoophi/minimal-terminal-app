# Phase 016: TUI Replay Fixtures

## Purpose

Phase 016의 목적은 실제 TUI 프로그램이 출력한 escape stream을 fixture로 저장하고 `terminal-core`에서 replay해 회귀 테스트하는 것이다.

실제 `less`, `vim`, `top`을 매번 자동 조작하는 것은 환경 의존성이 크다. Replay fixture는 AppKit이나 외부 프로그램 설치 상태에 덜 의존하면서 TUI compatibility를 넓히는 중간 단계다.

## Scope

Phase 016에서 다룰 작업:

1. TUI escape stream fixture 저장 위치 정의
2. `tui_replay.rs` test 추가
3. `less`, `vim`, `top` 최소 fixture 수집
4. alternate screen, cursor visibility, scroll region, style 상태 검증
5. matrix evidence 연결

## Proposed Work Breakdown

### Step 1. Fixture Format

권장 구조:

```text
crates/terminal-core/tests/fixtures/tui/less_open_close.ansi
crates/terminal-core/tests/fixtures/tui/vim_minimal.ansi
crates/terminal-core/tests/fixtures/tui/top_minimal.ansi
```

완료 기준:

- fixture 저장 형식과 naming rule이 정해져 있다.

### Step 2. Replay Test Harness

- fixture byte를 읽어 `TerminalState::append_bytes`에 전달한다.
- 최종 snapshot이나 mode를 검증한다.

완료 기준:

- 최소 한 개 TUI replay test가 통과한다.

### Step 3. Matrix Evidence

- replay test가 어떤 app smoke row를 보완하는지 `matrix.md`에 기록한다.

완료 기준:

- TUI smoke의 일부가 자동 replay evidence로 연결된다.

## Non-goals

- TUI process를 자동으로 조작하지 않는다.
- interactive input timing을 재현하지 않는다.
- AppKit rendering을 검증하지 않는다.

## Risks

### Fixture Drift

TUI 버전이나 shell 환경에 따라 escape stream이 달라질 수 있다.

대응:

- fixture 생성 환경을 주석 또는 문서에 기록한다.
- 최소 동작만 검증한다.

## Acceptance Criteria

- `tui_replay.rs`가 있다. `done`
- 최소 `less` 또는 `vim` replay fixture가 있다. `done`
- replay test가 `scripts/run-compatibility-core.sh`에 포함된다. `done`
- matrix evidence가 갱신되어 있다. `done`

## Implementation Update - 2026-06-10

Status: implementation complete.

구현된 내용:

- `crates/terminal-core/tests/tui_replay.rs`를 추가했다.
- `crates/terminal-core/tests/fixtures/tui/*.ansi` fixture 저장 위치를 추가했다.
- fixture는 diff가 쉬운 `\x1b`, `\n`, `\r` textual escape format으로 저장하고 test에서 byte stream으로 decode한다.
- `less`, `vim`, `top` 최소 replay fixture를 추가했다.
- replay test가 alternate screen restore, cursor visibility/style restore, styled redraw를 검증한다.
- matrix와 regression runner 문서를 갱신했다.

검증:

- `scripts/run-compatibility-core.sh`
- `cargo test`
