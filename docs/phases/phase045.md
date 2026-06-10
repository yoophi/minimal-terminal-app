# Phase 045: Mode-gated SGR Mouse Report App Smoke

## Purpose

Phase 045의 목적은 app 내부 PTY 런타임에서 mouse reporting mode가 켜진 뒤 앱 mouse report encoder가 SGR mouse report를 PTY로 쓰는 경로를 자동 확인하는 것이다.

Phase 024와 Phase 026은 mouse report encoding을 code-level로 검증했다. 이 phase에서는 terminal mode parsing, app smoke hook, PTY readback을 결합해 runtime evidence를 추가한다.

## Scope

Phase 045에서 다룰 작업:

1. smoke harness에 `MINIMAL_TERMINAL_SMOKE_MOUSE_REPORT` 추가
2. terminal buffer의 `mouse_reporting` / `sgr_mouse` mode를 확인한 뒤 mouse report 전송
3. `scripts/run-app-target-smokes.sh`에 `mouse-sgr-report` target 추가
4. app 내부 PTY에서 SGR mouse mode enable 후 report bytes readback 확인
5. matrix와 known gaps 갱신

## Proposed Work Breakdown

### Step 1. Add Mode-gated Mouse Smoke Hook

- smoke harness가 terminal buffer snapshot의 mouse mode를 확인한다.
- `mouse_reporting`이 꺼져 있으면 report를 쓰지 않는다.
- `sgr_mouse`가 켜져 있으면 SGR report를 쓰고, 아니면 legacy report를 쓴다.

완료 기준:

- app smoke hook이 mode-gated mouse report bytes를 PTY에 쓸 수 있다.

### Step 2. Add SGR Mouse Target

- app 내부 shell에서 raw mode를 켠다.
- `CSI ? 1000 h`와 `CSI ? 1006 h`를 출력해 mouse reporting과 SGR mouse mode를 켠다.
- smoke hook이 left press report를 보낸다.
- shell이 report bytes를 hex로 읽고 marker `mouse-sgr-report:1b5b3c303b333b324d`를 출력한다.

완료 기준:

- local environment에서 `mouse-sgr-report` app target smoke가 통과한다.

## Non-goals

- GUI synthetic `NSEvent`를 생성하지 않는다.
- `vim`/`less`의 실제 mouse workflow를 인증하지 않는다.
- 모든 xterm mouse mode variant를 인증하지 않는다.

## Acceptance Criteria

- [done] mode-gated mouse smoke hook이 추가되어 있다.
- [done] `mouse-sgr-report` app target smoke가 추가되어 있다.
- [done] 설치된 환경에서 SGR mouse report bytes marker가 확인된다.
- [done] matrix와 known gaps가 갱신되어 있다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## Implementation Update - 2026-06-11

상태: mode-gated SGR mouse report runtime evidence 구현 완료.

구현된 내용:

- `crates/terminal-app/src/smoke.rs`에 `MINIMAL_TERMINAL_SMOKE_MOUSE_REPORT` hook을 추가했다.
- hook은 terminal buffer의 mouse mode를 확인한 뒤 app mouse encoder로 report bytes를 생성한다.
- `scripts/run-app-target-smokes.sh`에 `mouse-sgr-report` target을 추가했다.
- compatibility matrix, smoke test 문서, known gaps, README를 갱신했다.

검증:

- `scripts/run-compatibility-core.sh`
- `cargo test`
- `scripts/run-app-smoke.sh`
- `scripts/run-app-command-smoke.sh`
- `scripts/run-app-target-smokes.sh`
