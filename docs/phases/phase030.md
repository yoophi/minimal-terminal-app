# Phase 030: Absolute Scrollback Selection

## Purpose

Phase 030의 목적은 selection anchor와 active point를 현재 viewport row가 아니라 terminal output 전체의 absolute row로 저장해 여러 scrollback page를 가로지르는 선택을 안정화하는 것이다.

Phase 014와 Phase 025에서 scrollback/live boundary selection과 drag autoscroll을 구현했지만, selection state가 viewport row를 저장하고 스크롤 때마다 좌표를 보정하는 방식이라 장거리 선택에서 누적 오차가 생길 수 있다.

## Scope

Phase 030에서 다룰 작업:

1. `TerminalSnapshot`에 현재 snapshot 시작 absolute row를 노출
2. `SelectionState`를 absolute row 기반으로 변경
3. 현재 viewport에 보이는 selection range만 계산해 highlight와 copy에 사용
4. multi-page selection unit test와 compatibility 문서 갱신

## Proposed Work Breakdown

### Step 1. Snapshot Absolute Row Metadata

- live screen과 scrollback을 합친 row space에서 snapshot 첫 줄의 absolute row를 계산한다.
- normal snapshot과 fallback snapshot도 같은 필드를 제공한다.

완료 기준:

- core test가 combined snapshot의 시작 absolute row를 검증한다.

### Step 2. Absolute Selection State

- mouse event의 viewport row를 snapshot 시작 absolute row와 더해 selection point로 저장한다.
- scrollback 이동 시 selection point를 직접 shift하지 않는다.
- draw/copy 시 현재 snapshot과 겹치는 selection range만 viewport 좌표로 변환한다.

완료 기준:

- selection unit test가 여러 page를 가로지르는 absolute selection을 현재 viewport range로 투영한다.

## Non-goals

- GUI 자동 드래그 smoke를 추가하지 않는다.
- terminal scrollback 저장 한도를 변경하지 않는다.
- rectangular/block selection은 구현하지 않는다.

## Acceptance Criteria

- `TerminalSnapshot`이 `viewport_start_absolute_row`를 제공한다. `done`
- selection anchor/active가 absolute row로 저장된다. `done`
- scrollback 이동 중 selection 좌표를 shift하지 않는다. `done`
- multi-page selection projection test가 추가되어 있다. `done`
- matrix와 known gaps가 갱신되어 있다. `done`
- `scripts/run-compatibility-core.sh`, `cargo test`, app smoke가 통과한다. `done`

## Implementation Update - 2026-06-11

Status: implementation complete.

구현된 내용:

- `TerminalSnapshot`에 `viewport_start_absolute_row`를 추가했다.
- `TerminalState::snapshot`, `scrollback_snapshot`, `combined_snapshot`에서 snapshot 시작 absolute row를 계산한다.
- `SelectionState`가 absolute row 기반 range를 유지하고, 현재 viewport와 겹치는 range만 투영한다.
- `TerminalView`가 mouse selection point를 absolute row로 저장한다.
- copy path가 retained scrollback과 live screen 전체 snapshot에서 absolute selection range를 추출한다.
- compatibility matrix와 known gaps를 갱신했다.

검증:

- `cargo test`
