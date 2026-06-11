# Phase 111 - htop F2 Setup App Smoke

## 목표

app 내부 PTY에서 `htop`의 F2 Setup workflow를 자동 smoke로 고정한다.

## 범위

- `scripts/run-app-target-smokes.sh`에 `htop-f2-setup` target을 추가한다.
- app 내부 PTY에서 `htop`을 실행한다.
- follow-up F2 sequence `ESC O Q`를 보내 Setup 화면에 진입한다.
- snapshot에서 `[Setup]` marker를 확인한다.
- compatibility matrix, smoke test 문서, app readiness, known gaps, README를 새 evidence에 맞게 갱신한다.

## 제외 범위

- Setup 화면 내부 설정 변경/저장
- `htop` mouse workflow
- 모든 setup category navigation 인증

## 완료 기준

- `scripts/run-app-target-smokes.sh`에서 `htop-f2-setup`이 통과한다.
- app snapshot에 `[Setup]` marker가 남는다.
- compatibility 문서가 htop setup evidence를 반영한다.

## 결과

상태: 구현 완료.

- local verification environment에서 app 내부 PTY로 `htop`을 실행하고 F2 sequence 뒤 `[Setup]` 화면 marker를 확인했다.
- `htop`의 남은 대표 gap은 mouse workflow와 setup 내부 설정 변경/저장 workflow로 좁혔다.
