# Phase 035: DEC Special Graphics Charset

## Purpose

Phase 035의 목적은 `ESC ( 0`으로 선택되는 DEC Special Graphics G0 character set을 기본 지원해 `tmux`, `vttest`, `less`류 TUI의 line drawing 호환성을 높이는 것이다.

현재 matrix는 character set designation을 `parsed but ignored`로 표시한다. 이 상태에서는 ACS line drawing을 사용하는 프로그램이 `q`, `x`, `j` 같은 ASCII fallback 문자로 보일 수 있다.

## Scope

Phase 035에서 다룰 작업:

1. parser에 G0 DEC Special Graphics charset state 추가
2. `ESC ( 0`과 `ESC ( B` 처리
3. 대표 DEC Special Graphics 문자를 Unicode box drawing 문자로 매핑
4. parser/state/compatibility evidence와 문서 갱신

## Proposed Work Breakdown

### Step 1. Track G0 Charset

- `Parser`가 `advance_bytes` 호출 사이에서도 charset state를 유지한다.
- `ESC ( 0`은 DEC Special Graphics를 켠다.
- `ESC ( B`는 ASCII로 되돌린다.

완료 기준:

- parser test가 여러 `advance_bytes` 호출을 가로질러 charset state가 유지되는지 검증한다.

### Step 2. Map Line Drawing Characters

- 흔한 ACS 문자 `q`, `x`, `l`, `m`, `j`, `k`, `t`, `u`, `v`, `w`, `n`을 Unicode box drawing 문자로 변환한다.
- 매핑하지 않은 문자는 원래 문자로 둔다.

완료 기준:

- state/compatibility test가 rendered line에 box drawing 문자가 저장되는지 검증한다.

## Non-goals

- 모든 ISO 2022 G-set 전환을 구현하지 않는다.
- G1/G2/G3 locking shift와 single shift를 구현하지 않는다.
- locale-specific charset을 구현하지 않는다.

## Acceptance Criteria

- `ESC ( 0`과 `ESC ( B`가 parser state에 반영된다. `done`
- 대표 DEC Special Graphics line drawing 문자가 Unicode box drawing 문자로 저장된다. `done`
- unsupported charset designation은 safe ignore된다. `done`
- matrix와 known gaps가 갱신되어 있다. `done`
- `scripts/run-compatibility-core.sh`, `cargo test`, app smoke가 통과한다. `done`

## Implementation Update - 2026-06-11

Status: implementation complete.

구현된 내용:

- `Parser`가 G0 charset state를 유지하도록 변경했다.
- `ESC ( 0`은 DEC Special Graphics, `ESC ( B`는 ASCII로 전환한다.
- 대표 DEC Special Graphics line drawing 문자를 Unicode box drawing 문자로 매핑한다.
- parser/state/compatibility test evidence를 추가했다.
- matrix, known gaps, README를 갱신했다.

검증:

- `cargo test`
