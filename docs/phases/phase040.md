# Phase 040: git Pager Quit App Smoke

## Purpose

Phase 040의 목적은 app 내부 PTY에서 `git log` 출력이 pager에 들어가고, follow-up `q` 입력으로 pager를 종료하는 workflow를 자동 확인하는 것이다.

Phase 033은 non-pager `git log --oneline -1`만 확인했다. 이 phase에서는 `less` pager를 명시적으로 사용해 quit workflow marker를 검증한다.

## Scope

Phase 040에서 다룰 작업:

1. `scripts/run-app-target-smokes.sh`에 `git-pager-quit` target 추가
2. app 내부 PTY에서 `git log ... | less` 실행
3. follow-up `q` 입력 후 marker 출력 확인
4. matrix와 known gaps 갱신

## Proposed Work Breakdown

### Step 1. Add Pager Quit Target

- `git log --oneline --graph --decorate -100 --color=never | less`를 실행한다.
- follow-up input `q`를 보낸다.
- pager가 종료된 뒤 shell marker `git-pager-quit-ok`가 출력되는지 확인한다.

완료 기준:

- local environment에서 `git-pager-quit` app target smoke가 통과한다.

### Step 2. Update Evidence

- `git log` matrix row에 pager quit evidence를 연결한다.
- pager scroll workflow는 별도 gap으로 유지한다.

완료 기준:

- quit workflow와 scroll workflow가 문서에서 구분되어 있다.

## Non-goals

- pager scroll/page navigation을 인증하지 않는다.
- 모든 git pager 설정 조합을 인증하지 않는다.
- GUI keyboard automation을 추가하지 않는다.

## Acceptance Criteria

- `git-pager-quit` app target smoke가 추가되어 있다. `done`
- 설치된 환경에서 pager quit marker가 확인된다. `done`
- matrix와 known gaps가 갱신되어 있다. `done`
- `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다. `done`

## Implementation Update - 2026-06-11

Status: implementation complete for git pager quit evidence.

구현된 내용:

- `scripts/run-app-target-smokes.sh`에 `git-pager-quit` target을 추가했다.
- app 내부 PTY에서 `git log ... | less`를 실행하고 follow-up `q` 뒤 marker `git-pager-quit-ok`를 확인한다.
- compatibility matrix, smoke test 문서, known gaps, README를 갱신했다.

검증:

- `scripts/run-app-target-smokes.sh`
