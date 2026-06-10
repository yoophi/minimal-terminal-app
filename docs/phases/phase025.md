# Phase 025: Selection Drag Autoscroll

## Purpose

Phase 025의 목적은 native selection drag가 viewport 위/아래로 벗어날 때 scrollback을 자동으로 이동해 긴 출력 선택 UX를 개선하는 것이다.

Phase 014에서 scrollback과 live screen을 함께 포함하는 viewport copy를 구현했지만, drag 중 자동 스크롤은 gap으로 남아 있었다. 이 phase에서는 viewport 단위 autoscroll과 selection row 보정을 추가한다.

## Scope

Phase 025에서 다룰 작업:

1. drag 위치가 viewport 상단/하단을 벗어나는지 판단
2. native selection drag 중 scrollback offset 자동 조정
3. viewport 이동 시 selection anchor/active row 보정
4. unit test와 compatibility 문서 갱신

## Proposed Work Breakdown

### Step 1. Define Autoscroll Direction

- viewport 위로 drag하면 scrollback offset을 증가시킨다.
- viewport 아래로 drag하면 scrollback offset을 감소시킨다.
- viewport 안에서는 offset을 바꾸지 않는다.

완료 기준:

- pure helper test가 상단/내부/하단 방향을 검증한다.

### Step 2. Preserve Selection Anchor

- scrollback offset이 변하면 기존 selection row를 같은 절대 라인에 머물도록 보정한다.
- row는 현재 viewport 범위로 clamp한다.

완료 기준:

- selection state test가 row shift를 검증한다.

## Non-goals

- 여러 page를 가로지르는 absolute row 기반 selection model 전체를 구현하지 않는다.
- drag speed에 따른 가변 autoscroll을 구현하지 않는다.
- GUI mouse drag smoke 자동화를 구현하지 않는다.

## Risks

### Selection Address Drift

viewport가 이동하면 기존 row 좌표가 다른 line을 가리킬 수 있다.

대응:

- scrollback offset 조정 직후 selection row를 같은 방향으로 shift한다.
- 더 긴 범위의 absolute selection model은 known gap으로 남긴다.

## Acceptance Criteria

- drag autoscroll direction test가 있다. `done`
- selection row shift test가 있다. `done`
- native selection drag가 상하단 밖에서 scrollback offset을 조정한다. `done`
- matrix와 known gaps가 갱신되어 있다. `done`
- `scripts/run-compatibility-core.sh`와 `cargo test`가 통과한다. `done`

## Implementation Update - 2026-06-11

Status: implementation complete for viewport drag autoscroll.

구현된 내용:

- `selection_drag_autoscroll_delta` helper를 추가했다.
- mouse drag 중 viewport 밖으로 이동하면 scrollback offset을 조정한다.
- `SelectionState::shift_rows`를 추가해 viewport 이동 시 selection row를 보정한다.
- unit test와 compatibility 문서를 갱신했다.

검증:

- `cargo test -p terminal-app`
- `scripts/run-compatibility-core.sh`
- `cargo test`
