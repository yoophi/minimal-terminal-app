# Phase 151 - vttest Version App Smoke

## 목적

`vttest` runtime coverage를 넓히기 전에 app 내부 PTY에서 `vttest -V` version output을 자동 smoke evidence로 고정한다.

## 범위

- `scripts/run-app-target-smokes.sh`에 `vttest-version` target을 추가한다.
- local PATH에 `vttest`가 있을 때 app 내부 PTY에서 `vttest -V`를 실행한다.
- snapshot marker `VT100 test program`을 확인한다.
- compatibility matrix, smoke-tests, known-gaps, README를 갱신한다.

## 완료 기준

- [done] `vttest-version` app target smoke가 추가되어 있다.
- [done] `vttest-version` app target smoke가 local verification environment에서 통과한다.
- [done] 관련 compatibility 문서가 version app smoke evidence를 반영한다.

## 비범위

- vttest full interactive menu suite 자동화
- vttest 개별 menu 항목별 pass/fail 판정
