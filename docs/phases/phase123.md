# Phase 123 - fzf Preview Query Stabilization

## Purpose

`fzf-preview` app target smoke가 follow-up query input timing에 따라 query가 shell prompt로 전달되는 flake를 제거한다.

## Scope

- `fzf-preview` target에서 follow-up input 대신 `fzf --query beta`를 사용한다.
- preview pane marker `preview:beta` 검증은 유지한다.
- compatibility 문서와 README에 preloaded query stabilization evidence를 기록한다.

## Proposed Work Breakdown

1. `fzf-preview` command에 `--query beta`를 추가한다.
2. `run_case_with_followup` 대신 `run_case`로 preview marker를 확인한다.
3. 전체 app target smoke suite를 실행한다.
4. 관련 문서를 갱신한다.

## Acceptance Criteria

- [done] `fzf-preview` target이 follow-up query timing에 의존하지 않는다.
- [done] 전체 app target smoke suite가 통과한다.
- [done] fzf preview evidence가 compatibility 문서에 기록되어 있다.

## Non-goals

- `fzf` 전체 preview option 인증
- 사용자별 shell/plugin 설정 인증
- GUI focus 기반 keyboard automation 추가

## Result

상태: 구현 완료.

`fzf-preview`는 `--query beta`로 preview pane을 준비하고 `preview:beta` marker를 확인한다.
