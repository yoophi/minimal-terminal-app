# Phase 128: Native Modified Navigation Key Smoke

## 목적

Phase 127은 modified function key runtime evidence를 추가했다. 이 phase는 navigation key 계열도 synthetic AppKit `keyDown:` event가 `TerminalView` 입력 경로를 지나 xterm-style modified sequence로 PTY에 전달되는지 확인한다.

## 범위

- `MINIMAL_TERMINAL_SMOKE_NATIVE_KEY=shift-option-up` hook을 추가한다.
- hook은 main-thread `TerminalView` redraw timer에서 synthetic Shift+Option+Up `NSEvent`를 만들고 `keyDown:`으로 전달한다.
- shell raw readback으로 `ESC [ 1 ; 4 A` bytes를 확인한다.
- `scripts/run-app-target-smokes.sh`에 `native-shift-option-up-key` target을 추가한다.
- compatibility matrix, smoke-tests, known-gaps, README를 갱신한다.

## 완료 기준

- [done] `native-shift-option-up-key` app target smoke가 추가되어 있다.
- [done] app snapshot에 `native-shift-option-up-key:1b5b313b3441` marker가 남는다.
- [done] `cargo test`가 통과한다.
- [done] `scripts/run-app-target-smokes.sh` 전체 suite가 통과한다.
- [done] 관련 compatibility 문서가 modified navigation key runtime evidence를 반영한다.

## 비범위

- 모든 modifier navigation key 조합의 runtime smoke
- 실제 물리 keyboard event automation
- 앱별 key binding workflow 전체 인증
