# Phase 060 - less Basic Quit App Smoke

## 목표

`less`는 replay fixture와 git pager workflow로 간접 검증되어 왔다. 대표 app row의 직접 evidence를 위해 app 내부 PTY에서 `less`를 실행하고 quit 후 shell 복원을 확인한다.

## 범위

1. `scripts/run-app-target-smokes.sh`에 `less-basic-quit` target을 추가한다.
2. 앱 내부 PTY에서 `printf ... | less`를 실행한다.
3. follow-up `q` 입력으로 pager를 종료한다.
4. 종료 후 shell marker가 출력되는지 확인한다.
5. matrix, smoke test, app readiness, known gap 문서를 새 evidence에 맞게 갱신한다.

## 비범위

- `less` search, horizontal scroll, mark는 git pager workflow에서 별도 evidence로 유지한다.
- mouse wheel, selection, file follow mode는 자동화하지 않는다.

## Acceptance Criteria

- [done] `less-basic-quit` app target smoke가 추가되어 있다.
- [done] 현재 local environment에서 `less-basic-quit` app target smoke가 통과한다.
- [done] `less` row가 direct app-internal evidence를 가리킨다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## 결과

- `/usr/bin/less` 668 환경에서 app 내부 PTY로 pager를 열고 `q`로 종료한 뒤 shell marker `less-basic-ok`를 확인한다.
