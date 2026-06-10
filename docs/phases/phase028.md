# Phase 028: vttest Menu Replay Evidence

## Purpose

Phase 028의 목적은 `vttest` runtime coverage gap을 줄이기 위해 시작 메뉴 output을 replay fixture로 고정하는 것이다.

Phase 018에서 `vttest` 설치와 version은 확인했지만 menu 기반 runtime 결과는 남아 있었다. 이 phase에서는 full interactive run을 보증하지 않고, 자동 테스트 가능한 시작 메뉴 stream만 core replay evidence로 추가한다.

## Scope

Phase 028에서 다룰 작업:

1. `vttest` 시작 menu output의 representative fixture 추가
2. primary DA query response path와 menu rendering 검증
3. matrix, known gaps, smoke-tests 문서 갱신

## Proposed Work Breakdown

### Step 1. Capture Representative Menu Stream

- `vttest` 시작 화면에서 나오는 reset, DA query, cursor movement, clear screen, menu text sequence를 fixture로 둔다.
- generated capture 전체가 아니라 유지보수 가능한 최소 representative stream을 저장한다.

완료 기준:

- `crates/terminal-core/tests/fixtures/tui/vttest_menu.ansi`가 있다.

### Step 2. Add Replay Test

- fixture를 `TerminalState`에 replay한다.
- menu title과 대표 menu row가 렌더링되는지 확인한다.
- primary DA response가 pending response로 큐잉되는지 확인한다.

완료 기준:

- `tests/tui_replay.rs::vttest_menu_replay_renders_menu_and_queues_da_response`가 통과한다.

## Non-goals

- 모든 `vttest` menu item 통과를 선언하지 않는다.
- 앱 내부 interactive vttest smoke를 대체하지 않는다.
- keyboard/mouse manual test 결과를 자동화했다고 주장하지 않는다.

## Risks

### Overclaiming vttest Coverage

시작 메뉴 replay는 full vttest compatibility가 아니다.

대응:

- matrix는 `partially supported`로 유지한다.
- interactive menu runtime result는 known gap으로 남긴다.

## Acceptance Criteria

- vttest menu replay fixture가 있다. `done`
- replay test가 menu rendering과 primary DA response를 검증한다. `done`
- matrix, known gaps, smoke-tests가 갱신되어 있다. `done`
- `scripts/run-compatibility-core.sh`와 `cargo test`가 통과한다. `done`

## Implementation Update - 2026-06-11

Status: implementation complete for vttest menu replay evidence.

구현된 내용:

- `vttest_menu.ansi` fixture를 추가했다.
- `vttest_menu_replay_renders_menu_and_queues_da_response` test를 추가했다.
- vttest matrix evidence를 version-only에서 replay fixture 포함으로 갱신했다.
- known gaps와 smoke-tests 문서를 갱신했다.

검증:

- `cargo test -p terminal-core --test tui_replay vttest_menu`
- `scripts/run-compatibility-core.sh`
- `cargo test`
