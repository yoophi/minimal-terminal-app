# Phase 119 - htop Styled Cell Background Rendering

## Goal

`htop`처럼 full-screen TUI가 배경색이 있는 공백 cell을 사용해 meter/header/layout을 그릴 때 AppKit 창에서 셀 단위 배경이 안정적으로 표시되도록 한다.

## Scope

1. trailing styled space가 `StyledLine` snapshot에서 사라지지 않게 보존한다.
2. AppKit renderer가 styled span의 배경을 텍스트 draw 전에 terminal cell 크기 사각형으로 칠한다.
3. 배경색만 있는 공백 cell 보존을 core unit test로 검증한다.

## Acceptance Criteria

- [done] 배경색 또는 inverse style이 있는 trailing space가 styled snapshot에 남는다.
- [done] renderer가 styled span background를 cell grid 크기로 그린다.
- [done] 관련 unit test와 htop app smoke가 통과한다.

## Result

상태: 구현 완료.

`preserves_trailing_styled_background_cells`가 배경색만 있는 trailing spaces를 검증한다. AppKit renderer는 styled text를 그리기 전에 span 폭만큼 `NSRectFill`로 배경 cell을 먼저 칠한다.
