# 알려진 호환성 Gap

이 문서는 Phase 007 이후 남아 있는 호환성 gap을 추적한다. 우선순위는 기본 shell 사용성, 데이터 손실/입력 오작동 위험, 대표 TUI 영향, 구현 범위, 테스트 가능성을 기준으로 정한다.

## Priority 1

### Mouse Reporting

상태: `partially supported`

예시:

- runtime `vim`/`less` mouse smoke evidence
- modifier-aware mouse report

중요한 이유:

- `vim`, `less`, multiplexer 및 여러 TUI는 mouse selection, scrolling, pane interaction에 mouse reporting을 사용할 수 있다.
- 현재 앱은 mouse drag를 native selection에만 사용한다.

권장 다음 작업:

- `vim` 또는 `less` mouse smoke 결과를 기록한다.
- modifier key가 포함된 mouse report가 필요한지 확인한다.

## Priority 2

### Cross-Scrollback Selection

상태: `partially supported`

중요한 이유:

- 현재 selection/copy는 viewport snapshot 기준으로 동작한다.
- Phase 014에서 scrollback과 live screen을 함께 포함하는 viewport copy는 지원했다.
- drag 중 자동 스크롤하거나 여러 scrollback page를 가로지르는 selection은 아직 모델링되어 있지 않다.

권장 다음 작업:

- drag autoscroll 정책을 정의한다.
- 여러 page를 가로지르는 selection address를 별도 모델로 확장한다.

## Priority 3

### Environment-dependent App Smoke Target

상태: `partially supported`

대상:

- `htop`: `htop 3.5.1` version smoke는 통과했다.
- `fzf`: `fzf 0.73.1` non-interactive filter smoke는 통과했다.
- `git log --oneline --graph --decorate`: 현재 repo에서 command output smoke는 통과했다.

중요한 이유:

- app 내부 interactive smoke는 로컬 설치, focus, 입력 조작에 따라 확인 가능성이 달라진다.
- `htop`, `fzf`, `git log`는 command-level smoke를 통과했지만 app 내부 TUI interaction evidence는 아직 없다.

권장 다음 작업:

- `htop` redraw, function key 또는 quit key workflow를 앱 내부에서 수행한다.
- `fzf` interactive navigation smoke를 앱 내부에서 수행한다.
- `git log` pager 진입, scroll, quit workflow를 앱 내부에서 수행한다.

## Priority 4

### Full xterm Compatibility Coverage

상태: `not supported`

중요한 이유:

- xterm compatibility는 넓은 장기 목표이며 단일 acceptance criterion으로 다루면 안 된다.

권장 다음 작업:

- 아직 확인하지 않은 modifier key variants를 앱별 smoke로 검증한다.
- OSC 52 query/readback과 세부 보안 정책은 별도 설계한다.
- modifier-aware mouse report가 필요한지 app smoke로 판단한다.
- 새로 지원하는 sequence마다 작은 parser/grid fixture를 우선 추가한다.

### vttest Runtime Coverage

상태: `not supported`

중요한 이유:

- `vttest`는 terminal emulator 호환성을 검증하는 de facto 테스트 도구다.
- 현재 local verification environment에서 `/opt/homebrew/bin/vttest`와 `vttest -V`는 확인했다.
- 실제 menu 기반 runtime 결과는 아직 수집하지 못했다.

권장 다음 작업:

- `docs/compatibility/smoke-tests.md`의 `vttest` 절차를 앱 내부에서 실행한다.
- 실패 항목을 cursor, erase, scrolling, reporting, character set, keyboard input, OSC 등 sequence family로 분해한다.
- 통과/실패 결과를 matrix evidence와 known gap으로 연결한다.

## Priority 5

### 대표 CLI/TUI Application Certification

상태: `not supported`

대상:

- `vim`
- `emacs -nw`: local verification environment의 PATH에서 `emacs`를 찾지 못했다.
- `tmux`: `tmux 3.6b` version smoke는 통과했다.
- `tmux` 안의 `vim`
- `claude` 또는 `claude-code`: `2.1.170 (Claude Code)` version smoke는 통과했다.
- `codex-cli`: `codex-cli 0.139.0` version smoke는 통과했다.

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

### Primary Device Attributes

상태: `supported`

Phase 018에서 `CSI c`, `CSI 0 c` parser action과 core response queue를 구현했다. 현재 VT100 계열 `ESC[?1;2c` 응답을 보낸다.

### Secondary Device Attributes

상태: `supported`

Phase 020에서 `CSI > c`, `CSI > 0 c` parser action과 core response queue를 구현했다. 현재 xterm 계열 secondary DA 형식 `ESC[>0;0;0c` 응답을 보낸다.

### Function and Modified Key Encoding

상태: `partially supported`

Phase 021에서 F1-F12, Shift/Option/Control modifier가 붙은 navigation/function key의 xterm-style encoding을 추가했다. 기존 Option 단독 word navigation은 shell 편집 UX를 위해 유지한다. 모든 modifier variant의 runtime app smoke는 full xterm gap으로 남긴다.

### Application Keypad Mode

상태: `supported`

Phase 022에서 `ESC =`, `ESC >` application keypad mode와 numeric keypad SS3 encoding을 구현했다. `TerminalModes`가 application keypad mode를 추적하고, app input layer가 keypad 0-9, decimal, enter, plus, minus, multiply, divide, equals를 application keypad sequence로 보낸다.

### OSC 52 Clipboard Write

상태: `partially supported`

Phase 023에서 OSC 52 clipboard write를 구현했다. core parser가 `OSC 52 ; Pc ; Pd` payload를 base64 decode해 pending clipboard write로 큐잉하고, AppKit layer가 main thread에서 pasteboard write를 수행한다. clipboard query/readback, prompt/permission policy, size policy tuning은 full xterm gap으로 남긴다.

### Legacy Mouse Encoding

상태: `supported`

Phase 024에서 SGR mouse mode가 꺼진 상태의 legacy X10-style mouse report를 구현했다. app input layer가 mouse reporting mode에서 legacy 또는 SGR encoding을 선택하고 press, release, drag, wheel report를 보낸다. modifier-aware mouse report와 runtime app smoke는 Mouse Reporting gap으로 남긴다.
