# Phase 116 - fzf Preview Smoke Stabilization

## Purpose

`fzf-preview` app target smoke가 fzf UI 준비 전에 query input을 보내 실패하는 문제를 안정화한다.

기존 target은 `fzf --preview` 실행 직후 1초 뒤 `b`를 입력했다. local verification environment에서 fzf preview pane이 늦게 준비되면 입력이 shell prompt로 전달되어 `preview:beta` marker를 확인하지 못했다.

## Scope

- `fzf-preview` target의 follow-up input delay와 snapshot delay를 늘린다.
- preview pane marker `preview:beta` 검증은 유지한다.
- `fzf` 구현이나 preview option 범위는 변경하지 않는다.

## Proposed Work Breakdown

1. `fzf-preview` 단독 실행으로 실패 원인을 확인한다.
2. fzf UI 준비 후 query가 들어가도록 delay 값을 조정한다.
3. target smoke suite가 `fzf-preview`를 지나갈 수 있는지 확인한다.
4. compatibility 문서에 smoke timing stabilization evidence를 기록한다.

## Acceptance Criteria

- [done] 지연 시간을 늘린 단독 `fzf-preview` app smoke에서 `preview:beta` marker가 확인된다.
- [done] `scripts/run-app-target-smokes.sh`의 `fzf-preview` target이 안정화된 delay를 사용한다.
- [done] compatibility 문서가 fzf preview smoke timing 안정화 사실을 기록한다.

## Non-goals

- `fzf` 전체 preview option 인증
- 사용자별 shell/plugin 설정 인증
- GUI focus 기반 keyboard automation 추가

## Result

상태: fzf preview smoke timing stabilization 완료.

검증한 단독 snapshot:

- `preview:beta`
- `▌ beta`
- query line `> b`
