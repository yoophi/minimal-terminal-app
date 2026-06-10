# Phase 031: OSC 52 Clipboard Query Policy

## Purpose

Phase 031의 목적은 OSC 52 clipboard query/readback gap을 보안 정책이 있는 명시적 동작으로 바꾸는 것이다.

Phase 023은 OSC 52 write만 지원하고 query `?`는 safe ignore했다. 이 phase에서는 query를 parser/state에서 인식하되, local pasteboard를 읽지 않는 deny-by-default 정책으로 처리한다.

## Scope

Phase 031에서 다룰 작업:

1. `OSC 52 ; Pc ; ?` query parser action 추가
2. core response queue에 clipboard 내용을 포함하지 않는 empty OSC 52 response 추가
3. parser/state/compatibility test evidence 추가
4. matrix와 known gaps 갱신

## Proposed Work Breakdown

### Step 1. Parse Query Separately

- `OSC 52 ; Pc ; ? BEL`을 malformed payload와 구분한다.
- `Pc` selector는 UTF-8과 작은 크기 제한을 통과한 경우만 응답에 사용한다.

완료 기준:

- parser test가 query를 별도 action으로 검증한다.

### Step 2. Deny Readback by Default

- terminal core는 local clipboard를 읽지 않는다.
- query에는 `OSC 52 ; Pc ; BEL` 형태의 empty payload response를 큐잉한다.
- clipboard write queue에는 아무 것도 추가하지 않는다.

완료 기준:

- state/compatibility test가 empty response와 no clipboard write를 검증한다.

## Non-goals

- 실제 clipboard readback을 구현하지 않는다.
- 사용자 prompt/permission UI를 구현하지 않는다.
- OSC 52 selector별 세부 permission policy를 구현하지 않는다.

## Acceptance Criteria

- OSC 52 query parser action이 있다. `done`
- OSC 52 query가 clipboard 내용을 읽지 않고 empty response를 큐잉한다. `done`
- OSC 52 write 동작은 유지된다. `done`
- matrix와 known gaps가 갱신되어 있다. `done`
- `scripts/run-compatibility-core.sh`, `cargo test`, app smoke가 통과한다. `done`

## Implementation Update - 2026-06-11

Status: implementation complete.

구현된 내용:

- parser가 `OSC 52 ; Pc ; ?`를 `ClipboardQueryDenied` action으로 변환한다.
- `TerminalState`가 query에 대해 local clipboard를 읽지 않고 `OSC 52 ; Pc ; BEL` empty response를 큐잉한다.
- OSC 52 write path는 기존 pending clipboard write queue를 유지한다.
- parser/state/compatibility test evidence를 추가했다.
- matrix와 known gaps를 갱신했다.

검증:

- `scripts/run-compatibility-core.sh`
- `cargo test`
