# Phase 037: Exhaustive Modified Key Test Coverage

## Purpose

Phase 037의 목적은 Shift/Option/Control modifier 조합이 붙은 navigation/function key encoding을 표본 테스트가 아니라 조합 테스트로 고정하는 것이다.

Phase 021에서 modified key encoding을 구현했지만 일부 조합만 테스트했다. 이 phase에서는 xterm modifier parameter 2-8에 해당하는 모든 Shift/Option/Control 조합을 자동 테스트 evidence로 남긴다.

## Scope

Phase 037에서 다룰 작업:

1. Shift, Option, Control, Shift+Option, Shift+Control, Option+Control, Shift+Option+Control 조합 테스트
2. navigation key CSI modifier sequence 검증
3. function key modifier sequence 검증
4. matrix와 known gaps 갱신

## Proposed Work Breakdown

### Step 1. Add Combination Tests

- `KEY_UP` 같은 CSI navigation key에 대해 modifier parameter 2-8을 검증한다.
- `KEY_F12` 같은 tilde function key에 대해 modifier parameter 2-8을 검증한다.

완료 기준:

- input unit test가 7개 modifier 조합을 모두 검증한다.

### Step 2. Update Evidence

- matrix의 modified key evidence에 exhaustive combination test를 연결한다.
- runtime app smoke gap은 유지한다.

완료 기준:

- code-level coverage와 runtime smoke gap이 문서에서 분리되어 있다.

## Non-goals

- GUI keyboard automation을 추가하지 않는다.
- macOS 키보드 레이아웃별 physical key mapping을 인증하지 않는다.
- Option 단독 word navigation 정책을 변경하지 않는다.

## Acceptance Criteria

- Shift/Option/Control 7개 modifier 조합이 navigation key test로 검증된다. `done`
- Shift/Option/Control 7개 modifier 조합이 function key test로 검증된다. `done`
- 기존 unmodified arrow와 Option word navigation 동작이 유지된다. `done`
- matrix와 known gaps가 갱신되어 있다. `done`
- `scripts/run-compatibility-core.sh`, `cargo test`, app smoke가 통과한다. `done`

## Implementation Update - 2026-06-11

Status: implementation complete for code-level modifier combination coverage.

구현된 내용:

- `input::tests::encodes_all_shift_option_control_navigation_combinations`를 추가했다.
- `input::tests::encodes_all_shift_option_control_function_key_combinations`를 추가했다.
- compatibility matrix와 known gaps를 갱신했다.

검증:

- `cargo test -p terminal-app input::tests::encodes_all_shift_option_control`
