# 알려진 호환성 Gap

이 문서는 Phase 007 이후 남아 있는 호환성 gap을 추적한다. 우선순위는 기본 shell 사용성, 데이터 손실/입력 오작동 위험, 대표 TUI 영향, 구현 범위, 테스트 가능성을 기준으로 정한다.

## Priority 1

### Mouse Reporting

상태: `partially supported`

예시:

- legacy mouse encoding
- runtime `vim`/`less` mouse smoke evidence
- modifier-aware mouse report

중요한 이유:

- `vim`, `less`, multiplexer 및 여러 TUI는 mouse selection, scrolling, pane interaction에 mouse reporting을 사용할 수 있다.
- 현재 앱은 mouse drag를 native selection에만 사용한다.

권장 다음 작업:

- legacy mouse encoding이 필요한지 runtime smoke로 판단한다.
- `vim` 또는 `less` mouse smoke 결과를 기록한다.
- modifier key가 포함된 mouse report가 필요한지 확인한다.

## Priority 2

### Cross-Scrollback Selection

상태: `partially supported`

중요한 이유:

- 현재 selection/copy는 현재 보이는 snapshot 기준으로 동작한다.
- live screen에서 scrollback으로 이어지거나 여러 scrollback page를 가로지르는 selection은 모델링되어 있지 않다.

권장 다음 작업:

- visible row 좌표 대신 안정적인 buffer address space 기준 selection 좌표를 정의한다.
- 현재 visible selection 동작은 첫 fallback으로 유지한다.

## Priority 3

### 미확인 App Smoke Target

상태: `unknown`

대상:

- `htop`
- `fzf`
- `git log --oneline --graph --decorate`

중요한 이유:

- Phase 008 matrix에는 대표 app smoke target이 포함되어 있지만, 이 항목들은 아직 runtime evidence가 없다.
- `htop`과 `fzf`는 로컬 설치 여부에 따라 확인 가능성이 달라진다.
- `git log --oneline --graph --decorate`는 history가 충분한 repository에서 확인해야 의미가 있다.

권장 다음 작업:

- 로컬에 설치된 도구부터 `docs/compatibility/smoke-tests.md` 절차로 확인한다.
- 통과하면 `matrix.md`의 상태와 evidence를 갱신한다.
- 실패하면 실패 증상을 이 문서의 구체적인 gap으로 승격한다.

## Priority 4

### Full xterm Compatibility Coverage

상태: `not supported`

중요한 이유:

- xterm compatibility는 넓은 장기 목표이며 단일 acceptance criterion으로 다루면 안 된다.

권장 다음 작업:

- `docs/compatibility/matrix.md`를 sequence family별로 계속 확장한다.
- smoke test 실패에서 나온 unknown row를 우선순위에 따라 승격한다.
- 새로 지원하는 sequence마다 작은 parser/grid fixture를 우선 추가한다.

## Priority 5

### 대표 CLI/TUI Application Certification

상태: `not supported`

대상:

- `vim`
- `emacs -nw`
- `tmux`
- `tmux` 안의 `vim`
- `claude` 또는 `claude-code`
- `codex-cli`

중요한 이유:

- Phase 009-018을 완료해도 특정 앱이 문제 없이 실행된다고 보증할 수는 없다.
- `TERM=xterm-256color`를 선언하는 이상 앱별 terminal capability 기대치와 실제 구현이 어긋날 수 있다.
- `tmux`와 editor, agent-style CLI는 DSR/DA, key encoding, resize, alternate screen, paste, mouse reporting 같은 여러 기능을 조합해서 사용한다.

권장 다음 작업:

- Phase 019에서 앱별 smoke workflow를 정의한다.
- 통과한 workflow만 `matrix.md`에 `supported`로 표시한다.
- 실패는 앱 이름이 아니라 구체적인 sequence/input/rendering gap으로 분해한다.
- 자세한 판단 기준은 `docs/compatibility/app-readiness.md`를 따른다.

## Resolved

### Device Status Report 응답

상태: `supported`

Phase 011에서 `CSI 5 n`, `CSI 6 n` parser action, core response queue, app PTY response path를 구현했다. 현재 지원 범위는 `CSI 5n`의 `ESC[0n` 응답과 `CSI 6n`의 1-based cursor position report 응답이다.

### Cursor Style Sequence

상태: `supported`

Phase 012에서 `CSI Ps SP q` parser action, cursor style mode, AppKit cursor shape rendering을 구현했다. 현재 block, bar, underline shape을 구분하며 blinking/steady 차이는 같은 steady rendering으로 처리한다.
