# Phase 122 - tmux Mouse Wheel Direction Stabilization

## Purpose

`tmux-mouse-wheel` app target smoke가 pane history의 이전 출력으로 이동해야 하는 workflow에서 잘못된 wheel direction 때문에 marker를 놓치는 flake를 줄인다.

## Scope

- smoke hook에 `wheel-up-20` report variant를 추가한다.
- `tmux-mouse-wheel` target이 pane history를 위로 스크롤하도록 `wheel-up-20`을 사용한다.
- compatibility 문서와 README에 Phase 122 evidence를 기록한다.

## Proposed Work Breakdown

1. SGR/legacy `wheel-up-20` smoke mouse report를 추가한다.
2. `tmux-mouse-wheel` target의 report를 `wheel-down-20`에서 `wheel-up-20`으로 변경한다.
3. 전체 app target smoke suite를 실행한다.
4. 관련 문서를 갱신한다.

## Acceptance Criteria

- [done] `tmux-mouse-wheel` target이 `wheel-up-20`을 사용한다.
- [done] 전체 app target smoke suite가 통과한다.
- [done] tmux mouse wheel evidence가 compatibility 문서에 기록되어 있다.

## Non-goals

- native GUI `NSEvent` mouse end-to-end 검증
- tmux mouse drag/pane resize workflow 검증
- 모든 xterm mouse mode variant 검증

## Result

상태: 구현 완료.

`tmux-mouse-wheel`은 app smoke hook의 `wheel-up-20` reports로 tmux pane history의 marker를 확인한다.
