# Phase 027: OSC Title Updates

## Purpose

Phase 027의 목적은 OSC title update sequence를 window title에 반영하는 것이다.

이전에는 OSC title payload를 safe ignore했다. terminal-aware CLI와 shell prompt는 window title을 업데이트할 수 있으므로 `OSC 0`과 `OSC 2`를 제한된 범위로 지원한다.

## Scope

Phase 027에서 다룰 작업:

1. `OSC 0 ; title BEL` parser handling 추가
2. `OSC 2 ; title BEL` parser handling 추가
3. core pending title write queue 추가
4. AppKit main thread window title 적용 경로 추가
5. compatibility evidence와 문서 갱신

## Proposed Work Breakdown

### Step 1. Parse OSC Title

- title payload는 UTF-8이어야 한다.
- 빈 title과 과도하게 큰 title은 safe ignore한다.

완료 기준:

- parser test가 OSC title update를 검증한다.

### Step 2. Queue and Apply Title

- core는 title을 pending queue에 저장한다.
- TerminalView timer가 main thread에서 queue를 drain하고 마지막 title을 window title에 적용한다.

완료 기준:

- state/compatibility test가 pending title write를 검증한다.
- app build와 smoke가 통과한다.

## Non-goals

- icon title과 window title을 별도로 표시하지 않는다.
- OSC title stack 또는 reset policy를 구현하지 않는다.
- 모든 OSC extension을 구현하지 않는다.

## Risks

### Excessive Title Updates

PTY output이 title update를 자주 보낼 수 있다.

대응:

- main thread drain 시 마지막 title만 window에 적용한다.
- title payload 크기를 제한한다.

## Acceptance Criteria

- OSC 0/2 title parser test가 있다. `done`
- pending title write state test가 있다. `done`
- AppKit window title update path가 있다. `done`
- matrix와 known gaps가 갱신되어 있다. `done`
- `scripts/run-compatibility-core.sh`와 `cargo test`가 통과한다. `done`

## Implementation Update - 2026-06-11

Status: implementation complete for OSC title updates.

구현된 내용:

- `SetWindowTitle` parser action을 추가했다.
- `TerminalState`가 pending title write queue를 제공한다.
- `TerminalBuffer`와 `TerminalView`가 pending title write를 main thread window title update로 반영한다.
- parser/state/compatibility test evidence를 추가했다.
- matrix와 known gaps를 갱신했다.

검증:

- `cargo test -p terminal-core osc_title`
- `cargo test -p terminal-app`
- `scripts/run-compatibility-core.sh`
- `cargo test`
