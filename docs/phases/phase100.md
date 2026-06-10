# Phase 100 - Vim Mouse Left Press App Smoke

## 목표

app 내부 PTY에서 clean `vim`이 SGR mouse report를 받아 `<LeftMouse>` mapping을 실행하는지 자동 smoke로 확인한다.

## 범위

- `scripts/run-app-target-smokes.sh`에 `vim-mouse-left-press` target을 추가한다.
- clean `vim`에서 `mouse=a`, `ttymouse=sgr`를 켜고 smoke hook의 SGR left press가 marker file write mapping을 실행하는지 확인한다.
- compatibility matrix, smoke test 문서, known gaps, README의 Phase 상태를 갱신한다.

## 제외 범위

- native `NSEvent`에서 PTY write까지 이어지는 GUI end-to-end mouse 검증
- `less` mouse workflow 검증
- `vim` resize, plugin 환경, complex key chord 검증
- 전체 mouse drag/wheel workflow 인증

## 완료 기준

- `scripts/run-app-target-smokes.sh`에서 `vim-mouse-left-press`가 통과한다.
- `docs/compatibility/*`와 README가 Phase 100 evidence를 반영한다.
