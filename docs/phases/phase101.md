# Phase 101 - less Mouse Wheel App Smoke

## 목표

app 내부 PTY에서 `less --mouse`가 mouse wheel report를 받아 pager 화면을 이동하는지 자동 smoke로 확인한다.

## 범위

- smoke hook에 `wheel-down` mouse report를 추가한다.
- `scripts/run-app-target-smokes.sh`에 `less-mouse-wheel-down` target을 추가한다.
- `less --mouse --wheel-lines=10` 실행 후 smoke hook의 wheel-down reports가 화면을 아래로 이동시켜 `less-mouse-line-045`가 snapshot에 나타나는지 확인한다.
- compatibility matrix, smoke test 문서, known gaps, README의 Phase 상태를 갱신한다.

## 제외 범위

- native `NSEvent`에서 PTY write까지 이어지는 GUI end-to-end mouse 검증
- `less` left click, drag, follow mode 검증
- 전체 xterm mouse mode variant 검증

## 완료 기준

- `scripts/run-app-target-smokes.sh`에서 `less-mouse-wheel-down`이 통과한다.
- `docs/compatibility/*`와 README가 Phase 101 evidence를 반영한다.
