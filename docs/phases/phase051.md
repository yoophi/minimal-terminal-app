# Phase 051: G1 DEC Special Graphics Locking Shift

## Purpose

Phase 051의 목적은 DEC Special Graphics charset 지원을 G0 전용에서 G1 designation과 SO/SI locking shift까지 확장하는 것이다.

Phase 035는 `ESC ( 0` / `ESC ( B` 기반 G0 charset switching을 구현했다. 이 phase에서는 `ESC ) 0`, `SO`, `SI` 경로를 추가해 xterm charset coverage를 넓힌다.

## Scope

Phase 051에서 다룰 작업:

1. parser가 G0/G1 charset 상태와 active charset을 추적
2. `ESC ) 0` / `ESC ) B`로 G1 charset designation 처리
3. `SO` (`0x0e`)로 G1 locking shift, `SI` (`0x0f`)로 G0 복귀 처리
4. parser/state 테스트 추가
5. matrix와 known gaps 갱신

## Proposed Work Breakdown

### Step 1. Extend Parser Charset State

- 기존 G0 charset 상태를 G0/G1 + active charset 상태로 확장한다.
- G0 기존 동작은 유지한다.

완료 기준:

- 기존 G0 DEC Special Graphics 테스트가 계속 통과한다.

### Step 2. Add G1 Locking Shift Support

- `ESC ) 0`은 G1을 DEC Special Graphics로 지정한다.
- `ESC ) B`는 G1을 ASCII로 지정한다.
- `SO`는 active charset을 G1로 바꾼다.
- `SI`는 active charset을 G0로 되돌린다.

완료 기준:

- `ESC ) 0`, `SO`, printable, `SI` 조합이 line drawing 문자를 렌더링한다.

## Non-goals

- G2/G3 designation을 구현하지 않는다.
- single shift `SS2`/`SS3`를 구현하지 않는다.
- locale-specific charset을 구현하지 않는다.

## Acceptance Criteria

- [done] parser가 G1 DEC Special Graphics designation과 SO/SI locking shift를 처리한다.
- [done] terminal state가 G1 line drawing output을 grid에 렌더링한다.
- [done] matrix와 known gaps가 갱신되어 있다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## Implementation Update - 2026-06-11

상태: G1 DEC Special Graphics locking shift 구현 완료.

구현된 내용:

- `Parser`가 G0/G1 charset과 active charset을 추적하도록 확장했다.
- `ESC ) 0`, `ESC ) B`, `SO`, `SI`를 처리한다.
- parser/state 테스트를 추가했다.
- compatibility matrix, known gaps, README를 갱신했다.

검증:

- `scripts/run-compatibility-core.sh`
- `cargo test`
- `scripts/run-app-smoke.sh`
- `scripts/run-app-command-smoke.sh`
- `scripts/run-app-target-smokes.sh`
