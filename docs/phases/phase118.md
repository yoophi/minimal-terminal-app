# Phase 118 - Shell Home and Exit Notice App Smoke

## Purpose

app 내부 login shell이 기본적으로 `$HOME`에서 시작하고, shell process가 종료되면 사용자에게 종료 notice를 표시하는지 자동 smoke로 고정한다.

원격 변경으로 shell home startup과 shell exit notice 구현이 추가되었지만, app target smoke suite에는 아직 반복 가능한 runtime evidence가 없었다.

## Scope

- `scripts/run-app-target-smokes.sh`에 `shell-home` target을 추가한다.
- `scripts/run-app-target-smokes.sh`에 `shell-exit-notice` target을 추가한다.
- compatibility 문서와 README에 shell startup/exit evidence를 연결한다.

## Proposed Work Breakdown

1. app 내부 shell에서 `printf "shell-home:%s\n" "$PWD"`를 실행한다.
2. snapshot marker가 local `$HOME`과 일치하는지 확인한다.
3. app 내부 shell에 `exit`를 보내 `[Shell process exited]` notice가 snapshot에 남는지 확인한다.
4. matrix, smoke-tests, known-gaps, README를 갱신한다.

## Acceptance Criteria

- [done] `shell-home` app target smoke가 추가되어 있다.
- [done] `shell-exit-notice` app target smoke가 추가되어 있다.
- [done] `scripts/run-app-target-smokes.sh`에서 두 target이 통과한다.
- [done] shell startup/exit behavior가 compatibility 문서에 기록되어 있다.

## Non-goals

- shell별 startup file 동작 인증
- shell exit 후 자동 restart 구현
- 종료 notice UI 스타일 변경

## Result

상태: shell home startup and exit notice app smoke 완료.

검증 marker:

- `shell-home:${HOME}`
- `[Shell process exited]`
