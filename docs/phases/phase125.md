# Phase 125: Background Color Erase for htop Layout

## 목적

`htop`처럼 colored background와 공백 cell을 함께 사용하는 TUI에서 `EL`, `ED`, `ECH`, insert/delete 동작이 기본 스타일 공백을 남기면 meter/header 영역이 시각적으로 깨질 수 있다. 이 phase는 xterm 계열 `bce` 기대 동작에 맞춰 현재 SGR 배경 스타일을 가진 blank cell을 남기도록 수정한다.

## 범위

- `Cell`에 현재 style 기반 blank/clear helper를 추가한다.
- CSI erase, erase chars, insert/delete chars, insert/delete lines가 현재 style을 blank cell에 적용한다.
- core test로 background style 보존을 고정한다.
- htop runtime smoke와 compatibility 문서를 갱신한다.

## 완료 기준

- [done] `state::tests::erase_and_blank_operations_preserve_current_background_style`가 추가되어 있다.
- [done] `cargo test`가 통과한다.
- [done] `scripts/run-app-target-smokes.sh`의 `htop-runtime`과 전체 suite가 통과한다.
- [done] compatibility matrix, smoke-tests, known-gaps, README가 Phase 125 evidence를 반영한다.

## 비범위

- full xterm erase semantics 전체 인증
- 실제 물리 mouse 기반 htop workflow
- htop setup 내부 특정 설정값 변경 workflow
