# Phase 136 - Native Shift Control Option Navigation Family Smoke

## 목적

Phase 132, 133, 135는 Control+Option, Shift+Control, Shift+Option navigation family runtime evidence를 추가했다. 이 phase는 같은 native key smoke parser를 사용해 Shift+Control+Option 조합에서도 navigation key family가 xterm-style modified sequence로 PTY에 전달되는지 확인한다.

## 범위

- Shift+Control+Option Up/Down/Right/Left/Home/End/PageUp/PageDown/Delete synthetic `NSEvent`를 raw readback으로 확인한다.
- `scripts/run-app-target-smokes.sh`에 `native-shift-control-option-navigation-family-key` target을 추가한다.
- native key smoke parser test에 `shift-control-option-page-down` case를 추가한다.
- compatibility matrix, smoke-tests, known-gaps, README를 갱신한다.

## 완료 기준

- [done] `native-shift-control-option-navigation-family-key` app target smoke가 추가되어 있다.
- [done] app snapshot에 9개 Shift+Control+Option navigation sequence의 per-key marker가 모두 남는다.
- [done] `terminal_view::tests::native_key_smoke_event_maps_known_cases`가 Shift+Control+Option PageDown smoke parser case를 검증한다.
- [done] `cargo test`가 통과한다.
- [done] `scripts/run-app-target-smokes.sh` 전체 suite가 통과한다.
- [done] 관련 compatibility 문서가 Shift+Control+Option navigation family runtime evidence를 반영한다.

## 비범위

- 모든 modifier와 모든 navigation key의 곱집합 runtime matrix
- 모든 function key의 전체 runtime matrix
- 실제 물리 keyboard event automation
