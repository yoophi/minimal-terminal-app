# Phase 102 - htop Layout Rendering Fix

## 목표

`htop` runtime 화면의 CPU/memory meter와 `Tasks:` line이 깨지는 문제를 수정한다.

## 원인

- `htop`은 `CSI Ps d` vertical position absolute sequence로 행 위치를 이동한다.
- 마지막 column에 문자를 쓴 직후에는 xterm처럼 다음 printable 문자까지 autowrap을 지연해야 한다.
- 기존 구현은 `CSI d`를 무시했고, 마지막 column 출력 즉시 줄바꿈해 `CSI K`와 `CR`이 다음 줄에 잘못 적용될 수 있었다.

## 범위

- `CSI Ps d`를 parser/state에서 지원한다.
- printable right margin autowrap을 pending state로 처리한다.
- `htop-runtime` app target smoke marker를 `Tasks:`로 강화해 meter layout 깨짐을 잡는다.
- compatibility matrix, known gaps, README를 새 evidence에 맞게 갱신한다.

## 제외 범위

- `htop` mouse workflow
- `htop` setup screen 편집
- 전체 xterm autowrap mode family 인증

## 완료 기준

- `terminal-core` parser/state test가 `CSI d`와 deferred autowrap을 검증한다.
- `scripts/run-app-target-smokes.sh`에서 `htop-runtime`이 `Tasks:` marker로 통과한다.
- app snapshot에서 `Mem[...] Tasks:`와 `Load average:` line이 분리되어 표시된다.
