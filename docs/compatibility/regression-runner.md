# Compatibility Regression Runner

이 문서는 Phase 008에서 검토한 regression command runner의 범위와 사용법을 기록한다. 자동화 가능한 terminal-core 호환성 검증은 script로 실행하고, AppKit GUI나 pasteboard, IME, TUI runtime 동작은 `smoke-tests.md`의 수동 절차로 검증한다.

전체 테스트 목표, 방법, 범위는 [test-strategy.md](test-strategy.md)를 기준으로 한다.

## Runner 범위

자동 runner가 검증하는 범위:

- terminal-core unit test
- parser/grid/state fixture
- compatibility matrix의 core evidence test

자동 runner가 검증하지 않는 범위:

- AppKit rendering의 실제 pixel 결과
- macOS pasteboard 동작
- macOS IME preedit 표시
- 실제 `less`, `vim`, `top`, `htop`, `fzf` 실행 결과
- screenshot 비교

이 경계는 의도적이다. Phase 008의 non-goal은 완전 자동 GUI 테스트 구축이므로, 자동 runner는 core regression을 빠르게 확인하는 용도로 제한한다.

## 실행 방법

```sh
scripts/run-compatibility-core.sh
```

이 script는 다음 명령을 실행한다.

```sh
cargo test -p terminal-core
cargo test -p terminal-core --test fixtures
cargo test -p terminal-core --test compatibility
```

## 실패 처리

자동 runner가 실패하면 다음 순서로 처리한다.

1. 실패한 test가 어떤 matrix row의 evidence인지 확인한다.
2. 구현이 깨진 경우 code를 수정한다.
3. matrix 상태가 과장되어 있던 경우 `matrix.md`의 status를 낮춘다.
4. 아직 지원하지 않는 behavior였음이 드러나면 `known-gaps.md`에 기록한다.

## 수동 Smoke와의 관계

자동 runner 통과는 Phase 008의 core evidence가 유지된다는 뜻이다. 앱 전체 호환성을 의미하지는 않는다. 다음 항목은 여전히 `smoke-tests.md`로 수동 확인한다.

- ANSI style이 실제 AppKit view에 보이는지
- Korean IME 조합 문자열이 cursor 위치에 보이는지
- selection/copy가 macOS pasteboard와 맞물리는지
- `less`, `vim`, `top` 같은 TUI가 실제로 읽을 수 있게 동작하는지
