# Phase 047: fzf Preview App Smoke

## Purpose

Phase 047의 목적은 app 내부 PTY에서 `fzf --preview` workflow가 query 입력 뒤 preview pane을 갱신하는지 자동 확인하는 것이다.

Phase 039와 Phase 043, Phase 046은 redraw, selection, multi-select를 확인했다. 이 phase에서는 preview window rendering evidence를 추가한다.

## Scope

Phase 047에서 다룰 작업:

1. `scripts/run-app-target-smokes.sh`에 `fzf-preview` target 추가
2. app 내부 PTY에서 `printf "alpha\nbeta\n" | fzf --preview "printf preview:{}"` 실행
3. follow-up `b` 입력
4. terminal snapshot marker `preview:beta` 확인
5. matrix와 known gaps 갱신

## Proposed Work Breakdown

### Step 1. Add fzf Preview Target

- `fzf --preview "printf preview:{}"`를 실행한다.
- follow-up input `b`를 보낸다.
- preview pane marker `preview:beta`가 snapshot에 출력되는지 확인한다.

완료 기준:

- local environment에서 `fzf-preview` app target smoke가 통과한다.

### Step 2. Update Evidence

- `fzf` matrix row에 preview workflow evidence를 연결한다.
- Environment-dependent App Smoke Target gap에서 `fzf` preview 항목을 정리한다.

완료 기준:

- fzf redraw, preview, selection, multi-select evidence가 문서에서 구분되어 있다.

## Non-goals

- `fzf`의 모든 preview option을 인증하지 않는다.
- shell integration을 인증하지 않는다.
- GUI focus 기반 keyboard automation을 추가하지 않는다.

## Acceptance Criteria

- [done] `fzf-preview` app target smoke가 추가되어 있다.
- [done] 설치된 환경에서 fzf preview marker가 확인된다.
- [done] matrix와 known gaps가 갱신되어 있다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## Implementation Update - 2026-06-11

상태: fzf preview workflow evidence 구현 완료.

구현된 내용:

- `scripts/run-app-target-smokes.sh`에 `fzf-preview` target을 추가했다.
- app 내부 PTY에서 `fzf --preview`에 query `b`를 보내 preview marker `preview:beta`를 확인한다.
- compatibility matrix, smoke test 문서, known gaps, README를 갱신했다.

검증:

- `scripts/run-compatibility-core.sh`
- `cargo test`
- `scripts/run-app-smoke.sh`
- `scripts/run-app-command-smoke.sh`
- `scripts/run-app-target-smokes.sh`
