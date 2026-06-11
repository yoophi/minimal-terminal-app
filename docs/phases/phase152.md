# Phase 152 - htop Mouse Open Files App Smoke

## 목적

`htop` mouse reporting workflow 중 하나를 app 내부 PTY smoke로 고정한다. 좌표 지정 synthetic mouse click report가 htop에 전달되어 process open-files 화면으로 진입하는지 확인한다.

## 범위

- smoke harness에 `left-click:<row>:<col>` 좌표 지정 mouse report를 추가한다.
- 좌표 지정 click은 press와 release report를 모두 보낸다.
- `scripts/run-app-target-smokes.sh`에 `htop-mouse-open-files` target을 추가한다.
- target은 app 내부 PTY에서 `htop` 실행 후 mouse click report를 보내고 `Snapshot of files open in process` marker를 확인한다.
- compatibility matrix, smoke-tests, known-gaps, app-readiness, README를 갱신한다.

## 완료 기준

- [done] `smoke::tests::parse_mouse_click_accepts_zero_based_row_and_col`가 추가되어 있다.
- [done] `htop-mouse-open-files` app target smoke가 추가되어 있다.
- [done] `htop-mouse-open-files` app target smoke가 local verification environment에서 통과한다.
- [done] 관련 compatibility 문서가 htop mouse workflow evidence를 반영한다.

## 비범위

- 실제 물리 mouse input end-to-end automation
- htop 모든 mouse action coverage
- htop setup 내부 특정 설정값 변경 workflow
