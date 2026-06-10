# 호환성 테스트 전략

이 문서는 Phase 008까지 구현된 기능을 어떤 목표와 범위로 자동화 테스트할지 정의한다. 핵심 방향은 자동화 가능한 core 동작은 Rust test와 runner로 고정하고, AppKit/IME/pasteboard/TUI runtime처럼 자동화가 불안정한 영역은 수동 smoke 절차와 분리해 추적하는 것이다.

## 테스트 목표

Phase 008까지의 테스트 목표는 다음과 같다.

1. `docs/compatibility/matrix.md`의 `supported` 항목이 실제 test 또는 runtime evidence와 연결되어 있는지 유지한다.
2. parser/grid/state 동작이 회귀하지 않도록 terminal-core test를 강화한다.
3. input, paste, selection처럼 AppKit 바깥으로 분리 가능한 로직은 unit test로 검증한다.
4. AppKit rendering, macOS IME, pasteboard, 실제 TUI 프로그램 동작은 반복 가능한 smoke 절차로 관리한다.
5. `unknown`, `partially supported`, `not supported` 항목이 문서에서 사라지거나 과장되지 않도록 한다.

## 테스트 범위

### 자동화 대상

자동화할 수 있는 범위:

- `terminal-core` parser 동작
- `terminal-core` grid/cursor/state 동작
- SGR style snapshot
- alternate screen 저장/복원
- scrollback snapshot
- scroll region
- insert/delete char/line
- malformed/unknown sequence safe ignore
- input encoder
- selection text extraction
- bracketed paste byte wrapping처럼 순수 함수로 분리 가능한 app logic
- compatibility matrix evidence test
- compatibility 문서와 테스트 이름의 일관성 검증

### 수동 Smoke 대상

수동 smoke로 남기는 범위:

- AppKit view에 실제 text/style이 보이는지
- cursor block이 눈에 보이는 위치에 맞는지
- macOS IME preedit text가 cursor 위치에 표시되는지
- macOS pasteboard와 `Cmd-C`/`Cmd-V`가 실제로 맞물리는지
- `less`, `vim`, `top`, `htop`, `fzf` 같은 실제 TUI 프로그램이 읽을 수 있게 동작하는지
- screenshot 기반 시각 검증

### 현재 범위 밖

현재 Phase 008 자동화 범위 밖:

- 모든 GUI 동작의 완전 자동화
- 외부 terminal emulator와 pixel-perfect 비교
- `vttest` 전체 자동 통과
- full xterm compatibility의 완전 증명
- GPU renderer 또는 graphics protocol 검증

## 테스트 방법

### 1. Core Compatibility Test

현재 자동화의 중심은 `terminal-core`다.

파일:

```text
crates/terminal-core/tests/compatibility.rs
crates/terminal-core/tests/fixtures.rs
```

검증 방식:

- byte stream을 `TerminalState::append_bytes`로 입력한다.
- `TerminalSnapshot`의 line, cursor, mode, style span을 확인한다.
- matrix의 `supported` row가 어떤 test로 검증되는지 evidence를 남긴다.

실행:

```sh
scripts/run-compatibility-core.sh
```

### 2. Matrix Coverage Test

추가하면 좋은 자동화:

```text
scripts/check-compatibility-docs.sh
```

검증할 내용:

- `matrix.md`에서 `supported`인 row가 evidence를 가지고 있는지
- `unknown`, `partially supported`, `not supported` row가 `known-gaps.md`와 연결되어 있는지
- `tests/compatibility.rs::test_name` 형태로 적힌 test name이 실제 test file에 존재하는지
- `docs/compatibility`의 주요 문서 링크가 존재하는지

이 검증은 Phase 008의 핵심인 "문서 matrix와 실제 evidence가 어긋나지 않는 상태"를 유지하기 위한 장치다.

### 3. Input/Paste Unit Test

추가하면 좋은 자동화:

- application cursor mode arrow key
- Option/Control/Command key 조합
- forward delete / delete backward
- confirmed IME text UTF-8 path
- bracketed paste wrapper

현재 `bracketed_paste_bytes`는 `terminal_view.rs` private 함수라 직접 테스트하기 어렵다. 장기적으로는 다음처럼 분리하는 것이 좋다.

```text
crates/terminal-app/src/paste.rs
```

분리 후 `paste.rs`에 unit test를 추가한다.

### 4. Selection Unit Test

현재 selection test는 기본 동작을 검증한다. 추가하면 좋은 범위:

- 빈 줄이 포함된 multi-line selection
- selection end column이 line width를 넘어가는 경우
- combining mark
- styled text snapshot에서 plain text만 추출되는지
- scrollback snapshot 기준 copy 동작

단, cross-scrollback selection은 아직 구현 범위가 아니므로 현재는 visible snapshot 기준까지만 검증한다.

### 5. App Smoke Script

추가하면 좋은 자동화:

```text
scripts/run-app-smoke.sh
```

검증할 내용:

- `scripts/bundle-macos-app.sh`가 성공하는지
- app binary가 직접 실행 가능한지
- 짧은 시간 동안 process가 생존하는지
- startup / pty spawn / first output 로그가 확인 가능한지

이 script는 macOS 로컬 smoke 용도로 둔다. CI 환경에서는 GUI 권한이나 display session 문제로 불안정할 수 있으므로 core runner와 분리한다.

### 6. TUI Replay Fixture

실제 `less`, `vim`, `top`을 완전히 자동 조작하기 전에 ANSI stream replay fixture를 먼저 만든다.

권장 구조:

```text
crates/terminal-core/tests/tui_replay.rs
crates/terminal-core/tests/fixtures/tui/less_open_close.ansi
crates/terminal-core/tests/fixtures/tui/vim_minimal.ansi
```

검증 방식:

- 실제 TUI에서 캡처한 escape stream을 fixture로 저장한다.
- `TerminalState`에 replay한다.
- 최종 snapshot, alternate screen 복원, cursor mode, style span을 확인한다.

이 방식은 AppKit이나 외부 프로그램 설치 상태에 덜 의존하면서 TUI compatibility를 넓히는 중간 단계다.

## 권장 실행 순서

1. `scripts/run-compatibility-core.sh`
2. `scripts/check-compatibility-docs.sh`
3. `cargo test`
4. `scripts/run-app-smoke.sh`
5. `docs/compatibility/smoke-tests.md`의 수동 TUI smoke

현재 `scripts/run-compatibility-core.sh`는 1번과 2번을 함께 실행한다. 4번은 다음 자동화 후보로 둔다.

## 완료 기준

Phase 008 테스트 자동화가 충분하다고 판단하려면 다음 조건을 만족해야 한다.

- `scripts/run-compatibility-core.sh`가 통과한다.
- `cargo test`가 통과한다.
- `matrix.md`의 `supported` row에는 test 또는 runtime evidence가 있다.
- `unknown`, `partially supported`, `not supported` row는 `known-gaps.md`에 연결되어 있다.
- GUI/runtime 동작은 `smoke-tests.md` 절차로 반복 가능하다.
- 새 compatibility 기능을 추가할 때 matrix, known gaps, test evidence를 함께 갱신하는 규칙이 유지된다.
