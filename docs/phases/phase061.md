# Phase 061 - htop F1 Help App Smoke

## 목표

Phase 044는 htop F10 quit workflow를 확인했다. function key coverage를 넓히기 위해 app 내부 PTY에서 F1 help 진입 후 help와 htop을 종료하는 workflow를 자동화한다.

## 범위

1. `scripts/run-app-target-smokes.sh`에 `htop-f1-help-quit` target을 추가한다.
2. 앱 내부 PTY에서 `htop`을 실행한다.
3. follow-up input으로 F1 sequence `ESC O P`와 `q q`를 보낸다.
4. htop이 종료된 뒤 shell marker가 출력되는지 확인한다.
5. matrix, smoke test, known gap 문서를 새 evidence에 맞게 갱신한다.

## 비범위

- htop mouse workflow, function key 전체 조합, setup screen 편집은 자동화하지 않는다.
- htop 사용자 설정 파일이나 process list 내용은 검증하지 않는다.

## Acceptance Criteria

- [done] `htop-f1-help-quit` app target smoke가 추가되어 있다.
- [done] 현재 local environment에서 `htop-f1-help-quit` app target smoke가 통과한다.
- [done] htop smoke 문서가 F1 help workflow evidence를 설명한다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## 결과

- `/opt/homebrew/bin/htop` 3.5.1 환경에서 app 내부 PTY로 F1 help workflow 후 shell marker `htop-f1-ok`를 확인한다.
