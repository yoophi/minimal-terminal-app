# Phase 018: vttest and xterm Compatibility Expansion

## Purpose

Phase 018의 목적은 `vttest`와 xterm reference를 바탕으로 장기 terminal compatibility를 체계적으로 확장하는 것이다.

Phase 008까지는 대표 sequence와 대표 app smoke를 중심으로 matrix를 만들었다. Phase 018에서는 `vttest` 결과와 xterm control sequence 문서를 사용해 matrix를 더 세분화한다.

## Scope

Phase 018에서 다룰 작업:

1. `vttest`를 smoke target으로 추가
2. 실패 항목을 sequence family로 분해
3. xterm control sequence 문서 기준으로 matrix row 확장
4. 우선순위 높은 sequence부터 구현
5. full xterm compatibility umbrella gap 재평가

## Proposed Work Breakdown

### Step 1. Add vttest Smoke Protocol

- `vttest` 설치 여부 확인
- 실행 방법과 기록 형식을 문서화한다.

완료 기준:

- `smoke-tests.md`에 `vttest` 절차가 있다.

### Step 2. Break Down Failures

- 실패를 "vttest failed"로 뭉뚱그리지 않는다.
- cursor, erase, scrolling, reporting, character set 등 sequence family로 분류한다.

완료 기준:

- 실패 항목이 matrix row 또는 known gap으로 분해되어 있다.

### Step 3. Implement Priority Rows

우선순위 기준:

- shell/editor 실사용 영향
- 데이터 손실/입력 오작동 위험
- 구현 범위
- test 가능성

완료 기준:

- 선택한 sequence family마다 parser/state test가 있다.

## Non-goals

- 단기간에 full xterm compatibility를 완료한다고 선언하지 않는다.
- 모든 `vttest` 항목 통과를 단일 phase의 필수 완료 기준으로 삼지 않는다.
- graphics protocol은 별도 장기 범위로 둔다.

## Risks

### Scope Explosion

xterm compatibility는 매우 넓다.

대응:

- umbrella gap을 세부 matrix row로 쪼갠다.
- 한 번에 하나의 sequence family를 구현한다.

## Acceptance Criteria

- `vttest` smoke 절차가 문서화되어 있다.
- 최소 한 번의 `vttest` 결과가 matrix/known gaps에 반영되어 있다.
- full xterm compatibility gap이 세부 row로 분해되기 시작했다.
- 새로 구현한 sequence는 test evidence를 가진다.
- `scripts/run-compatibility-core.sh`와 `cargo test`가 통과한다.

## After Phase 018

Phase 018을 완료해도 `vim`, `emacs`, `tmux`, `claude`, `codex-cli` 같은 대표 CLI/TUI 애플리케이션이 문제 없이 실행된다고 보증하지 않는다.

Phase 018은 sequence family와 `vttest` 중심의 compatibility 확장 단계다. 실제 애플리케이션 지원 여부는 앱별 workflow, runtime smoke evidence, 실패 gap 분해가 필요하다.

Phase 018 이후에는 [Phase 019: Representative CLI/TUI Application Certification](phase019.md)을 진행한다. 판단 기준은 [대표 CLI/TUI 애플리케이션 실행 준비도](../compatibility/app-readiness.md)를 따른다.
