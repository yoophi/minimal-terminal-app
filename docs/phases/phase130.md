# Phase 130: Native Up Modifier Matrix Smoke

## 목적

Phase 127-129는 대표 modified key runtime evidence를 개별 target으로 추가했다. 이 phase는 같은 AppKit `keyDown:` 경로에서 Up key의 Shift/Option/Control 7개 modifier 조합 전체가 xterm-style sequence로 PTY에 전달되는지 한 번에 검증한다.

## 범위

- `MINIMAL_TERMINAL_SMOKE_NATIVE_KEY` hook이 쉼표로 구분한 key smoke 목록을 처리하도록 확장한다.
- `shift-up`, `option-up`, `control-up`, `shift-control-up`, `control-option-up`, `shift-control-option-up` 매핑을 추가한다.
- 기존 `shift-option-up`과 함께 Up key modifier matrix 7개 조합을 raw readback으로 확인한다.
- `scripts/run-app-target-smokes.sh`에 `native-up-modifier-matrix-key` target을 추가한다.
- compatibility matrix, smoke-tests, known-gaps, README를 갱신한다.

## 완료 기준

- [done] comma-separated native key smoke hook이 추가되어 있다.
- [done] `native-up-modifier-matrix-key` app target smoke가 추가되어 있다.
- [done] app snapshot에 7개 Up modifier sequence의 per-modifier marker가 모두 남는다.
- [done] `cargo test`가 통과한다.
- [done] `scripts/run-app-target-smokes.sh` 전체 suite가 통과한다.
- [done] 관련 compatibility 문서가 Up key modifier matrix runtime evidence를 반영한다.

## 비범위

- 모든 navigation key와 function key의 전체 runtime matrix
- 실제 물리 keyboard event automation
- 앱별 key binding workflow 전체 인증
