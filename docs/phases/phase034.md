# Phase 034: Representative CLI Version App Smoke

## Purpose

Phase 034의 목적은 대표 CLI/TUI target의 version-level smoke를 native app 내부 PTY에서도 자동 확인하는 것이다.

Phase 019는 local shell에서 `tmux`, `htop`, `claude`, `codex-cli` version smoke를 확인했다. Phase 032-033에서 app-internal smoke harness가 생겼으므로, 설치된 target은 앱 내부 PTY snapshot evidence로 보강한다.

## Scope

Phase 034에서 다룰 작업:

1. `tmux -V` app-internal smoke 추가
2. `htop --version` app-internal smoke 추가
3. `claude --version` app-internal smoke 추가
4. `codex-cli --version`은 설치된 경우만 app-internal smoke 실행
5. matrix와 known gaps 갱신

## Proposed Work Breakdown

### Step 1. Extend Target Smoke Script

- `scripts/run-app-target-smokes.sh`에 version-level targets를 추가한다.
- target이 없으면 skip으로 출력한다.

완료 기준:

- 설치된 target의 marker가 app snapshot에서 확인된다.

### Step 2. Update Evidence

- matrix의 `tmux`, `htop`, `claude`, `codex-cli` 근거를 app-internal smoke 범위에 맞게 갱신한다.
- interactive workflow는 known gap으로 유지한다.

완료 기준:

- version smoke와 interactive certification이 문서에서 구분되어 있다.

## Non-goals

- `tmux` pane workflow를 인증하지 않는다.
- `htop` redraw/function key workflow를 인증하지 않는다.
- `claude`/`codex-cli` authenticated interactive prompt를 인증하지 않는다.

## Acceptance Criteria

- installed representative CLI version targets가 app-internal snapshot smoke로 검증된다. `done`
- missing target은 실패가 아니라 skip으로 기록된다. `done`
- matrix와 known gaps가 갱신되어 있다. `done`
- `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다. `done`

## Implementation Update - 2026-06-11

Status: implementation complete.

구현된 내용:

- `scripts/run-app-target-smokes.sh`에 `tmux-version`, `htop-version`, `claude-version`, `codex-cli-version`을 추가했다.
- 현재 local environment에서 `tmux`, `htop`, `claude` app-internal version smoke가 통과했다.
- 현재 PATH에서 `codex-cli`는 찾지 못해 `codex-cli-version`은 skip된다.
- compatibility matrix, smoke test 문서, known gaps, README를 갱신했다.

검증:

- `scripts/run-app-target-smokes.sh`
