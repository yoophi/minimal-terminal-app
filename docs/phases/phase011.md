# Phase 011: Device Status Report Responses

## Purpose

Phase 011의 목적은 Device Status Report, 특히 `CSI 5 n`과 `CSI 6 n` 요청에 응답하는 경로를 구현하는 것이다.

현재 matrix에서 Device Status Report는 `not supported`다. 이 기능은 parser state만으로 끝나지 않고, terminal이 PTY로 응답을 써야 하므로 core와 app layer 사이의 response 경로가 필요하다.

## Scope

Phase 011에서 다룰 작업:

1. DSR parser action 추가
2. `TerminalState`에서 response 생성
3. app layer에서 response를 PTY writer로 전달
4. parser/state/compatibility test 추가
5. matrix와 known gaps 갱신

## Proposed Work Breakdown

### Step 1. Parser Action

- `CSI 5 n`을 status report 요청으로 인식한다.
- `CSI 6 n`을 cursor position report 요청으로 인식한다.
- 알 수 없는 `CSI Ps n`은 안전하게 무시한다.

완료 기준:

- parser test가 DSR action을 검증한다.

### Step 2. State Response Queue

- `TerminalState::append_bytes` 처리 중 response byte를 보존할 방법을 추가한다.
- 예: `take_pending_responses()` 또는 append 결과에 response를 포함하는 방식.
- `CSI 5 n` 응답은 최소 `ESC[0n` 계열을 검토한다.
- `CSI 6 n` 응답은 cursor row/col을 1-based로 `ESC[row;colR` 형태로 보낸다.

완료 기준:

- state test가 cursor position response를 검증한다.

### Step 3. App Integration

- `TerminalBuffer::append_bytes` 또는 PTY reader path에서 pending response를 app writer로 전달할 수 있는 구조를 설계한다.
- thread/lock 구조에서 deadlock을 만들지 않도록 주의한다.

완료 기준:

- DSR 요청이 들어왔을 때 PTY로 response가 쓰인다.

## Non-goals

- 모든 status report variant를 구현하지 않는다.
- terminal identification 전체 응답은 별도 phase로 남길 수 있다.

## Risks

### Response Path Complexity

현재 output read path는 buffer에 bytes를 append하는 구조다. 여기에 PTY write response를 섞으면 lock 순서 문제가 생길 수 있다.

대응:

- response queue를 분리하고, app layer에서 writer lock을 짧게 잡는다.

## Acceptance Criteria

- `CSI 5 n`, `CSI 6 n` parser/state test가 있다.
- PTY response path가 구현되어 있다.
- `docs/compatibility/matrix.md`의 Device Status Report row가 갱신되어 있다.
- `docs/compatibility/known-gaps.md`에서 해당 gap이 제거되거나 제한이 명확히 낮춰져 있다.
- `cargo test`와 `scripts/run-compatibility-core.sh`가 통과한다.

