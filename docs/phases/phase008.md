# Phase 008: Terminal Compatibility Test Matrix

## Purpose

Phase 008의 목적은 terminal compatibility를 감으로 판단하지 않고, 반복 가능한 test matrix로 추적하는 것이다.

Phase 004에서 compatibility matrix의 초안을 만들었고, Phase 007에서 TUI 앱 호환성을 확장한다. Phase 008에서는 이 정보를 체계화해 ANSI/VT/xterm sequence, terminal behavior, 대표 앱 smoke result를 한 곳에서 관리한다.

이 phase는 단일 기능 구현보다 "앞으로 호환성을 어떻게 안전하게 늘릴 것인가"에 초점을 둔다.

## Scope

Phase 008에서 다룰 작업은 다음과 같다.

1. terminal compatibility matrix 문서화
2. sequence별 support status 정의
3. 대표 command/app smoke matrix 정의
4. fixture/golden test와 matrix 항목 연결
5. regression command runner 초안 검토
6. known gaps와 next priority 산정 방식 정의

## Matrix Categories

### ANSI/VT sequence matrix

예상 category:

- C0 control
- ESC sequence
- CSI cursor movement
- CSI erase
- CSI insert/delete
- SGR style/color
- OSC
- DEC private mode
- alternate screen
- scroll region
- device status/report

상태 값:

- `supported`
- `partially supported`
- `parsed but ignored`
- `ignored safely`
- `not supported`
- `unknown`

### Runtime behavior matrix

예상 category:

- prompt redraw
- Unicode wide character
- IME input
- paste
- bracketed paste
- selection/copy
- scrollback
- resize
- alternate screen restore

### App smoke matrix

예상 target:

- zsh prompt
- bash prompt
- `less`
- `vim`
- `top`
- `htop`
- `fzf`
- `git log --oneline --graph --decorate`

## Proposed Files

권장 문서/테스트 구조:

```text
docs/
└── compatibility/
    ├── csi.md
    ├── standards-and-tests.md
    ├── matrix.md
    ├── smoke-tests.md
    ├── known-gaps.md
    └── regression-runner.md

scripts/
└── run-compatibility-core.sh

crates/
└── terminal-core/
    └── tests/
        ├── fixtures.rs
        └── compatibility.rs
```

`matrix.md`는 사람이 읽는 상태판이다. `compatibility.rs`는 자동화 가능한 parser/grid behavior를 검증한다. `smoke-tests.md`는 macOS AppKit runtime에서 사람이 확인해야 하는 항목을 기록한다. `run-compatibility-core.sh`는 자동화 가능한 core regression 검증을 반복 실행하는 최소 runner다.

## Proposed Work Breakdown

### Step 1. Define matrix schema

- category, sequence, expected behavior, current status, evidence, notes column 정의
- status 값의 의미 문서화
- evidence에 test name, runtime command, screenshot path 등을 기록하는 방식 정의

완료 기준:

- 새 compatibility matrix 문서가 생성되어 있다.

### Step 2. Seed matrix from Phase 004-007

- Phase 004 compatibility matrix 내용을 옮긴다.
- Phase 005 IME 결과를 runtime behavior matrix에 반영한다.
- Phase 006 selection/copy 결과를 반영한다.
- Phase 007 TUI smoke 결과를 반영한다.

완료 기준:

- 기존 phase 결과가 compatibility matrix에 연결되어 있다.

### Step 3. Connect tests to matrix

- 각 fixture/golden test가 어떤 matrix row를 검증하는지 주석 또는 문서로 연결
- 테스트가 없는 supported 항목은 evidence 부족으로 표시
- unknown 항목을 다음 작업 후보로 분류

완료 기준:

- `supported` 상태에는 명확한 evidence가 있다.
- evidence 없는 항목은 `unknown` 또는 `partially supported`로 낮춘다.

### Step 4. Define smoke command protocol

- 앱 실행 방법
- log 확인 방법
- command 입력 방법
- screenshot 저장 위치
- pass/fail 기록 방식

완료 기준:

- 수동 smoke test를 반복할 수 있다.
- 실패 사례를 matrix/known-gaps로 되돌릴 수 있다.

### Step 5. Prioritize next compatibility work

우선순위 계산 기준:

- 기본 shell 사용성 영향
- 데이터 손실/입력 오작동 위험
- 대표 TUI 앱 영향
- 구현 범위
- 테스트 가능성

완료 기준:

- 다음 phase 후보가 matrix 기반으로 산출된다.

## Non-goals

Phase 008에서 하지 않을 작업:

- 모든 compatibility gap 즉시 구현
- 완전 자동 GUI 테스트 구축
- 외부 terminal emulator와 1:1 pixel-perfect 비교
- GPU renderer
- sixel/graphics protocol

## Risks

### Matrix drift

문서 matrix가 코드와 달라질 수 있다.

대응:

- matrix row에 evidence를 반드시 연결한다.
- 새 compatibility 구현 시 matrix 업데이트를 acceptance criteria에 포함한다.

### Over-documentation

matrix가 너무 크면 관리되지 않는다.

대응:

- Phase 008에서는 대표 sequence와 대표 앱부터 시작한다.
- unknown을 모두 구현하려 하지 않고 우선순위를 계산한다.

## Acceptance Criteria

Phase 008 완료 기준:

- `docs/compatibility/matrix.md`가 생성되어 있다. `done`
- `docs/compatibility/smoke-tests.md`가 생성되어 있다. `done`
- `docs/compatibility/known-gaps.md`가 생성되어 있다. `done`
- `docs/compatibility/csi.md`가 생성되어 있다. `done`
- `docs/compatibility/standards-and-tests.md`가 생성되어 있다. `done`
- `docs/compatibility/regression-runner.md`가 생성되어 있다. `done`
- `scripts/run-compatibility-core.sh`가 생성되어 있다. `done`
- Phase 004-007 결과가 matrix에 반영되어 있다. `done`
- supported 항목마다 test 또는 runtime evidence가 있다. `done`
- unknown/partially supported 항목이 known gaps로 정리되어 있다. `done`
- regression command runner 초안이 검토되어 자동화 범위와 수동 smoke 범위가 분리되어 있다. `done`
- 다음 compatibility 작업 우선순위가 matrix 기반으로 제안되어 있다. `done`
- `cargo test`가 통과한다. `done`

## Implementation Update - 2026-06-10

Status: matrix documentation and automated core evidence are implemented. GUI/runtime smoke scenarios remain manual by design.

구현된 내용:

- `docs/compatibility/matrix.md`를 추가해 ANSI/VT sequence, runtime behavior, app smoke status를 한 곳에서 추적한다.
- `docs/compatibility/smoke-tests.md`를 추가해 AppKit runtime, IME, pasteboard, TUI smoke 절차를 반복 가능하게 정리했다.
- `docs/compatibility/known-gaps.md`를 추가해 Phase 007에서 넘긴 mouse reporting, device status report, cursor style, full xterm coverage gap을 우선순위와 함께 기록했다.
- `docs/compatibility/csi.md`와 `docs/compatibility/standards-and-tests.md`를 추가해 CSI 용어, 준수 이유, 표준/테스트 판단 근거를 문서화했다.
- `docs/compatibility/regression-runner.md`와 `scripts/run-compatibility-core.sh`를 추가해 자동화 가능한 core regression 검증 범위를 정의했다.
- `crates/terminal-core/tests/compatibility.rs`를 추가해 matrix의 핵심 supported row가 `TerminalState` 공개 API로 검증되도록 했다.
- `README.md`의 현재 단계와 project layout을 Phase 008 기준으로 갱신했다.

검증:

- `cargo test`
- `scripts/run-compatibility-core.sh`
