# Phase 020: Secondary Device Attributes

## Purpose

Phase 020의 목적은 `CSI > c` secondary device attributes query에 응답해 `tmux`와 terminal-aware CLI가 사용하는 capability query gap을 줄이는 것이다.

Phase 018에서 primary device attributes를 구현했고, secondary device attributes는 full xterm compatibility gap의 세부 항목으로 남아 있었다. 이 phase에서는 해당 항목만 작게 구현하고 테스트 evidence를 matrix에 연결한다.

## Scope

Phase 020에서 다룰 작업:

1. `CSI > c`, `CSI > 0 c` parser action 추가
2. core response queue에 secondary DA 응답 추가
3. compatibility evidence test 추가
4. matrix와 known gaps 갱신

## Proposed Work Breakdown

### Step 1. Parse Secondary DA

- `CSI > c`
- `CSI > 0 c`

완료 기준:

- parser test가 secondary DA query를 별도 action으로 검증한다.

### Step 2. Queue Response

- app PTY response path는 기존 pending response 경로를 재사용한다.
- 응답은 xterm 계열 secondary DA 형식 `ESC[>0;0;0c`를 사용한다.

완료 기준:

- state test가 pending response를 검증한다.

### Step 3. Update Compatibility Evidence

- `docs/compatibility/matrix.md`의 Secondary device attributes row를 `supported`로 갱신한다.
- `docs/compatibility/known-gaps.md`에서 해당 gap을 resolved로 이동한다.

완료 기준:

- supported row가 parser/state/compatibility test evidence에 연결되어 있다.

## Non-goals

- xterm version/vendor 값을 정확히 에뮬레이션하지 않는다.
- XTGETTCAP, DA3, DEC private reporting 전체를 구현하지 않는다.
- `tmux` 전체 workflow 지원을 이 phase에서 보증하지 않는다.

## Risks

### Overclaiming xterm Compatibility

secondary DA 응답은 capability query 중 하나일 뿐이다.

대응:

- full xterm compatibility와 representative app certification gap은 유지한다.
- matrix의 supported 범위는 `CSI > c` 응답으로 제한한다.

## Acceptance Criteria

- `CSI > c`, `CSI > 0 c`가 parser action으로 처리된다. `done`
- core가 `ESC[>0;0;0c` 응답을 pending response로 큐잉한다. `done`
- `tests/compatibility.rs`에 secondary DA evidence가 있다. `done`
- matrix와 known gaps가 갱신되어 있다. `done`
- `scripts/run-compatibility-core.sh`와 `cargo test`가 통과한다. `done`

## Implementation Update - 2026-06-11

Status: implementation complete.

구현된 내용:

- `SecondaryDeviceAttributes` parser action을 추가했다.
- `TerminalState`가 `CSI > c`, `CSI > 0 c` 입력에 `ESC[>0;0;0c`를 큐잉한다.
- parser/state/compatibility test evidence를 추가했다.
- matrix에서 Secondary device attributes를 `supported`로 갱신했다.
- known gaps에서 Secondary DA를 resolved로 이동했다.

검증:

- `cargo test -p terminal-core secondary_device_attributes`
- `scripts/run-compatibility-core.sh`
- `cargo test`
