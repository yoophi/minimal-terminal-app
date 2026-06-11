# Phase 134 - htop DEC Autowrap Mode Fix

## 목표

`htop` runtime 화면에서 right margin과 하단 function-key row redraw가 깨질 수 있는 문제를 줄인다.

## 원인

- `htop`은 `CSI ? 7 h/l` DEC autowrap private mode로 우측 끝 wrap 동작을 제어한다.
- 기존 구현은 deferred autowrap 자체는 처리했지만, autowrap mode on/off 상태는 무시했다.
- 따라서 `htop`처럼 화면 마지막 column 근처를 직접 제어하는 TUI에서 다음 printable 출력이 원하지 않는 줄바꿈을 만들 수 있었다.

## 구현

- `terminal-core` parser가 `CSI ? 7 h`를 autowrap on, `CSI ? 7 l`을 autowrap off action으로 변환한다.
- `TerminalModes`가 autowrap 상태를 보관한다.
- autowrap off 상태에서는 right margin 다음 printable 출력이 다음 줄로 넘어가지 않고 마지막 cell을 갱신한다.
- autowrap을 다시 켜면 이후 right margin 출력은 기존 deferred wrap 경로를 따른다.
- `htop-runtime` smoke marker를 `Mem[`과 `F1Help  F2Setup`까지 강화했다.

## 제외 범위

- `htop` mouse workflow
- `htop` setup 내부 특정 설정값 변경 검증
- full xterm private mode coverage

## 완료 기준

- `parser::tests::parses_tui_private_modes`가 `CSI ? 7 h/l` parsing을 검증한다.
- `state::tests::autowrap_mode_can_disable_right_margin_wrap`와 `state::tests::autowrap_mode_can_be_reenabled`가 right margin 동작을 검증한다.
- `terminal_core::tests::autowrap_private_mode_has_core_evidence`가 compatibility evidence로 유지된다.
- `scripts/run-app-target-smokes.sh`의 `htop-runtime` target이 meter/status/table/function-key marker를 확인한다.
