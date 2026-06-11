# Phase 112 - Claude and Codex Help App Smoke

## 목표

app 내부 PTY에서 `claude`와 `codex`/`codex-cli`의 인증 없는 help output workflow를 자동 smoke로 고정한다.

## 범위

- `scripts/run-app-target-smokes.sh`에 `claude-help` target을 추가한다.
- `codex-cli`가 있으면 `codex-cli-help`, 없고 `codex`가 있으면 `codex-help` target을 실행한다.
- app 내부 PTY에서 `--help` 출력이 terminal buffer snapshot에 반영되는지 확인한다.
- compatibility matrix, smoke test 문서, app readiness, known gaps, README를 새 evidence에 맞게 갱신한다.

## 제외 범위

- 인증이 필요한 interactive prompt workflow
- 네트워크 호출
- long-running agent session, spinner/redraw, interrupt workflow
- paste와 multiline prompt workflow

## 완료 기준

- `scripts/run-app-target-smokes.sh`에서 `claude-help`가 통과한다.
- `scripts/run-app-target-smokes.sh`에서 `codex-cli-help` 또는 `codex-help`가 통과한다.
- compatibility 문서가 CLI help output evidence를 반영한다.

## 결과

상태: 구현 완료.

- local verification environment에서 app 내부 PTY로 `claude --help` 출력 marker `Usage: claude`를 확인했다.
- local verification environment에서 app 내부 PTY로 `codex --help` 출력 marker `Commands:`를 확인했다.
- 인증/네트워크가 필요한 interactive prompt workflow는 계속 gap으로 남겼다.
