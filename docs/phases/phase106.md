# Phase 106 - less Follow Mode App Smoke

## 목표

app 내부 PTY에서 `less +F` follow mode workflow를 자동 smoke로 고정한다.

## 범위

- `scripts/run-app-target-smokes.sh`에 `less-follow` target을 추가한다.
- 임시 log file을 만들고 `less +F`로 연다.
- background writer가 새 line과 marker를 append한다.
- snapshot에서 appended marker `less-follow-marker`를 확인한다.
- compatibility matrix, smoke test 문서, app readiness, known gaps, README를 새 evidence에 맞게 갱신한다.

## 제외 범위

- `less +F`에서 interrupt 후 search/scroll로 전환하는 workflow
- log rotation과 deleted file follow
- 장시간 follow 안정성

## 완료 기준

- `scripts/run-app-target-smokes.sh`에서 `less-follow`가 통과한다.
- `less +F` 실행 뒤 append된 `less-follow-marker`가 app snapshot에 나타난다.
- compatibility 문서가 direct `less` follow mode evidence를 반영한다.

## 결과

상태: 구현 완료.

- local verification environment에서 app 내부 PTY로 `less +F`를 실행하고 background append marker가 snapshot에 나타나는 것을 확인했다.
- direct `less` 대표 workflow는 quit, search, mouse wheel, follow mode까지 자동 smoke evidence를 갖는다.
