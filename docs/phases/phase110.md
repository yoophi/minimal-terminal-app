# Phase 110 - htop AppKit Cell Width Rendering Fix

## 목표

`htop`의 terminal buffer snapshot은 정상인데 실제 AppKit 화면에서 meter와 table layout이 깨질 수 있는 문제를 수정한다.

## 원인

- App renderer가 cell 폭을 고정값 `8.4`로 사용했다.
- 실제 선택된 mono font의 glyph advance와 고정값이 다르면, 색상 span이 많은 `htop` meter/table 렌더링에서 x 좌표가 조금씩 밀릴 수 있다.
- `htop`은 CPU/memory meter, process table, function key bar에 style span이 많아 이 문제가 눈에 잘 보인다.

## 범위

- AppKit renderer의 grid cell 폭을 실제 terminal font의 `M` glyph 폭으로 측정한다.
- text drawing, selection highlight, cursor, mouse coordinate, window resize column 계산이 같은 cell 폭을 사용하게 한다.
- 기존 core terminal state와 parser 동작은 변경하지 않는다.

## 제외 범위

- `htop` mouse workflow
- `htop` setup/editing workflow
- font fallback별 pixel-perfect rendering 보증

## 완료 기준

- `cargo test`가 통과한다.
- app 내부 PTY에서 `htop` runtime snapshot의 `Mem[...] Tasks:`와 `Load average:` line이 정상 분리된다.
- 실제 AppKit 화면에서 htop layout이 고정 cell grid에 맞춰 표시된다.

## 결과

상태: 구현 완료.

- `terminal_view.rs`의 hard-coded cell width를 font 측정 기반 cell width로 교체했다.
- `cargo test`를 통과했다.
- local verification environment에서 app 내부 PTY로 `htop 3.5.1`을 실행하고 `Tasks:` marker와 layout snapshot을 확인했다.
