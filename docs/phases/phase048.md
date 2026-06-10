# Phase 048: Git Pager Search App Smoke

## Purpose

Phase 048의 목적은 app 내부 PTY에서 `git log ... | less` pager에 진입한 뒤 search 입력과 quit 입력이 순서대로 처리되는 workflow를 자동 확인하는 것이다.

Phase 040은 pager quit, Phase 042는 page navigation을 확인했다. 이 phase에서는 `/검색어` search key path까지 smoke coverage를 넓힌다.

## Scope

Phase 048에서 다룰 작업:

1. `scripts/run-app-target-smokes.sh`에 `git-pager-search-quit` target 추가
2. app 내부 PTY에서 `git log ... | less` 실행
3. follow-up `/Implement`, Enter, `q` 입력
4. shell marker `git-pager-search-ok` 확인
5. matrix와 known gaps 갱신

## Proposed Work Breakdown

### Step 1. Add Git Pager Search Target

- `git log --oneline --graph --decorate -100 --color=never | less`를 실행한다.
- follow-up input으로 `/Implement\rq`를 보낸다.
- search 성공 뒤 pager가 `q`로 종료되고 shell marker가 출력되는지 확인한다.

완료 기준:

- local environment에서 `git-pager-search-quit` app target smoke가 통과한다.

### Step 2. Update Evidence

- `git log --oneline --graph --decorate` matrix row에 search workflow evidence를 연결한다.
- Environment-dependent App Smoke Target gap에서 pager search 항목을 정리한다.

완료 기준:

- pager quit, page navigation, search evidence가 문서에서 구분되어 있다.

## Non-goals

- `less`의 모든 search option을 인증하지 않는다.
- mark와 horizontal scroll workflow를 인증하지 않는다.
- GUI focus 기반 keyboard automation을 추가하지 않는다.

## Acceptance Criteria

- [done] `git-pager-search-quit` app target smoke가 추가되어 있다.
- [done] 설치된 환경에서 pager search 뒤 quit marker가 확인된다.
- [done] matrix와 known gaps가 갱신되어 있다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## Implementation Update - 2026-06-11

상태: git pager search workflow evidence 구현 완료.

구현된 내용:

- `scripts/run-app-target-smokes.sh`에 `git-pager-search-quit` target을 추가했다.
- app 내부 PTY에서 `less` pager에 `/Implement`, Enter, `q`를 보내 marker `git-pager-search-ok`를 확인한다.
- compatibility matrix, smoke test 문서, known gaps, README를 갱신했다.

검증:

- `scripts/run-compatibility-core.sh`
- `cargo test`
- `scripts/run-app-smoke.sh`
- `scripts/run-app-command-smoke.sh`
- `scripts/run-app-target-smokes.sh`
