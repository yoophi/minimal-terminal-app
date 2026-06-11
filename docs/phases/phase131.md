# Phase 131: Native F5 Modifier Matrix Smoke

## 목적

Phase 130은 Up key의 modifier runtime matrix를 검증했다. 이 phase는 function key 계열에서도 F5의 Shift/Option/Control 7개 modifier 조합 전체가 AppKit `keyDown:` 경로를 지나 xterm-style modified function key sequence로 PTY에 전달되는지 확인한다.

## 범위

- `shift-f5`, `option-f5`, `shift-option-f5`, `shift-control-f5`, `control-option-f5`, `shift-control-option-f5` native key smoke 매핑을 추가한다.
- Shift-only function key가 text input path로 빠지지 않도록 special key 분기를 보정한다.
- 기존 `control-f5`와 함께 F5 modifier matrix 7개 조합을 raw readback으로 확인한다.
- `scripts/run-app-target-smokes.sh`에 `native-f5-modifier-matrix-key` target을 추가한다.
- compatibility matrix, smoke-tests, known-gaps, README를 갱신한다.

## 완료 기준

- [done] F5 modifier matrix native key smoke 매핑이 추가되어 있다.
- [done] Shift-only function key가 text input path로 빠지지 않는 unit test가 추가되어 있다.
- [done] `native-f5-modifier-matrix-key` app target smoke가 추가되어 있다.
- [done] app snapshot에 7개 F5 modifier sequence의 per-modifier marker가 모두 남는다.
- [done] `cargo test`가 통과한다.
- [done] `scripts/run-app-target-smokes.sh` 전체 suite가 통과한다.
- [done] 관련 compatibility 문서가 F5 modifier matrix runtime evidence를 반영한다.

## 비범위

- 모든 function key의 전체 runtime matrix
- 실제 물리 keyboard event automation
- 앱별 key binding workflow 전체 인증
