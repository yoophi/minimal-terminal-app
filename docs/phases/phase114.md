# Phase 114 - Vim Resize Redraw App Smoke

## 목표

app 내부 PTY에서 clean `vim` 실행 중 terminal grid와 PTY size가 바뀐 뒤에도 redraw와 command input이 유지되는지 자동 smoke로 고정한다.

## 범위

- smoke harness에 `MINIMAL_TERMINAL_SMOKE_RESIZE=rowsxcols` hook을 추가한다.
- resize hook은 `TerminalBuffer::resize`와 `PtyWriter::resize`를 함께 호출한다.
- `scripts/run-app-target-smokes.sh`에 `vim-resize-redraw` target을 추가한다.
- clean `vim --clean -Nu NONE -n` 실행 중 `24x80` resize를 적용한다.
- Vim 내부에서 `&lines`와 `&columns`를 tempfile에 쓰고 shell 복원 marker로 확인한다.
- compatibility matrix, smoke test 문서, app readiness, known gaps, README를 새 evidence에 맞게 갱신한다.

## 제외 범위

- 실제 AppKit window drag/resize event end-to-end 검증
- Vim plugin 환경
- tmux 내부 resize workflow
- long-running redraw stress test

## 완료 기준

- `smoke::tests::parse_resize_*`가 통과한다.
- `scripts/run-app-target-smokes.sh`에서 `vim-resize-redraw`가 통과한다.
- app snapshot에 `vim-resize-result:lines=24 columns=80` marker가 남는다.
- compatibility 문서가 clean Vim resize redraw evidence를 반영한다.

## 결과

상태: 구현 완료.

- smoke harness가 resize hook으로 terminal buffer와 PTY window size를 함께 조정한다.
- local verification environment에서 app 내부 PTY로 clean Vim을 실행하고 resize 후 `&lines=24`, `&columns=80`을 확인했다.
- `vim`의 남은 대표 gap은 plugin 환경과 실제 AppKit window resize end-to-end workflow로 좁혔다.
