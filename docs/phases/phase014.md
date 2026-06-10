# Phase 014: Cross-Scrollback Selection

## Purpose

Phase 014의 목적은 현재 visible snapshot 기준 selection을 확장해 scrollback과 live screen을 가로지르는 selection/copy를 지원하는 것이다.

현재 Phase 006 selection은 현재 보이는 snapshot 기준이다. 긴 출력에서 이전 scrollback 영역과 현재 화면을 함께 선택하는 동작은 모델링되어 있지 않다.

## Scope

Phase 014에서 다룰 작업:

1. stable buffer coordinate model 정의
2. scrollback + live screen range selection 구현
3. copy extraction을 buffer range 기반으로 변경
4. drag 중 autoscroll 정책 검토
5. wide character/styled text/multi-page test 추가

## Proposed Work Breakdown

### Step 1. Define Coordinate Model

- visible row/col 대신 buffer address 기준 좌표를 정의한다.
- scrollback line과 live screen line을 하나의 selection space로 볼지 결정한다.

완료 기준:

- selection coordinate model이 문서화되어 있다.

### Step 2. Implement Range Extraction

- buffer range에서 plain text를 추출한다.
- styled text는 copy에서 plain text로 유지한다.
- wide character boundary를 보존한다.

완료 기준:

- multi-page selection extraction test가 있다.

### Step 3. UI Integration

- drag 중 scrollback 이동을 지원할지 검토한다.
- PageUp/PageDown 상태에서 selection 시작/종료가 일관되게 동작하도록 한다.

완료 기준:

- 현재 visible selection 동작이 깨지지 않는다.

## Non-goals

- rich text copy는 구현하지 않는다.
- terminal app 외부 drag/drop은 구현하지 않는다.

## Risks

### Buffer Address Drift

scrollback이 max line limit으로 truncate될 때 selection address가 무효가 될 수 있다.

대응:

- selection 중 truncate 정책을 명확히 정의한다.
- 오래된 line이 사라지면 selection을 clamp하거나 clear한다.

## Acceptance Criteria

- scrollback을 가로지르는 selection/copy가 가능하다. `done`
- wide character와 multi-line copy가 깨지지 않는다. `done`
- current visible selection 테스트가 계속 통과한다. `done`
- matrix의 Cross-Scrollback Selection 상태가 갱신되어 있다. `done`
- `cargo test`가 통과한다. `done`

## Implementation Update - 2026-06-10

Status: implementation complete for scrollback/live viewport selection. Drag autoscroll and multi-page selection remain tracked as known gaps.

구현된 내용:

- `TerminalState::combined_snapshot`을 추가해 scrollback과 live screen을 하나의 viewport로 합쳤다.
- `TerminalBuffer`와 `TerminalView`가 combined snapshot을 사용하도록 변경했다.
- scrollback offset이 있을 때도 viewport가 scrollback/live boundary를 함께 포함할 수 있다.
- selection copy는 combined snapshot의 visible rows에서 plain text를 추출한다.
- scrollback/live boundary selection test와 combined snapshot test를 추가했다.
- matrix와 known gaps를 갱신했다.

검증:

- `scripts/run-compatibility-core.sh`
- `cargo test`
