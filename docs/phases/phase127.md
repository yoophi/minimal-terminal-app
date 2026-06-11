# Phase 127: Native Modified Function Key Smoke

## 목적

modifier key variants는 code-level test로 검증되어 있지만 app runtime evidence가 부족하다. 이 phase는 synthetic AppKit `keyDown:` event가 `TerminalView` 입력 경로를 지나 xterm-style modified function key sequence로 PTY에 전달되는지 자동 확인한다.

## 범위

- `MINIMAL_TERMINAL_SMOKE_NATIVE_KEY=control-f5` hook을 추가한다.
- hook은 main-thread `TerminalView` redraw timer에서 synthetic Control+F5 `NSEvent`를 만들고 `keyDown:`으로 전달한다.
- shell raw readback으로 `ESC [ 15 ; 5 ~` bytes를 확인한다.
- `scripts/run-app-target-smokes.sh`에 `native-control-f5-key` target을 추가한다.
- compatibility matrix, smoke-tests, known-gaps, README를 갱신한다.

## 완료 기준

- [done] `native-control-f5-key` app target smoke가 추가되어 있다.
- [done] app snapshot에 `native-control-f5-key:1b5b31353b357e` marker가 남는다.
- [done] `cargo test`가 통과한다.
- [done] `scripts/run-app-target-smokes.sh` 전체 suite가 통과한다.
- [done] 관련 compatibility 문서가 modified function key runtime evidence를 반영한다.

## 비범위

- 모든 modifier key 조합의 runtime smoke
- 실제 물리 keyboard event automation
- 앱별 key binding workflow 전체 인증
