# Phase 161 - htop Cell-Clipped AppKit Rendering

## Purpose

`htop` buffer snapshot은 정상인데 AppKit 화면에서 glyph fallback 또는 attributed background 폭 때문에 layout이 깨져 보일 수 있는 문제를 줄인다.

## Scope

- AppKit text drawing을 terminal cell clip rect 안에서 수행한다.
- text attribute의 background drawing에 의존하지 않고, 기존 cell-aligned background fill만 사용한다.
- `htop-runtime` snapshot marker가 계속 유지되는지 확인한다.

## Proposed Work Breakdown

1. per-character text drawing에 cell-width clip을 적용한다.
2. styled text attribute에서 variable glyph-bound background drawing을 제거한다.
3. htop runtime snapshot marker와 Rust test suite를 검증한다.
4. 관련 compatibility 문서에 htop rendering evidence를 기록한다.

## Acceptance Criteria

- [done] AppKit glyph drawing이 terminal cell clip rect 밖으로 번지지 않는다.
- [done] styled cell background는 cell-aligned fill 경로로만 처리한다.
- [done] `htop` snapshot에서 `Mem[`, `Tasks:`, `Load average:`, `PID USER`, `Command`, `F1Help  F2Setup`, `F10Quit` marker가 유지된다.

## Non-goals

- font fallback별 pixel-perfect 보증
- screenshot 기반 visual diff 자동화
- terminal-core parser/grid 동작 변경

## Result

상태: 구현 완료.

AppKit renderer가 각 glyph를 terminal cell 폭으로 clip하고, background attribute 대신 기존 cell-aligned background fill을 사용한다.
