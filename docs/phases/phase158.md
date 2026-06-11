# Phase 158 - htop Combined Mouse Mode Fix

## Purpose

`htop`이 한 CSI private mode sequence 안에서 SGR mouse mode와 mouse reporting mode를 함께 설정할 때 terminal state가 두 mode를 모두 반영하도록 수정한다.

## Scope

- `CSI ?1006;1000h`를 `SetSgrMouse(true)`와 `SetMouseReporting(true)`로 모두 처리한다.
- `CSI ?1006;1000l`를 `SetSgrMouse(false)`와 `SetMouseReporting(false)`로 모두 처리한다.
- parser/state level regression test를 추가한다.
- compatibility 문서에 htop runtime evidence를 연결한다.

## Proposed Work Breakdown

1. private CSI mode set/reset이 여러 parameter를 포함할 때 각 mode action을 순서대로 반환하게 한다.
2. parser test에 htop 형태의 combined mouse mode sequence를 추가한다.
3. state test에 combined mode enable/disable 검증을 추가한다.
4. 관련 compatibility 문서를 갱신한다.

## Acceptance Criteria

- [done] `parser::tests::parses_tui_private_modes`가 `CSI ?1006;1000h/l` action 분해를 검증한다.
- [done] `terminal_core::tests::tracks_tui_modes`가 combined mouse mode enable/disable 상태를 검증한다.
- [done] compatibility 문서가 htop combined mouse mode fix evidence를 반영한다.

## Non-goals

- htop 전체 mouse action coverage
- 실제 물리 mouse input end-to-end automation
- htop Setup 전체 설정 항목 coverage

## Result

상태: 구현 완료.

검증 sequence:

- `CSI ?1006;1000h`
- `CSI ?1006;1000l`

App smoke 영향:

- 기존 `htop-mouse-open-files` 좌표는 정확한 SGR mouse mode 처리 후 htop 하단 `F10Quit` 위치로 판정되어 `htop-mouse-setup` target으로 교체했다.
