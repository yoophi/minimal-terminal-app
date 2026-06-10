# Phase 052: G2/G3 DEC Special Graphics Locking Shift

## Purpose

Phase 052의 목적은 DEC Special Graphics charset 지원을 G2/G3 designation과 locking shift까지 확장하는 것이다.

Phase 051은 G1 designation과 SO/SI locking shift를 구현했다. 이 phase에서는 `ESC * 0`, `ESC + 0`, `ESC n`, `ESC o`를 추가해 xterm charset coverage를 더 넓힌다.

## Scope

Phase 052에서 다룰 작업:

1. parser가 G2/G3 charset 상태를 추적
2. `ESC * 0` / `ESC * B`로 G2 charset designation 처리
3. `ESC + 0` / `ESC + B`로 G3 charset designation 처리
4. `ESC n`으로 G2 locking shift, `ESC o`로 G3 locking shift 처리
5. parser/state 테스트 추가
6. matrix와 known gaps 갱신

## Proposed Work Breakdown

### Step 1. Extend Parser Charset State

- 기존 G0/G1 charset 상태를 G0/G1/G2/G3로 확장한다.
- G0/G1 기존 동작은 유지한다.

완료 기준:

- 기존 G0/G1 DEC Special Graphics 테스트가 계속 통과한다.

### Step 2. Add G2/G3 Locking Shift Support

- `ESC * 0`은 G2를 DEC Special Graphics로 지정한다.
- `ESC + 0`은 G3를 DEC Special Graphics로 지정한다.
- `ESC n`은 active charset을 G2로 바꾼다.
- `ESC o`는 active charset을 G3로 바꾼다.
- `SI`는 active charset을 G0로 되돌린다.

완료 기준:

- G2/G3 locking shift 조합이 line drawing 문자를 렌더링한다.

## Non-goals

- single shift `SS2`/`SS3`를 구현하지 않는다.
- locale-specific charset을 구현하지 않는다.
- right-side G-set locking shift를 구현하지 않는다.

## Acceptance Criteria

- [done] parser가 G2/G3 DEC Special Graphics designation과 locking shift를 처리한다.
- [done] terminal state가 G2/G3 line drawing output을 grid에 렌더링한다.
- [done] matrix와 known gaps가 갱신되어 있다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## Implementation Update - 2026-06-11

상태: G2/G3 DEC Special Graphics locking shift 구현 완료.

구현된 내용:

- `Parser`가 G0/G1/G2/G3 charset과 active charset을 추적하도록 확장했다.
- `ESC * 0`, `ESC * B`, `ESC + 0`, `ESC + B`, `ESC n`, `ESC o`를 처리한다.
- parser/state 테스트를 추가했다.
- compatibility matrix, known gaps, README를 갱신했다.

검증:

- `scripts/run-compatibility-core.sh`
- `cargo test`
- `scripts/run-app-smoke.sh`
- `scripts/run-app-command-smoke.sh`
- `scripts/run-app-target-smokes.sh`
