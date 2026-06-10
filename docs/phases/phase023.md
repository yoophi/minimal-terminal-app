# Phase 023: OSC 52 Clipboard Write

## Purpose

Phase 023의 목적은 OSC 52 clipboard write를 제한된 범위로 지원해 terminal-aware CLI의 clipboard integration gap을 줄이는 것이다.

Phase 018 이후 OSC 52는 보안 정책이 필요한 미지원 항목으로 남아 있었다. 이 phase에서는 clipboard query/readback은 제외하고, PTY 출력이 요청한 clipboard write만 AppKit pasteboard에 반영한다.

## Scope

Phase 023에서 다룰 작업:

1. `OSC 52 ; Pc ; Pd` parser handling 추가
2. base64 payload decode와 UTF-8 검증
3. core pending clipboard write queue 추가
4. AppKit main thread pasteboard write 경로 추가
5. compatibility evidence와 문서 갱신

## Proposed Work Breakdown

### Step 1. Parse OSC 52

- `OSC 52 ; Pc ; Pd BEL`
- `Pd`는 base64 payload다.
- malformed payload와 query `?`는 safe ignore한다.

완료 기준:

- parser test가 valid write, query ignore, invalid payload ignore를 검증한다.

### Step 2. Queue Clipboard Write

- core는 clipboard write를 즉시 실행하지 않고 pending queue에 보관한다.
- terminal-app은 main thread timer에서 pending clipboard write를 drain한다.

완료 기준:

- state/compatibility test가 pending clipboard write를 검증한다.

### Step 3. Apply Pasteboard Write

- AppKit pasteboard write는 main thread에서 수행한다.
- clipboard 내용은 log에 남기지 않는다.

완료 기준:

- app build와 smoke가 통과한다.

## Non-goals

- OSC 52 query/readback을 구현하지 않는다.
- 사용자 prompt/permission UI를 구현하지 않는다.
- 모든 OSC extension을 구현하지 않는다.
- clipboard payload size policy를 세밀하게 설정하지 않는다.

## Risks

### Clipboard Security

OSC 52는 shell program이 clipboard를 바꿀 수 있게 한다.

대응:

- 이번 phase는 write-only로 제한하고 query/readback은 구현하지 않는다.
- payload는 UTF-8과 크기 제한을 통과한 경우에만 큐잉한다.
- clipboard 내용을 로그에 남기지 않는다.

## Acceptance Criteria

- OSC 52 valid write parser/state test가 있다. `done`
- OSC 52 query와 invalid payload가 safe ignore된다. `done`
- AppKit pasteboard write가 main thread path에 연결되어 있다. `done`
- matrix와 known gaps가 갱신되어 있다. `done`
- `scripts/run-compatibility-core.sh`와 `cargo test`가 통과한다. `done`

## Implementation Update - 2026-06-11

Status: implementation complete for OSC 52 clipboard write.

구현된 내용:

- `base64` dependency를 추가했다.
- parser가 OSC 52 clipboard write를 `SetClipboard` action으로 변환한다.
- `TerminalState`가 pending clipboard write queue를 제공한다.
- `TerminalBuffer`와 `TerminalView`가 pending clipboard write를 main thread pasteboard write로 반영한다.
- parser/state/compatibility test evidence를 추가했다.
- matrix와 known gaps를 갱신했다.

검증:

- `cargo test -p terminal-core osc52`
- `scripts/run-compatibility-core.sh`
- `cargo test`
