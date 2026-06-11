# Phase 138 - Native Control Navigation Family Smoke

## 목적

Phase 137은 Shift 단독 navigation family runtime evidence를 추가했다. 이 phase는 Control 단독 조합에서도 navigation key family가 xterm-style modified sequence로 PTY에 전달되는지 확인한다.

## 범위

- Control Up/Down/Right/Left/Home/End/PageUp/PageDown/Delete synthetic `NSEvent`를 raw readback으로 확인한다.
- `scripts/run-app-target-smokes.sh`에 `native-control-navigation-family-key` target을 추가한다.
- native key smoke parser test에 `control-page-down` case를 추가한다.
- compatibility matrix, smoke-tests, known-gaps, README를 갱신한다.

## 완료 기준

- [done] `native-control-navigation-family-key` app target smoke가 추가되어 있다.
- [done] app snapshot에 9개 Control navigation sequence의 per-key marker가 모두 남는다.
- [done] `terminal_view::tests::native_key_smoke_event_maps_known_cases`가 Control PageDown smoke parser case를 검증한다.
- [done] `cargo test`가 통과한다.
- [done] `scripts/run-app-target-smokes.sh` 전체 suite가 통과한다.
- [done] 관련 compatibility 문서가 Control navigation family runtime evidence를 반영한다.

## 비범위

- Option-only Left/Right/Backspace는 기존 shell word navigation 정책과 충돌하므로 별도 phase에서 판단한다.
- 모든 function key의 전체 runtime matrix
- 실제 물리 keyboard event automation
