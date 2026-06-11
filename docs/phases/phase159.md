# Phase 159 - vttest Device Status Report App Smoke

## Purpose

`vttest` runtime coverage를 terminal reports 메뉴의 Device Status Report 하위 테스트까지 확장한다.

## Scope

- `scripts/run-app-target-smokes.sh`에 `vttest-dsr` target을 추가한다.
- local PATH에 `vttest`가 있을 때 app 내부 PTY에서 `vttest`를 실행한다.
- follow-up input `6`, Return, `3`, Return으로 terminal reports의 DSR 하위 테스트에 진입한다.
- snapshot에서 cursor position report 성공 marker `Report is: <27> [ 5 ; 1 R  -- OK`를 확인한다.
- compatibility matrix, smoke-tests, known-gaps, README를 새 evidence에 맞게 갱신한다.

## Proposed Work Breakdown

1. `vttest-dsr` app target smoke를 추가한다.
2. 시작 메뉴에서 terminal reports 메뉴와 DSR 하위 테스트를 선택하는 follow-up input을 보낸다.
3. DSR 6 cursor position report 성공 marker를 확인한다.
4. 관련 compatibility 문서에 Phase 159 evidence를 연결한다.

## Acceptance Criteria

- [done] `vttest-dsr` app target smoke가 추가되어 있다.
- [done] `vttest-dsr` app target smoke가 local verification environment에서 통과한다.
- [done] compatibility 문서가 vttest DSR runtime evidence를 반영한다.

## Non-goals

- vttest full interactive suite 자동화
- terminal reports 메뉴 전체 항목 pass/fail 판정
- DSR 외 terminal report 응답 전체 coverage

## Result

상태: 구현 완료.

검증 marker:

- `Report is: <27> [ 5 ; 1 R  -- OK`
