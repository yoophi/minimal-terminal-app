# Phase 015: App Runtime Smoke Automation

## Purpose

Phase 015의 목적은 AppKit 앱의 최소 runtime smoke를 자동화하는 것이다.

Core test는 자동화되어 있지만, app bundle 생성과 native app binary 생존 여부는 별도 확인이 필요하다. Phase 015는 완전한 GUI 자동화가 아니라 로컬 macOS 환경에서 반복 가능한 최소 smoke를 제공한다.

## Scope

Phase 015에서 다룰 작업:

1. `scripts/run-app-smoke.sh` 추가
2. app bundle build 확인
3. app binary 직접 실행 확인
4. 짧은 시간 process 생존 확인
5. 가능한 경우 startup/PTY log 확인
6. local-only 제한 문서화

## Proposed Work Breakdown

### Step 1. Build Smoke

- `scripts/bundle-macos-app.sh`를 실행한다.
- 생성된 `.app` 경로와 binary 존재를 확인한다.

완료 기준:

- app bundle 생성 실패를 script가 감지한다.

### Step 2. Runtime Smoke

- app binary를 직접 실행한다.
- 2-3초 생존 여부를 확인한다.
- 종료 처리를 안전하게 수행한다.

완료 기준:

- 초기화 직후 crash를 감지할 수 있다.

### Step 3. Logging Smoke

- 가능하면 `/usr/bin/log`로 startup/PTY 로그를 확인한다.
- 환경별 로그 접근 제한이 있으면 optional로 둔다.

완료 기준:

- 로그 확인 가능 여부가 문서화되어 있다.

## Non-goals

- GUI rendering pixel 검증
- IME 자동 입력
- pasteboard 자동 조작
- CI에서 항상 안정적으로 실행되는 GUI test

## Risks

### macOS Session Dependency

GUI app 실행은 display session, 권한, focus 상태에 영향을 받을 수 있다.

대응:

- local-only smoke로 명시한다.
- CI 필수 gate에는 core runner만 둔다.

## Acceptance Criteria

- `scripts/run-app-smoke.sh`가 있다. `done`
- app bundle build와 binary 생존 smoke를 수행한다. `done`
- 실패 시 non-zero exit를 반환한다. `done`
- `docs/compatibility/smoke-tests.md`가 script 사용법을 반영한다. `done`

## Implementation Update - 2026-06-10

Status: implementation complete.

구현된 내용:

- `scripts/run-app-smoke.sh`를 추가했다.
- script가 `scripts/bundle-macos-app.sh`를 실행해 app bundle을 새로 생성한다.
- app binary 존재와 실행 권한을 확인한다.
- app binary를 직접 실행하고 기본 3초 생존 여부를 확인한다.
- 실패하면 stdout/stderr log 경로를 출력하고 non-zero로 종료한다.
- `docs/compatibility/smoke-tests.md`, `docs/compatibility/test-strategy.md`, README에 실행 방법을 반영했다.

검증:

- `scripts/run-app-smoke.sh`
- `scripts/run-compatibility-core.sh`
- `cargo test`
