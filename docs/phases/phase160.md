# Phase 160 - vttest Device Attributes App Smoke

## Purpose

`vttest` runtime coverage를 terminal reports 메뉴의 Primary Device Attributes와 Secondary Device Attributes 하위 테스트까지 확장한다.

## Scope

- `scripts/run-app-target-smokes.sh`에 `vttest-primary-da` target을 추가한다.
- `scripts/run-app-target-smokes.sh`에 `vttest-secondary-da` target을 추가한다.
- local PATH에 `vttest`가 있을 때 app 내부 PTY에서 `vttest`를 실행한다.
- follow-up input `6`, Return, `4`, Return으로 primary DA 하위 테스트에 진입한다.
- follow-up input `6`, Return, `5`, Return으로 secondary DA 하위 테스트에 진입한다.
- snapshot에서 primary DA marker `means VT100 with AVO`를 확인한다.
- snapshot에서 secondary DA marker `Pv=0, firmware version 0.0`을 확인한다.
- compatibility matrix, smoke-tests, known-gaps, README를 새 evidence에 맞게 갱신한다.

## Proposed Work Breakdown

1. `vttest-primary-da` app target smoke를 추가한다.
2. `vttest-secondary-da` app target smoke를 추가한다.
3. terminal reports 메뉴에서 DA 하위 테스트를 선택하는 follow-up input을 보낸다.
4. 관련 compatibility 문서에 Phase 160 evidence를 연결한다.

## Acceptance Criteria

- [done] `vttest-primary-da` app target smoke가 추가되어 있다.
- [done] `vttest-secondary-da` app target smoke가 추가되어 있다.
- [done] 두 target이 local verification environment에서 통과한다.
- [done] compatibility 문서가 vttest DA runtime evidence를 반영한다.

## Non-goals

- vttest full interactive suite 자동화
- terminal reports 메뉴 전체 항목 pass/fail 판정
- DA 응답 문자열의 xterm 버전 정확도 확장

## Result

상태: 구현 완료.

검증 marker:

- `means VT100 with AVO`
- `Pv=0, firmware version 0.0`
