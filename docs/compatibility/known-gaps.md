# 알려진 호환성 Gap

이 문서는 Phase 007 이후 남아 있는 호환성 gap을 추적한다. 우선순위는 기본 shell 사용성, 데이터 손실/입력 오작동 위험, 대표 TUI 영향, 구현 범위, 테스트 가능성을 기준으로 정한다.

## Priority 1

### Mouse Reporting

상태: `partially supported`

예시:

- GUI synthetic event 기반 `vim`/`less` mouse smoke evidence

중요한 이유:

- `vim`, `less`, multiplexer 및 여러 TUI는 mouse selection, scrolling, pane interaction에 mouse reporting을 사용할 수 있다.
- 현재 앱은 mouse drag를 native selection에만 사용한다.
- Phase 045에서 app 내부 PTY의 mode-gated SGR mouse report readback은 자동화했다.

권장 다음 작업:

- GUI synthetic `NSEvent` 또는 수동 smoke로 `vim`/`less` mouse workflow 결과를 기록한다.

## Priority 2

### Environment-dependent App Smoke Target

상태: `partially supported`

대상:

- `htop`: `htop 3.5.1` version smoke는 통과했다.
- `fzf`: `fzf 0.73.1` non-interactive filter smoke는 통과했다.
- `git log --oneline --graph --decorate`: 현재 repo에서 command output smoke는 통과했다.

중요한 이유:

- app 내부 interactive smoke는 로컬 설치, focus, 입력 조작에 따라 확인 가능성이 달라진다.
- `htop`, `fzf`, `git log`는 command-level smoke를 통과했지만 app 내부 TUI interaction evidence는 아직 없다.
- Phase 032에서 app 내부 PTY command output snapshot smoke를 자동화했다.
- Phase 033에서 app 내부 `fzf --filter`와 non-pager `git log --oneline` snapshot smoke를 자동화했다.
- Phase 034에서 설치된 `tmux`, `htop`, `claude`의 app-internal version snapshot smoke를 자동화했다. 현재 PATH에서 `codex-cli`는 찾지 못해 skip된다.
- Phase 038에서 app 내부 `htop` full-screen redraw snapshot smoke를 자동화했다.
- Phase 039에서 app 내부 `fzf` interactive query redraw snapshot smoke를 자동화했다.
- Phase 040에서 app 내부 `git log ... | less` pager quit workflow를 자동화했다.
- Phase 041에서 app 내부 `htop` quit workflow를 자동화했다.
- Phase 042에서 app 내부 `git log ... | less` pager page navigation 뒤 quit workflow를 자동화했다.
- Phase 043에서 app 내부 `fzf` Enter selection workflow를 자동화했다.
- Phase 044에서 app 내부 `htop` F10 function key quit workflow를 자동화했다.
- Phase 046에서 app 내부 `fzf -m` multi-select workflow를 자동화했다.
- Phase 047에서 app 내부 `fzf --preview` preview pane redraw workflow를 자동화했다.
- Phase 048에서 app 내부 `git log ... | less` pager search 뒤 quit workflow를 자동화했다.
- Phase 049에서 app 내부 `git log ... | less -S` pager horizontal scroll 뒤 quit workflow를 자동화했다.
- interactive key workflow는 아직 남아 있다.

권장 다음 작업:

- `htop` mouse 또는 추가 function key workflow를 앱 내부에서 수행한다.
- `fzf` shell integration workflow를 앱 내부에서 수행한다.
- `git log` pager mark workflow를 앱 내부에서 수행한다.

## Priority 3

### Full xterm Compatibility Coverage

상태: `not supported`

중요한 이유:

- xterm compatibility는 넓은 장기 목표이며 단일 acceptance criterion으로 다루면 안 된다.

권장 다음 작업:

- code-level로 검증된 modifier key variants를 앱별 runtime smoke로 검증한다.
- G1/G2/G3 locking shift, single shift, locale-specific charset 같은 DEC Special Graphics 밖의 charset 동작을 별도 검토한다.
- 새로 지원하는 sequence마다 작은 parser/grid fixture를 우선 추가한다.

### vttest Runtime Coverage

상태: `not supported`

중요한 이유:

- `vttest`는 terminal emulator 호환성을 검증하는 de facto 테스트 도구다.
- 현재 local verification environment에서 `/opt/homebrew/bin/vttest`와 `vttest -V`는 확인했다.
- Phase 028에서 vttest 시작 메뉴 출력 replay fixture를 추가했다.
- Phase 036에서 app 내부 PTY 시작 메뉴 snapshot smoke를 추가했다.
- full interactive menu result는 아직 수집하지 못했다.

권장 다음 작업:

- `docs/compatibility/smoke-tests.md`의 `vttest` 절차를 앱 내부에서 실행한다.
- 실패 항목을 cursor, erase, scrolling, reporting, character set, keyboard input, OSC 등 sequence family로 분해한다.
- 통과/실패 결과를 matrix evidence와 known gap으로 연결한다.

## Priority 4

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

Phase 021에서 F1-F12, Shift/Option/Control modifier가 붙은 navigation/function key의 xterm-style encoding을 추가했다. 기존 Option 단독 word navigation은 shell 편집 UX를 위해 유지한다.

Phase 037에서 Shift, Option, Control, Shift+Option, Shift+Control, Option+Control, Shift+Option+Control 7개 조합을 navigation/function key unit test로 고정했다. 실제 AppKit event에서 각 조합이 같은 sequence로 들어가는지는 runtime app smoke gap으로 남긴다.

### Application Keypad Mode

상태: `supported`

Phase 022에서 `ESC =`, `ESC >` application keypad mode와 numeric keypad SS3 encoding을 구현했다. `TerminalModes`가 application keypad mode를 추적하고, app input layer가 keypad 0-9, decimal, enter, plus, minus, multiply, divide, equals를 application keypad sequence로 보낸다.

### OSC 52 Clipboard Write and Query Policy

상태: `supported`

Phase 023에서 OSC 52 clipboard write를 구현했다. core parser가 `OSC 52 ; Pc ; Pd` payload를 base64 decode해 pending clipboard write로 큐잉하고, AppKit layer가 main thread에서 pasteboard write를 수행한다.

Phase 031에서 `OSC 52 ; Pc ; ?` query를 deny-by-default 정책으로 처리하도록 명시했다. terminal core는 local clipboard를 읽지 않고 `OSC 52 ; Pc ; BEL` empty response를 큐잉한다. 실제 clipboard readback과 prompt/permission UI는 보안상 non-goal이다.

### Legacy Mouse Encoding

상태: `supported`

Phase 024에서 SGR mouse mode가 꺼진 상태의 legacy X10-style mouse report를 구현했다. app input layer가 mouse reporting mode에서 legacy 또는 SGR encoding을 선택하고 press, release, drag, wheel report를 보낸다. runtime app smoke는 Mouse Reporting gap으로 남긴다.

### Modifier-aware Mouse Report

상태: `supported`

Phase 026에서 Shift, Option, Control modifier bit를 legacy 및 SGR mouse report code에 반영했다. Option은 xterm Meta modifier bit로 인코딩한다. runtime `vim`/`less` mouse smoke evidence는 Mouse Reporting gap으로 남긴다.

### Mode-gated SGR Mouse Report App Smoke

상태: `partially supported`

Phase 045에서 `scripts/run-app-target-smokes.sh`에 `mouse-sgr-report` target을 추가했다. local verification environment에서 app 내부 PTY로 mouse reporting mode와 SGR mouse mode를 켠 뒤 smoke hook이 terminal buffer mode를 확인하고 SGR left press report를 썼다. shell readback marker `mouse-sgr-report:1b5b3c303b333b324d`를 확인했다. GUI synthetic `NSEvent`와 `vim`/`less` mouse workflow는 Mouse Reporting gap으로 계속 추적한다.

### OSC Title Update

상태: `supported`

Phase 027에서 `OSC 0 ; title BEL`과 `OSC 2 ; title BEL` window title update를 구현했다. core parser가 pending title write를 큐잉하고 AppKit layer가 main thread에서 window title에 반영한다.

### Cross-Scrollback Selection

상태: `supported`

Phase 030에서 `TerminalSnapshot`에 `viewport_start_absolute_row`를 추가하고 selection anchor/active를 absolute row 기반으로 저장하도록 변경했다. draw path는 현재 viewport와 겹치는 selection range만 투영하고, copy path는 retained scrollback과 live screen 전체 snapshot에서 absolute range를 추출한다.

### App Command Smoke Harness

상태: `supported`

Phase 032에서 smoke 전용 환경 변수 기반 harness를 추가했다. `scripts/run-app-command-smoke.sh`는 native app 내부 PTY에 marker command를 주입하고 terminal buffer snapshot에서 marker output을 확인한다. interactive TUI key workflow는 Environment-dependent App Smoke Target gap으로 계속 추적한다.

### App Target Command Smoke

상태: `supported`

Phase 033에서 `scripts/run-app-target-smokes.sh`를 추가했다. local verification environment에서 app 내부 PTY를 통해 `fzf --filter`와 non-pager `git log --oneline` snapshot smoke가 통과했다. `fzf` interactive navigation과 `git` pager workflow는 Environment-dependent App Smoke Target gap으로 계속 추적한다.

### Representative CLI Version App Smoke

상태: `supported`

Phase 034에서 `scripts/run-app-target-smokes.sh`에 `tmux -V`, `htop --version`, `claude --version`, `codex-cli --version` target을 추가했다. 현재 local verification environment에서는 `tmux-version`, `htop-version`, `claude-version`이 app 내부 PTY snapshot smoke를 통과했고, `codex-cli-version`은 PATH에서 `codex-cli`를 찾지 못해 skip된다. 실제 interactive workflow는 대표 CLI/TUI Application Certification gap으로 계속 추적한다.

### DEC Special Graphics Charset

상태: `partially supported`

Phase 035에서 `ESC ( 0`과 `ESC ( B` G0 charset switching을 구현했다. 대표 DEC Special Graphics line drawing 문자는 Unicode box drawing 문자로 매핑한다. G1/G2/G3 locking shift, single shift, locale-specific charset은 full xterm compatibility gap으로 남긴다.

### vttest App Runtime Menu Smoke

상태: `partially supported`

Phase 036에서 `scripts/run-app-target-smokes.sh`에 `vttest-menu` target을 추가했다. local verification environment에서 app 내부 PTY로 `vttest`를 실행하고 시작 메뉴 snapshot marker `VT100 test program`을 확인했다. 전체 interactive menu suite 결과는 vttest Runtime Coverage gap으로 계속 추적한다.

### htop App Runtime Snapshot Smoke

상태: `partially supported`

Phase 038에서 `scripts/run-app-target-smokes.sh`에 `htop-runtime` target을 추가했다. local verification environment에서 app 내부 PTY로 `htop`을 실행하고 full-screen redraw snapshot marker `Load average`를 확인했다. function key와 quit workflow는 Environment-dependent App Smoke Target gap으로 계속 추적한다.

### fzf Interactive App Smoke

상태: `partially supported`

Phase 039에서 smoke harness에 follow-up input을 추가하고 `scripts/run-app-target-smokes.sh`에 `fzf-interactive` target을 추가했다. local verification environment에서 app 내부 PTY로 `fzf`를 실행한 뒤 query input `b`를 보내 filtered redraw marker `beta`를 확인했다.

### fzf Enter Selection App Smoke

상태: `partially supported`

Phase 043에서 `scripts/run-app-target-smokes.sh`에 `fzf-select` target을 추가했다. local verification environment에서 app 내부 PTY로 `selected="$(printf "alpha\nbeta\n" | fzf)"; printf "fzf-select:%s\n" "$selected"`를 실행하고 query input `b`와 Enter 입력 뒤 shell marker `fzf-select:beta`를 확인했다.

### fzf Multi-select App Smoke

상태: `partially supported`

Phase 046에서 `scripts/run-app-target-smokes.sh`에 `fzf-multi-select` target을 추가했다. local verification environment에서 app 내부 PTY로 `selected="$(printf "alpha\nbeta\n" | fzf -m)"; printf "fzf-multi:%s\n" "$selected"`를 실행하고 query input `b`, Tab, Enter 입력 뒤 shell marker `fzf-multi:beta`를 확인했다.

### fzf Preview App Smoke

상태: `partially supported`

Phase 047에서 `scripts/run-app-target-smokes.sh`에 `fzf-preview` target을 추가했다. local verification environment에서 app 내부 PTY로 `printf "alpha\nbeta\n" | fzf --preview "printf preview:{}"`를 실행하고 query input `b` 뒤 preview pane marker `preview:beta`를 확인했다. shell integration workflow는 Environment-dependent App Smoke Target gap으로 계속 추적한다.

### git Pager Quit App Smoke

상태: `partially supported`

Phase 040에서 `scripts/run-app-target-smokes.sh`에 `git-pager-quit` target을 추가했다. local verification environment에서 app 내부 PTY로 `git log --oneline --graph --decorate -100 --color=never | less`를 실행하고 follow-up `q` 입력 뒤 shell marker `git-pager-quit-ok`를 확인했다.

### git Pager Page Navigation App Smoke

상태: `partially supported`

Phase 042에서 `scripts/run-app-target-smokes.sh`에 `git-pager-page-quit` target을 추가했다. local verification environment에서 app 내부 PTY로 `git log --oneline --graph --decorate -100 --color=never | less`를 실행하고 follow-up Space 입력 뒤 두 번째 follow-up `q` 입력으로 shell marker `git-pager-page-quit-ok`를 확인했다.

### git Pager Search App Smoke

상태: `partially supported`

Phase 048에서 `scripts/run-app-target-smokes.sh`에 `git-pager-search-quit` target을 추가했다. local verification environment에서 app 내부 PTY로 `git log --oneline --graph --decorate -100 --color=never | less`를 실행하고 follow-up `/Implement`, Enter, `q` 입력으로 shell marker `git-pager-search-ok`를 확인했다.

### git Pager Horizontal Scroll App Smoke

상태: `partially supported`

Phase 049에서 `scripts/run-app-target-smokes.sh`에 `git-pager-horizontal-quit` target을 추가했다. local verification environment에서 app 내부 PTY로 `git log --pretty=format:"%H %s" -100 --color=never | less -S`를 실행하고 follow-up Right Arrow sequence `ESC [ C`와 `q` 입력으로 shell marker `git-pager-horizontal-ok`를 확인했다. pager mark workflow는 Environment-dependent App Smoke Target gap으로 계속 추적한다.

### htop Quit App Smoke

상태: `partially supported`

Phase 041에서 `scripts/run-app-target-smokes.sh`에 `htop-quit` target을 추가했다. local verification environment에서 app 내부 PTY로 `htop`을 실행하고 follow-up `q` 입력 뒤 shell marker `htop-quit-ok`를 확인했다.

### htop F10 Function Key App Smoke

상태: `partially supported`

Phase 044에서 `scripts/run-app-target-smokes.sh`에 `htop-f10-quit` target을 추가했다. local verification environment에서 app 내부 PTY로 `htop`을 실행하고 follow-up F10 sequence `ESC [ 21 ~` 입력 뒤 shell marker `htop-f10-ok`를 확인했다. mouse와 추가 function key workflow는 Environment-dependent App Smoke Target gap으로 계속 추적한다.
