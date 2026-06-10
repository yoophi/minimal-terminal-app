# Phase 042: Git Pager Page Navigation App Smoke

## Purpose

Phase 042의 목적은 app 내부 PTY에서 `git log ... | less` pager에 진입한 뒤 page navigation 입력과 quit 입력이 순서대로 처리되는 workflow를 자동 확인하는 것이다.

Phase 040은 pager quit만 확인했다. 이 phase에서는 `less` 내부 page navigation key path까지 smoke coverage를 넓힌다.

## Scope

Phase 042에서 다룰 작업:

1. smoke harness에 두 번째 follow-up input을 추가
2. `scripts/run-app-target-smokes.sh`에 `git-pager-page-quit` target 추가
3. app 내부 PTY에서 `git log ... | less` 실행
4. follow-up Space 입력 후 두 번째 follow-up `q` 입력으로 shell marker 확인
5. matrix와 known gaps 갱신

## Proposed Work Breakdown

### Step 1. Add Staged Follow-up Input

- `MINIMAL_TERMINAL_SMOKE_SECOND_FOLLOWUP_INPUT`을 추가한다.
- `MINIMAL_TERMINAL_SMOKE_SECOND_FOLLOWUP_INPUT_DELAY_MS`로 두 번째 입력 지연 시간을 조정한다.

완료 기준:

- app smoke harness가 command input, first follow-up, second follow-up을 순서대로 PTY에 쓴다.

### Step 2. Add Git Pager Page Navigation Target

- `git log --oneline --graph --decorate -100 --color=never | less`를 실행한다.
- first follow-up Space로 page navigation을 보낸다.
- second follow-up `q`로 pager를 종료한다.
- 종료 뒤 shell marker `git-pager-page-quit-ok`를 확인한다.

완료 기준:

- local environment에서 `git-pager-page-quit` app target smoke가 통과한다.

### Step 3. Update Evidence

- `git log --oneline --graph --decorate` matrix row에 page navigation evidence를 연결한다.
- Environment-dependent App Smoke Target gap에서 pager scroll/page navigation 항목을 정리한다.

완료 기준:

- pager quit과 pager page navigation evidence가 문서에서 구분되어 있다.

## Non-goals

- `less`의 모든 navigation key를 인증하지 않는다.
- GUI focus 기반 keyboard automation을 추가하지 않는다.
- pager search, mark, horizontal scroll workflow를 인증하지 않는다.

## Acceptance Criteria

- [done] smoke harness가 second follow-up input을 지원한다.
- [done] `git-pager-page-quit` app target smoke가 추가되어 있다.
- [done] 설치된 환경에서 pager page navigation 뒤 quit marker가 확인된다.
- [done] matrix와 known gaps가 갱신되어 있다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## Implementation Update - 2026-06-11

상태: git pager page navigation workflow evidence 구현 완료.

구현된 내용:

- `crates/terminal-app/src/smoke.rs`에 second follow-up input 환경 변수를 추가했다.
- `scripts/run-app-target-smokes.sh`에 `run_case_with_two_followups`와 `git-pager-page-quit` target을 추가했다.
- app 내부 PTY에서 `less` pager에 Space를 보낸 뒤 `q`를 보내 marker `git-pager-page-quit-ok`를 확인한다.
- compatibility matrix, smoke test 문서, known gaps, README를 갱신했다.

검증:

- `scripts/run-compatibility-core.sh`
- `cargo test`
- `scripts/run-app-smoke.sh`
- `scripts/run-app-command-smoke.sh`
- `scripts/run-app-target-smokes.sh`
