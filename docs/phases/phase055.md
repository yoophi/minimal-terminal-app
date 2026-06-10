# Phase 055 - Codex CLI Version Smoke Fallback

## 목표

현재 local verification environment에서는 `codex-cli` 이름의 바이너리는 없지만 `codex --version`이 `codex-cli 0.139.0`을 출력한다. Phase 034의 대표 CLI version smoke가 이 환경 차이 때문에 skip되지 않도록 `codex` fallback을 추가한다.

## 범위

1. `scripts/run-app-target-smokes.sh`에서 `codex-cli`가 없고 `codex`가 있으면 `codex --version`을 app 내부 PTY에서 실행한다.
2. snapshot marker는 기존과 같은 `codex-cli` 문자열을 사용한다.
3. compatibility matrix, smoke test 문서, known gap 문서를 현재 evidence에 맞게 갱신한다.

## 비범위

- `codex` interactive prompt, 인증, 네트워크 동작을 자동화하지 않는다.
- `codex`와 `codex-cli`의 배포/설치 차이를 해결하지 않는다.

## Acceptance Criteria

- [done] `codex-cli` 또는 `codex` 중 설치된 바이너리로 app-internal version smoke가 실행된다.
- [done] 현재 local environment에서 `codex-version` app target smoke가 통과한다.
- [done] matrix와 smoke test 문서가 `codex` fallback evidence를 설명한다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## 결과

- `scripts/run-app-target-smokes.sh`에 `codex` fallback target을 추가했다.
- 현재 local environment에서 `codex --version`은 `codex-cli 0.139.0`을 출력하며, app 내부 PTY snapshot smoke가 이 marker를 확인한다.
