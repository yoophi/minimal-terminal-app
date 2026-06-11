# Phase 141 - Native Shift Function Key Family Smoke

## 목적

Phase 131은 F5 한 키의 전체 modifier matrix runtime evidence를 추가했다. 이 phase는 Shift 단독 조합에서 F1-F12 function key family 전체가 xterm-style modified sequence로 PTY에 전달되는지 확인한다.

## 범위

- Shift+F1-F4 synthetic `NSEvent`를 6-byte raw readback으로 확인한다.
- Shift+F5-F12 synthetic `NSEvent`를 7-byte raw readback으로 확인한다.
- `scripts/run-app-target-smokes.sh`에 `native-shift-function-f1-f4-key`, `native-shift-function-f5-f12-key` target을 추가한다.
- native key smoke parser test에 `shift-f12` case를 추가한다.
- compatibility matrix, smoke-tests, known-gaps, README를 갱신한다.

## 완료 기준

- [done] Shift F1-F12 app target smoke가 추가되어 있다.
- [done] app snapshot에 12개 Shift function key sequence의 per-key marker가 모두 남는다.
- [done] `terminal_view::tests::native_key_smoke_event_maps_known_cases`가 Shift+F12 smoke parser case를 검증한다.
- [done] `cargo test`가 통과한다.
- [done] `scripts/run-app-target-smokes.sh` 전체 suite가 통과한다.
- [done] 관련 compatibility 문서가 Shift function key family runtime evidence를 반영한다.

## 비범위

- Option/Control/Shift+Option/Shift+Control/Control+Option/Shift+Control+Option function key family runtime smoke
- 실제 물리 keyboard event automation
