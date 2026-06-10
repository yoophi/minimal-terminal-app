# Phase 049: Git Pager Horizontal Scroll App Smoke

## Purpose

Phase 049의 목적은 app 내부 PTY에서 `git log ... | less -S` pager에 진입한 뒤 horizontal scroll 입력과 quit 입력이 순서대로 처리되는 workflow를 자동 확인하는 것이다.

Phase 040은 pager quit, Phase 042는 page navigation, Phase 048은 search를 확인했다. 이 phase에서는 horizontal scroll key path까지 smoke coverage를 넓힌다.

## Scope

Phase 049에서 다룰 작업:

1. `scripts/run-app-target-smokes.sh`에 `git-pager-horizontal-quit` target 추가
2. app 내부 PTY에서 긴 줄 형식의 `git log ... | less -S` 실행
3. follow-up Right Arrow, `q` 입력
4. shell marker `git-pager-horizontal-ok` 확인
5. matrix와 known gaps 갱신

## Proposed Work Breakdown

### Step 1. Add Git Pager Horizontal Scroll Target

- `git log --pretty=format:"%H %s" -100 --color=never | less -S`를 실행한다.
- follow-up input으로 Right Arrow sequence `ESC [ C`와 `q`를 보낸다.
- horizontal scroll 입력 뒤 pager가 `q`로 종료되고 shell marker가 출력되는지 확인한다.

완료 기준:

- local environment에서 `git-pager-horizontal-quit` app target smoke가 통과한다.

### Step 2. Update Evidence

- `git log --oneline --graph --decorate` matrix row에 horizontal scroll workflow evidence를 연결한다.
- Environment-dependent App Smoke Target gap에서 pager horizontal scroll 항목을 정리한다.

완료 기준:

- pager quit, page navigation, search, horizontal scroll evidence가 문서에서 구분되어 있다.

## Non-goals

- `less`의 모든 horizontal scroll option을 인증하지 않는다.
- mark workflow를 인증하지 않는다.
- GUI focus 기반 keyboard automation을 추가하지 않는다.

## Acceptance Criteria

- [done] `git-pager-horizontal-quit` app target smoke가 추가되어 있다.
- [done] 설치된 환경에서 pager horizontal scroll 뒤 quit marker가 확인된다.
- [done] matrix와 known gaps가 갱신되어 있다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## Implementation Update - 2026-06-11

상태: git pager horizontal scroll workflow evidence 구현 완료.

구현된 내용:

- `scripts/run-app-target-smokes.sh`에 `git-pager-horizontal-quit` target을 추가했다.
- app 내부 PTY에서 `less -S` pager에 Right Arrow와 `q`를 보내 marker `git-pager-horizontal-ok`를 확인한다.
- compatibility matrix, smoke test 문서, known gaps, README를 갱신했다.

검증:

- `scripts/run-compatibility-core.sh`
- `cargo test`
- `scripts/run-app-smoke.sh`
- `scripts/run-app-command-smoke.sh`
- `scripts/run-app-target-smokes.sh`
