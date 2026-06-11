# Phase 124 - Native NSEvent Mouse App Smoke

## Purpose

mouse reporting gap 중 smoke hook이 PTY에 직접 mouse bytes를 쓰는 경로가 아니라, synthetic `NSEvent`가 `TerminalView`의 `mouseDown:` 경로를 지나 PTY로 전달되는 runtime evidence를 추가한다.

## Scope

- `MINIMAL_TERMINAL_SMOKE_NATIVE_MOUSE_REPORT=left-press` smoke hook을 추가한다.
- AppKit main thread에서 synthetic left mouse down `NSEvent`를 생성하고 `TerminalView`의 `mouseDown:` selector로 전달한다.
- `scripts/run-app-target-smokes.sh`에 `native-mouse-sgr-report` target을 추가한다.
- compatibility matrix, smoke-tests, known-gaps, README를 새 evidence에 맞게 갱신한다.

## Proposed Work Breakdown

1. `TerminalView` redraw timer에서 native mouse smoke 요청을 감지한다.
2. mouse reporting mode가 켜진 뒤 synthetic `NSEvent`를 생성한다.
3. 기존 `mouseDown:` path가 SGR mouse bytes를 PTY에 쓰는지 shell readback으로 확인한다.
4. 관련 문서에 direct smoke hook과 native event path evidence를 구분해 기록한다.

## Acceptance Criteria

- [done] `native-mouse-sgr-report` app target smoke가 추가되어 있다.
- [done] target은 `TerminalView`의 `mouseDown:` selector를 사용한다.
- [done] app target smoke에서 native mouse SGR report prefix가 확인된다.
- [done] Mouse Reporting gap이 native synthetic event evidence로 좁혀져 있다.

## Non-goals

- 실제 물리 mouse input automation
- drag, release, wheel native event 전체 coverage
- 모든 xterm mouse mode variant 검증

## Result

상태: 구현 완료.

검증 marker:

- `native-mouse-sgr-report:1b5b3c30`
