# Phase 054: British NRCS Charset

## Purpose

Phase 054의 목적은 locale-specific charset coverage의 첫 단계로 British NRCS designation을 구현하는 것이다.

Phase 053까지는 DEC Special Graphics 중심의 G-set switching을 다뤘다. 이 phase에서는 `ESC ( A` 계열 British charset에서 `#`을 `£`로 매핑한다.

## Scope

Phase 054에서 다룰 작업:

1. parser charset enum에 British NRCS 추가
2. `ESC ( A`, `ESC ) A`, `ESC * A`, `ESC + A` designation 처리
3. British charset에서 `#`을 `£`로 매핑
4. parser/state 테스트 추가
5. matrix와 known gaps 갱신

## Proposed Work Breakdown

### Step 1. Add British Charset Mapping

- British NRCS는 `#`만 `£`로 매핑한다.
- 다른 printable character는 그대로 둔다.

완료 기준:

- `ESC ( A #`가 `£`로 렌더링된다.

### Step 2. Wire British Designation Across G-sets

- G0-G3 designation intermediate에서 final byte `A`를 처리한다.
- 기존 ASCII, DEC Special Graphics 동작은 유지한다.

완료 기준:

- G0 designation과 G2 single shift 경로에서 British mapping이 동작한다.

## Non-goals

- 모든 NRCS/locale-specific charset을 구현하지 않는다.
- right-side G-set locking shift를 구현하지 않는다.

## Acceptance Criteria

- [done] parser가 British NRCS designation을 처리한다.
- [done] terminal state가 British `#` -> `£` output을 grid에 렌더링한다.
- [done] matrix와 known gaps가 갱신되어 있다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## Implementation Update - 2026-06-11

상태: British NRCS charset 구현 완료.

구현된 내용:

- `Charset::British`를 추가했다.
- `ESC ( A`, `ESC ) A`, `ESC * A`, `ESC + A`를 처리한다.
- British charset에서 `#`을 `£`로 매핑한다.
- parser/state 테스트를 추가했다.
- compatibility matrix, known gaps, README를 갱신했다.

검증:

- `scripts/run-compatibility-core.sh`
- `cargo test`
- `scripts/run-app-smoke.sh`
- `scripts/run-app-command-smoke.sh`
- `scripts/run-app-target-smokes.sh`
