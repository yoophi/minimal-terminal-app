# Phase 154 - vttest Cursor Movement App Smoke

## Purpose

`vttest` runtime coverage를 시작 메뉴 snapshot에서 실제 하위 테스트 화면 진입까지 확장한다.

## Scope

- `scripts/run-app-target-smokes.sh`에 `vttest-cursor-movement` target을 추가한다.
- local PATH에 `vttest`가 있을 때 app 내부 PTY에서 `vttest`를 실행한다.
- follow-up input `1`과 Return으로 `Test of cursor movements` 하위 테스트에 진입한다.
- snapshot에서 cursor movement test 안내 marker `The screen should be cleared`를 확인한다.
- compatibility matrix, smoke-tests, known-gaps, README를 새 evidence에 맞게 갱신한다.

## Proposed Work Breakdown

1. `vttest-cursor-movement` app target smoke를 추가한다.
2. 시작 메뉴에서 하위 테스트 1번을 선택하는 follow-up input을 보낸다.
3. 하위 테스트 화면 marker를 확인한다.
4. 관련 compatibility 문서에 Phase 154 evidence를 연결한다.

## Acceptance Criteria

- [done] `vttest-cursor-movement` app target smoke가 추가되어 있다.
- [done] `vttest-cursor-movement` app target smoke가 local verification environment에서 통과한다.
- [done] compatibility 문서가 vttest cursor movement runtime evidence를 반영한다.

## Non-goals

- vttest full interactive suite 자동화
- vttest 개별 menu 항목별 pass/fail 판정 전체 구현
- 화면 border의 pixel-perfect visual 판정

## Result

상태: 구현 완료.

검증 marker:

- `The screen should be cleared`
