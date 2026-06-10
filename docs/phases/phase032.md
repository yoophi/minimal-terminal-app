# Phase 032: App Command Smoke Harness

## Purpose

Phase 032의 목적은 native app 내부 PTY에 명령 입력을 주입하고 terminal buffer snapshot을 파일로 남기는 자동 smoke harness를 추가하는 것이다.

기존 `scripts/run-app-smoke.sh`는 앱이 일정 시간 생존하는지만 확인한다. 이 phase에서는 앱 프로세스 내부에서 shell command가 PTY를 통해 실행되고, 출력이 terminal buffer에 반영되는지 자동으로 검증한다.

## Scope

Phase 032에서 다룰 작업:

1. smoke 전용 환경 변수 기반 input injection 추가
2. smoke 전용 terminal snapshot file dump 추가
3. app command smoke script 추가
4. compatibility 문서와 README 갱신

## Proposed Work Breakdown

### Step 1. Add Smoke Harness

- `MINIMAL_TERMINAL_SMOKE_INPUT`이 설정되면 일정 지연 후 PTY에 bytes를 쓴다.
- `MINIMAL_TERMINAL_SMOKE_SNAPSHOT_PATH`가 설정되면 terminal buffer의 retained snapshot을 파일로 저장한다.
- smoke 완료 후 기본적으로 process를 종료한다.

완료 기준:

- 앱 binary를 실행해 command output snapshot을 파일로 얻을 수 있다.

### Step 2. Add Script Evidence

- `scripts/run-app-command-smoke.sh`를 추가한다.
- script는 앱 bundle을 만들고, app 내부 shell에서 marker 출력 명령을 실행한 뒤 snapshot에서 marker를 확인한다.

완료 기준:

- script가 local macOS 환경에서 통과한다.

## Non-goals

- GUI focus와 keyboard automation을 완전히 대체하지 않는다.
- `htop`, `fzf`, `git log`의 interactive key workflow를 이 phase에서 인증하지 않는다.
- Accessibility permission이 필요한 automation을 추가하지 않는다.

## Acceptance Criteria

- smoke harness가 환경 변수로만 활성화된다. `done`
- command output snapshot smoke script가 추가되어 있다. `done`
- 기존 일반 앱 실행 경로는 smoke env가 없으면 바뀌지 않는다. `done`
- matrix와 known gaps가 갱신되어 있다. `done`
- `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, app command smoke가 통과한다. `done`

## Implementation Update - 2026-06-11

Status: implementation complete.

구현된 내용:

- `smoke.rs`를 추가해 smoke 전용 환경 변수 기반 input injection과 snapshot dump를 구현했다.
- app launch 후 smoke env가 있을 때만 background smoke worker를 시작한다.
- `scripts/run-app-command-smoke.sh`를 추가했다.
- compatibility matrix, smoke test 문서, known gaps, README를 갱신했다.

검증:

- `scripts/run-app-command-smoke.sh`
