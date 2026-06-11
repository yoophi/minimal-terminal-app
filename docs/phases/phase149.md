# Phase 149 - htop VT Index Scroll Fix

## 목표

`htop` 같은 ncurses TUI에서 화면 일부가 위아래로 밀릴 때 레이아웃이 깨지지 않도록 VT index/scroll 계열 시퀀스를 구현한다.

## 범위

- `ESC D` Index
- `ESC E` Next Line
- `ESC M` Reverse Index
- `CSI Ps S` Scroll Up
- `CSI Ps T` Scroll Down
- scroll region 안에서 현재 배경 스타일을 유지하는 blank line 삽입

## 완료 기준

- parser가 위 시퀀스를 별도 action으로 변환한다.
- state/grid가 scroll region 안에서 up/down scroll을 적용한다.
- `cargo test -p terminal-core vt_index`, `cargo test -p terminal-core csi_scroll`, `cargo test -p terminal-core parses_tui_editing_sequences`가 통과한다.
- app target smoke에서 `htop-runtime`, `htop-f10-quit`, `htop-f1-help-quit`, `htop-f5-tree`, `htop-f2-setup`, `htop-setup-save`가 통과한다.

## 비목표

- htop의 모든 mouse workflow 인증
- vttest 전체 항목 자동 인증
