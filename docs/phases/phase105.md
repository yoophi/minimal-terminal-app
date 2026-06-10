# Phase 105 - less Search App Smoke

## 목표

app 내부 PTY에서 direct `less` search workflow를 자동 smoke로 고정한다.

## 범위

- `scripts/run-app-target-smokes.sh`에 `less-search` target을 추가한다.
- 120줄 입력을 `less`로 연다.
- follow-up `/less-search-line-080`와 Enter를 보내 현재 화면 밖 line으로 이동한다.
- snapshot에서 `less-search-line-080` marker를 확인한다.
- compatibility matrix, smoke test 문서, app readiness, README를 새 evidence에 맞게 갱신한다.

## 제외 범위

- `less +F` follow mode
- search highlight style 세부 검증
- direct `less` mark/horizontal scroll 조합

## 완료 기준

- `scripts/run-app-target-smokes.sh`에서 `less-search`가 통과한다.
- 검색 전 첫 화면에는 없는 `less-search-line-080`이 follow-up search 뒤 snapshot에 나타난다.
- compatibility 문서가 direct `less` search evidence를 반영한다.

## 결과

상태: 구현 완료.

- local verification environment에서 app 내부 PTY로 `less`를 실행하고 `/less-search-line-080` 검색 뒤 target line이 snapshot에 나타나는 것을 확인했다.
- `less`의 남은 대표 gap은 follow mode로 좁혔다.
