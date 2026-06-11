# Phase 115 - htop Layout Smoke Hardening

## Purpose

`htop`이 app 내부 PTY에서 정상적으로 full-screen layout을 그리는지 더 엄격하게 확인한다.

기존 `htop-runtime` smoke는 `Tasks:` marker만 확인해서 meter, process table header, bottom function-key row가 함께 유지되는지 충분히 보장하지 못했다.

## Scope

- `htop-runtime` app target smoke를 다중 marker 검증으로 강화한다.
- 실패 중이던 experimental `htop` mouse quit target은 제거한다.
- `htop` mouse workflow는 별도 phase gap으로 유지한다.

## Proposed Work Breakdown

1. app target smoke script에 required marker helper를 추가한다.
2. `htop-runtime`에서 `Tasks:`, `Load average:`, `PID USER`, `Command`, `F10Quit`를 모두 확인한다.
3. 실패하는 mouse 좌표 target과 전용 smoke hook 변경을 제거한다.
4. compatibility 문서의 `htop-runtime` evidence를 갱신한다.

## Acceptance Criteria

- [done] `htop-runtime`이 meter/status/header/function-key row marker를 모두 확인한다.
- [done] 실패하는 `htop-mouse-f10-quit` target이 target smoke suite에 남아 있지 않다.
- [done] 강화된 `htop-runtime` target이 local verification environment에서 통과한다.
- [done] `htop` mouse workflow는 known gap으로 계속 추적한다.

## Non-goals

- native GUI screenshot 기반 visual diff 자동화
- `htop` mouse workflow 인증
- `htop` setup 내부 설정 변경/저장 workflow 인증

## Result

상태: htop runtime layout smoke hardening 완료.

검증한 marker:

- `Tasks:`
- `Load average:`
- `PID USER`
- `Command`
- `F10Quit`

검증 명령:

- `cargo test -p terminal-app smoke -- --nocapture`
- targeted `htop-runtime` app smoke with required markers
