# Phase 126: Native AppKit Window Resize Smoke

## 목적

Phase 114의 resize smoke는 smoke harness가 terminal buffer와 PTY size를 직접 조정했다. 이 phase는 실제 AppKit window content size 변경이 `drawRect`와 app resize path를 지나 PTY window size로 반영되는지 자동 evidence를 추가한다.

## 범위

- `MINIMAL_TERMINAL_SMOKE_NATIVE_WINDOW_RESIZE=rowsxcols` hook을 추가한다.
- hook은 main-thread `TerminalView` redraw timer에서 `NSWindow` content size를 조정한다.
- 조정된 view bounds가 기존 `apply_resize_if_needed` 경로를 통해 terminal buffer와 PTY resize로 이어지는지 확인한다.
- `scripts/run-app-target-smokes.sh`에 `native-window-resize` target을 추가한다.
- compatibility matrix, smoke-tests, known-gaps, app-readiness, README를 갱신한다.

## 완료 기준

- [done] native resize target이 app snapshot에 `native-window-resize-after:24 80` marker를 남긴다.
- [done] `cargo test`가 통과한다.
- [done] `scripts/run-app-target-smokes.sh` 전체 suite가 통과한다.
- [done] 관련 compatibility 문서가 native AppKit window resize evidence를 반영한다.

## 비범위

- 사용자가 손으로 창을 드래그하는 수동 workflow 자동화
- tmux 사용자 설정별 resize workflow 전체 인증
- long-running resize stress test
