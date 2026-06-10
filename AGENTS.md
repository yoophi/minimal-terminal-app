# AGENTS.md

작업 전 반드시 현재 대상 phase 문서를 먼저 확인한다.

## Phase 확인

- Phase 문서 위치: `docs/phases/phaseNNN.md`
- 작업 시작 전 확인할 내용:
  - `Purpose`
  - `Scope`
  - `Proposed Work Breakdown`
  - `Acceptance Criteria`
  - `Non-goals`

## Compatibility 작업

터미널 호환성 관련 작업은 함께 확인한다.

- `docs/compatibility/matrix.md`
- `docs/compatibility/known-gaps.md`
- `docs/compatibility/test-strategy.md`
- `docs/compatibility/app-readiness.md`

`supported`로 표시하는 항목은 테스트 또는 runtime evidence를 연결한다. 구현하지 못한 항목은 `known-gaps.md`에 남긴다.

## 완료 기준

phase 작업을 마치면 해당 phase의 `Acceptance Criteria`를 기준으로 확인하고, 필요한 경우 관련 compatibility 문서를 함께 갱신한다.
