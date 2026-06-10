# Phase 103 - htop F5 Tree Function Key App Smoke

## 목표

app 내부 PTY에서 `htop`의 추가 function key workflow를 자동 smoke로 고정한다.

## 범위

- `scripts/run-app-target-smokes.sh`에 `htop-f5-tree` target을 추가한다.
- app 내부 PTY에서 `htop`을 실행한다.
- follow-up F5 sequence `ESC [ 15 ~`를 보내 tree view toggle을 수행한다.
- snapshot에서 process tree marker `├─`를 확인한다.
- compatibility matrix, smoke test 문서, known gaps, README를 새 evidence에 맞게 갱신한다.

## 제외 범위

- `htop` mouse workflow
- `htop` setup screen 편집
- 모든 function key 조합 인증

## 완료 기준

- `scripts/run-app-target-smokes.sh`에서 `htop-f5-tree`가 통과한다.
- `docs/compatibility/smoke-tests.md`가 새 target을 설명한다.
- `docs/compatibility/matrix.md`와 `docs/compatibility/known-gaps.md`가 htop 추가 function key evidence를 반영한다.

## 결과

상태: 구현 완료.

- local verification environment에서 app 내부 PTY로 `htop`을 실행하고 F5 tree toggle 뒤 snapshot marker `├─`를 확인했다.
- `htop`의 남은 대표 gap은 mouse workflow와 setup/editing workflow로 좁혔다.
