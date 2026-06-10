# Phase 019: Representative CLI/TUI Application Certification

## Purpose

Phase 019의 목적은 Phase 018 이후에도 남는 실제 애플리케이션 실행 리스크를 대표 CLI/TUI workflow 기준으로 검증하고, `vim`, `emacs`, `tmux`, `claude`, `codex-cli` 같은 도구의 지원 상태를 근거 있게 분류하는 것이다.

Phase 018은 `vttest`와 xterm reference를 통해 sequence compatibility를 확장하지만, 특정 앱이 문제 없이 동작한다고 보증하지는 않는다. Phase 019는 sequence family 검증을 실제 앱별 acceptance gate로 연결한다.

## Scope

Phase 019에서 다룰 작업:

1. 대표 CLI/TUI smoke target 선정
2. 앱별 최소 workflow 정의
3. `smoke-tests.md`와 `matrix.md` 갱신
4. 자동화 가능한 workflow를 script 또는 replay fixture로 고정
5. 실패 증상을 구체적인 sequence/input/rendering gap으로 분해
6. `known-gaps.md` 갱신

## Target Applications

우선순위 target:

1. `vim`
2. `emacs -nw`
3. `tmux`
4. `tmux` 안의 `vim`
5. `claude` 또는 `claude-code`
6. `codex-cli`
7. `fzf`
8. `htop`
9. `git log --oneline --graph --decorate`

`claude`, `claude-code`, `codex-cli`는 설치 여부와 인증 상태에 따라 smoke 가능성이 달라진다. 인증이 필요한 기능은 local-only manual smoke로 분리하고, 자동화는 실행 가능 여부와 기본 terminal interaction까지만 다룬다.

## Proposed Work Breakdown

### Step 1. Define App Workflows

각 앱별로 최소 workflow를 정의한다.

공통 workflow:

- 실행
- 일반 텍스트 입력
- cursor 이동
- paste
- window resize
- 종료
- main screen 복원 확인

앱별 추가 workflow:

- `vim`: insert/normal mode 전환, 저장하지 않고 종료, search highlight 확인
- `emacs -nw`: text insert, cursor movement, Meta key 확인, 종료
- `tmux`: session 생성, pane split, pane 이동, resize, detach/exit
- `tmux` 안의 `vim`: nested alternate screen과 cursor mode 복원 확인
- `claude`/`codex-cli`: prompt 입력, multiline paste, spinner/redraw, interrupt 확인
- `fzf`: list navigation, selection, cancel
- `htop`: redraw, function key 또는 quit key, optional mouse
- `git log`: pager 진입, scroll, quit

완료 기준:

- `docs/compatibility/smoke-tests.md`에 앱별 workflow가 있다.

### Step 2. Add Matrix Rows

`docs/compatibility/matrix.md`의 App Smoke 매트릭스를 앱별 workflow 단위로 확장한다.

완료 기준:

- 각 앱 row가 `supported`, `partially supported`, `unknown`, `not supported` 중 하나로 분류되어 있다.
- 각 row는 runtime evidence 또는 known gap에 연결된다.

### Step 3. Automate What Can Be Automated

자동화 우선순위:

1. 앱 설치 여부 확인
2. 앱 process launch smoke
3. fixture replay 가능한 ANSI stream 캡처
4. resize와 process survival 확인
5. core state로 검증 가능한 redraw sequence test

완료 기준:

- 자동화 가능한 target은 script 또는 Rust fixture test로 고정되어 있다.
- 자동화 불가능한 target은 manual smoke로 명확히 분리되어 있다.

### Step 4. Break Failures into Gaps

실패를 "vim failed" 또는 "tmux failed"로 기록하지 않는다. 다음처럼 분해한다.

- key encoding gap
- DSR/DA response gap
- mouse reporting gap
- cursor style gap
- alternate screen nesting gap
- resize/SIGWINCH gap
- OSC/xterm extension gap
- rendering or selection gap

완료 기준:

- 실패 증상은 `known-gaps.md`의 구체 항목으로 연결되어 있다.

## Non-goals

- 모든 CLI/TUI 애플리케이션을 보증하지 않는다.
- 네트워크 인증이나 외부 서비스 정상 동작을 terminal compatibility 기준으로 삼지 않는다.
- full xterm compatibility 완료를 선언하지 않는다.
- GUI pixel-perfect rendering을 요구하지 않는다.

## Risks

### Environment-dependent Results

`claude`, `codex-cli`, `htop`, `fzf`, `emacs`는 설치 여부와 로컬 설정에 따라 결과가 달라질 수 있다.

대응:

- 설치 여부를 smoke result에 기록한다.
- 미설치 target은 `unknown`으로 유지한다.
- 인증이나 계정 상태가 필요한 기능은 terminal compatibility와 분리한다.

### tmux Scope

`tmux`는 terminal capability와 nested TUI 동작을 많이 사용하므로 단일 phase에서 모든 문제를 해결하기 어렵다.

대응:

- `tmux` 자체 실행과 `tmux` 안의 `vim`을 별도 row로 나눈다.
- 실패 증상을 sequence/input/rendering gap으로 분해한다.

## Acceptance Criteria

- `docs/compatibility/app-readiness.md`의 판단 기준이 Phase 019 결과와 맞게 갱신되어 있다.
- `docs/compatibility/smoke-tests.md`에 대표 앱별 workflow가 있다.
- `docs/compatibility/matrix.md`에 대표 앱별 row와 evidence가 있다.
- 실패 target은 `docs/compatibility/known-gaps.md`에 구체 gap으로 연결되어 있다.
- 자동화 가능한 smoke는 script, Rust test, 또는 replay fixture로 고정되어 있다.
- `scripts/run-compatibility-core.sh`와 `cargo test`가 통과한다.
