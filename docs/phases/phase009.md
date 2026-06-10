# Phase 009: Compatibility Documentation Consistency Checks

## Purpose

Phase 009의 목적은 `docs/compatibility` 문서와 실제 테스트 evidence가 서로 어긋나지 않도록 자동 검증을 추가하는 것이다.

Phase 008에서 compatibility matrix, known gaps, smoke protocol, test strategy를 만들었지만 문서와 코드가 시간이 지나며 달라질 수 있다. Phase 009에서는 문서 자체를 회귀 대상으로 보고, supported/unknown/not supported 상태가 evidence와 일관되는지 확인하는 script를 만든다.

## Scope

Phase 009에서 다룰 작업:

1. `scripts/check-compatibility-docs.sh` 추가
2. `matrix.md`의 status/evidence 기본 형식 검증
3. `supported` row에 evidence가 있는지 확인
4. `unknown`, `partially supported`, `not supported` row가 `known-gaps.md`와 연결되는지 확인
5. `tests/compatibility.rs::test_name` 형식의 evidence가 실제 test file에 존재하는지 확인
6. `docs/compatibility` 내부 주요 markdown link 존재 여부 확인

## Proposed Work Breakdown

### Step 1. Define Document Rules

- `supported` row는 빈 evidence를 허용하지 않는다.
- `unknown`, `partially supported`, `not supported` row는 known gap 또는 smoke 문서와 연결한다.
- test evidence는 실제 test name과 맞아야 한다.
- 내부 문서 링크는 존재해야 한다.

완료 기준:

- 문서 검증 규칙이 script 또는 문서에 명시되어 있다.

### Step 2. Implement Check Script

- `scripts/check-compatibility-docs.sh`를 추가한다.
- shell script로 시작하되 필요하면 이후 Rust/Python 도구로 승격한다.
- `rg`, `awk`, `grep` 등 기본 CLI로 검증 가능한 범위부터 구현한다.

완료 기준:

- script가 실패 시 원인을 출력한다.
- 현재 문서 상태에서 script가 통과한다.

### Step 3. Integrate With Runner

- `scripts/run-compatibility-core.sh`에서 문서 검증을 같이 실행할지 결정한다.
- core test와 문서 검증을 분리할 필요가 있으면 README에 실행 순서를 명시한다.

완료 기준:

- Phase 008 test strategy의 권장 실행 순서가 실제 script와 일치한다.

## Non-goals

- markdown parser를 완전 구현하지 않는다.
- 모든 자연어 evidence의 정확성을 자동 판단하지 않는다.
- 외부 URL 생존 여부를 매번 네트워크로 확인하지 않는다.

## Risks

### Script Fragility

문서 형식이 조금만 바뀌어도 script가 깨질 수 있다.

대응:

- 검증 규칙을 단순하게 유지한다.
- matrix table 구조를 안정적으로 유지한다.

## Acceptance Criteria

- `scripts/check-compatibility-docs.sh`가 있다. `done`
- 현재 `docs/compatibility` 문서에서 script가 통과한다. `done`
- `supported` row의 evidence 누락을 잡아낼 수 있다. `done`
- `unknown`/`partially supported`/`not supported` row의 known gap 연결 누락을 잡아낼 수 있다. `done`
- `scripts/run-compatibility-core.sh` 또는 README에 실행 방법이 반영되어 있다. `done`
- `cargo test`가 통과한다. `done`

## Implementation Update - 2026-06-10

Status: implementation complete.

구현된 내용:

- `scripts/check-compatibility-docs.sh`를 추가했다.
- `supported` row의 evidence 누락을 검증한다.
- `unknown`, `partially supported`, `not supported` row가 `known-gaps.md`와 연결되는지 검증한다.
- `tests/compatibility.rs::test_name` evidence가 실제 compatibility integration test에 존재하는지 검증한다.
- `docs/compatibility` 내부 markdown link 대상이 존재하는지 검증한다.
- `scripts/run-compatibility-core.sh`에 문서 검증을 연결했다.
- README와 regression runner 문서에 실행 방법을 반영했다.

검증:

- `scripts/check-compatibility-docs.sh`
- `scripts/run-compatibility-core.sh`
- `cargo test`
