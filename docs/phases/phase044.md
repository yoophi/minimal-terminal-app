# Phase 044: htop F10 Function Key App Smoke

## Purpose

Phase 044의 목적은 app 내부 PTY에서 `htop` full-screen UI에 진입한 뒤 xterm F10 function key sequence로 종료되는 workflow를 자동 확인하는 것이다.

Phase 041은 `q` quit key path를 확인했다. 이 phase에서는 function key path까지 확인해 `htop` interactive key workflow evidence를 넓힌다.

## Scope

Phase 044에서 다룰 작업:

1. `scripts/run-app-target-smokes.sh`에 `htop-f10-quit` target 추가
2. app 내부 PTY에서 `htop` 실행
3. follow-up xterm F10 sequence `ESC [ 21 ~` 입력
4. shell marker `htop-f10-ok` 확인
5. matrix와 known gaps 갱신

## Proposed Work Breakdown

### Step 1. Add htop F10 Target

- `htop; printf "htop-f10-ok\n"`를 실행한다.
- follow-up input으로 F10 sequence `ESC [ 21 ~`를 보낸다.
- `htop` 종료 뒤 shell marker가 출력되는지 확인한다.

완료 기준:

- local environment에서 `htop-f10-quit` app target smoke가 통과한다.

### Step 2. Update Evidence

- `htop` matrix row에 F10 function key workflow evidence를 연결한다.
- Environment-dependent App Smoke Target gap에서 `htop` function key 항목을 정리한다.

완료 기준:

- htop version, redraw, `q` quit, F10 quit evidence가 문서에서 구분되어 있다.

## Non-goals

- `htop`의 모든 function key를 인증하지 않는다.
- mouse interaction을 인증하지 않는다.
- GUI focus 기반 keyboard automation을 추가하지 않는다.

## Acceptance Criteria

- [done] `htop-f10-quit` app target smoke가 추가되어 있다.
- [done] 설치된 환경에서 htop F10 quit marker가 확인된다.
- [done] matrix와 known gaps가 갱신되어 있다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## Implementation Update - 2026-06-11

상태: htop F10 function key workflow evidence 구현 완료.

구현된 내용:

- `scripts/run-app-target-smokes.sh`에 `htop-f10-quit` target을 추가했다.
- app 내부 PTY에서 `htop`을 실행하고 follow-up F10 sequence 뒤 marker `htop-f10-ok`를 확인한다.
- compatibility matrix, smoke test 문서, known gaps, README를 갱신했다.

검증:

- `scripts/run-compatibility-core.sh`
- `cargo test`
- `scripts/run-app-smoke.sh`
- `scripts/run-app-command-smoke.sh`
- `scripts/run-app-target-smokes.sh`
