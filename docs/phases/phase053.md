# Phase 053: G2/G3 DEC Special Graphics Single Shift

## Purpose

Phase 053의 목적은 DEC Special Graphics charset 지원에 G2/G3 single shift를 추가하는 것이다.

Phase 052는 G2/G3 locking shift를 구현했다. 이 phase에서는 `ESC N`과 `ESC O`를 처리해 다음 printable character에만 G2/G3 charset을 적용한다.

## Scope

Phase 053에서 다룰 작업:

1. parser가 single shift charset 상태를 추적
2. `ESC N`으로 다음 printable character에 G2 적용
3. `ESC O`로 다음 printable character에 G3 적용
4. printable 처리 직후 single shift 상태 해제
5. parser/state 테스트 추가
6. matrix와 known gaps 갱신

## Proposed Work Breakdown

### Step 1. Add Single Shift State

- active charset과 별도로 one-shot charset 상태를 둔다.
- printable character를 처리하면 one-shot 상태를 해제한다.

완료 기준:

- single shift가 다음 한 글자에만 영향을 준다.

### Step 2. Add SS2/SS3 Support

- `ESC N`은 G2 single shift로 처리한다.
- `ESC O`는 G3 single shift로 처리한다.
- 기존 G0-G3 locking shift 동작은 유지한다.

완료 기준:

- G2/G3 single shift 조합이 line drawing 문자를 렌더링한다.

## Non-goals

- 8-bit C1 `SS2`/`SS3` bytes를 별도로 구현하지 않는다.
- right-side G-set locking shift를 구현하지 않는다.
- locale-specific charset을 구현하지 않는다.

## Acceptance Criteria

- [done] parser가 `ESC N` / `ESC O` single shift를 처리한다.
- [done] terminal state가 G2/G3 single shift line drawing output을 grid에 렌더링한다.
- [done] matrix와 known gaps가 갱신되어 있다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## Implementation Update - 2026-06-11

상태: G2/G3 DEC Special Graphics single shift 구현 완료.

구현된 내용:

- `Parser`에 one-shot single shift charset 상태를 추가했다.
- `ESC N`, `ESC O`를 처리하고 다음 printable character 뒤 single shift 상태를 해제한다.
- parser/state 테스트를 추가했다.
- compatibility matrix, known gaps, README를 갱신했다.

검증:

- `scripts/run-compatibility-core.sh`
- `cargo test`
- `scripts/run-app-smoke.sh`
- `scripts/run-app-command-smoke.sh`
- `scripts/run-app-target-smokes.sh`
