# Phase 140 - Less Follow Smoke Timing Stabilization

## 목적

Phase 139 검증 중 기존 `less-follow` target이 append marker를 snapshot 전에 관측하지 못하는 timing failure를 보였다. 이 phase는 `less +F` follow mode evidence가 앱 시작 지연에 덜 민감하도록 안정화한다.

## 범위

- `less-follow` target의 background append를 더 이르게 실행한다.
- `less-follow` snapshot delay를 늘려 appended marker가 화면에 반영될 시간을 확보한다.

## 완료 기준

- [done] `less-follow` app target smoke가 `less-follow-marker`를 안정적으로 관측한다.
- [done] `scripts/run-app-target-smokes.sh` 전체 suite가 통과한다.

## 비범위

- `less +F` 장시간 follow, log rotation, 파일 삭제 후 복구 workflow
