# 알려진 호환성 Gap

이 문서는 Phase 007 이후 남아 있는 호환성 gap을 추적한다. 우선순위는 기본 shell 사용성, 데이터 손실/입력 오작동 위험, 대표 TUI 영향, 구현 범위, 테스트 가능성을 기준으로 정한다.

## Priority 1

### Mouse Reporting

상태: `partially supported`

예시:

- 실제 물리 mouse input에서 시작되는 end-to-end mouse workflow evidence

중요한 이유:

- `vim`, `less`, multiplexer 및 여러 TUI는 mouse selection, scrolling, pane interaction에 mouse reporting을 사용할 수 있다.
- mouse reporting mode가 꺼져 있을 때 앱은 mouse drag를 native selection에 사용한다.
- Phase 045에서 app 내부 PTY의 mode-gated SGR mouse report readback은 자동화했다.
- Phase 124에서 synthetic `NSEvent`가 `TerminalView`의 `mouseDown:` selector를 지나 PTY로 SGR left press bytes를 보내는 workflow를 자동화했다.
- Phase 100에서 app 내부 PTY의 clean `vim` mouse left press workflow를 smoke hook 기반 SGR mouse report로 자동화했다.
- Phase 101에서 app 내부 PTY의 `less --mouse` wheel-down workflow를 smoke hook 기반 mouse report로 자동화했다.

권장 다음 작업:

- 실제 물리 mouse input 또는 OS-level event injection부터 PTY write까지 이어지는 end-to-end mouse workflow를 별도로 확인한다.

## Priority 2

### Environment-dependent App Smoke Target

상태: `partially supported`

대상:

- `htop`: `htop 3.5.1` version smoke, runtime redraw, AppKit cell width rendering fix, background color erase fix, quit, F10 quit, F1 help quit, F5 tree, F2 setup, 임시 `HTOPRC` 기반 setup save smoke는 통과했다.
- `fzf`: `fzf 0.73.1` non-interactive filter smoke는 통과했다.
- `git log --oneline --graph --decorate`: 현재 repo에서 command output smoke는 통과했다.

중요한 이유:

- app 내부 interactive smoke는 로컬 설치, focus, 입력 조작에 따라 확인 가능성이 달라진다.
- `htop`, `fzf`, `git log`는 command-level smoke를 통과했지만 app 내부 TUI interaction evidence는 아직 없다.
- Phase 032에서 app 내부 PTY command output snapshot smoke를 자동화했다.
- Phase 033에서 app 내부 `fzf --filter`와 non-pager `git log --oneline` snapshot smoke를 자동화했다.
- Phase 034에서 설치된 `tmux`, `htop`, `claude`의 app-internal version snapshot smoke를 자동화했다.
- Phase 055에서 현재 PATH의 `codex --version` fallback을 app-internal version snapshot smoke로 자동화했다.
- Phase 038에서 app 내부 `htop` full-screen redraw snapshot smoke를 자동화했다.
- Phase 102에서 `CSI d`와 deferred autowrap을 구현해 `htop` meter layout 깨짐을 수정하고 `htop-runtime` marker를 `Tasks:`로 강화했다.
- Phase 110에서 AppKit renderer의 hard-coded cell width를 실제 terminal font cell width 측정값으로 교체해 `htop` visual layout 밀림을 수정했다.
- Phase 115에서 `htop-runtime` smoke를 `Tasks:`, `Load average:`, `PID USER`, `Command`, `F10Quit` 다중 marker 검증으로 강화해 meter/status/table/function-key row가 함께 유지되는지 확인한다.
- Phase 125에서 erase/insert/delete 계열 blank cell이 현재 SGR 배경색을 보존하도록 수정해 `xterm-256color`의 background color erase 기대 동작과 `htop` meter/header background rendering을 보강했다.
- Phase 039에서 app 내부 `fzf` interactive query redraw snapshot smoke를 자동화했다.
- Phase 040에서 app 내부 `git log ... | less` pager quit workflow를 자동화했다.
- Phase 041에서 app 내부 `htop` quit workflow를 자동화했다.
- Phase 042에서 app 내부 `git log ... | less` pager page navigation 뒤 quit workflow를 자동화했다.
- Phase 043에서 app 내부 `fzf` Enter selection workflow를 자동화했다.
- Phase 044에서 app 내부 `htop` F10 function key quit workflow를 자동화했다.
- Phase 061에서 app 내부 `htop` F1 help 뒤 quit workflow를 자동화했다.
- Phase 103에서 app 내부 `htop` F5 tree toggle workflow를 자동화했다.
- Phase 111에서 app 내부 `htop` F2 Setup 화면 진입 workflow를 자동화했다.
- Phase 121에서 임시 `HTOPRC` 경로를 사용해 app 내부 `htop` Setup 진입 후 clean exit 시 htoprc가 저장되는 workflow를 자동화했다.
- Phase 046에서 app 내부 `fzf -m` multi-select workflow를 자동화했다.
- Phase 047에서 app 내부 `fzf --preview` preview pane redraw workflow를 자동화했다.
- Phase 116에서 `fzf-preview` target의 follow-up/snapshot delay를 늘려 preview pane 준비 전에 query가 shell prompt로 전달되는 smoke timing 문제를 완화했다.
- Phase 123에서 `fzf-preview` target을 follow-up query input 대신 `--query beta` 기반 preloaded query workflow로 바꿔 preview marker 검증을 안정화했다.
- Phase 062에서 app 내부 zsh `fzf` Ctrl-T shell integration workflow를 자동화했다.
- Phase 063에서 app 내부 zsh `fzf` Alt-C directory widget workflow를 자동화했다.
- Phase 064에서 app 내부 zsh `fzf` Ctrl-R history widget workflow를 자동화했다.
- Phase 048에서 app 내부 `git log ... | less` pager search 뒤 quit workflow를 자동화했다.
- Phase 049에서 app 내부 `git log ... | less -S` pager horizontal scroll 뒤 quit workflow를 자동화했다.
- Phase 050에서 app 내부 `git log ... | less` pager mark 뒤 quit workflow를 자동화했다.
- Phase 117에서 app 내부 login shell이 home directory에서 시작해도 git target들이 current repository를 `git -C`로 명시하도록 안정화했다.
- Phase 058에서 app 내부 `vim --clean -Nu NONE -n` edit/write/quit workflow를 자동화했다.
- Phase 059에서 app 내부 `tmux` 안의 clean `vim` edit/write/quit workflow를 자동화했다.
- Phase 113에서 app 내부 `tmux` split pane 안의 clean `vim` pane resize 후 edit/write/quit workflow를 자동화했다.
- Phase 117에서 app 내부 login shell이 home directory에서 시작해도 generated helper script를 absolute path로 실행하도록 `tmux-split-vim-resize` target을 안정화했다.
- Phase 120에서 긴 follow-up 입력과 snapshot delay를 함께 쓰는 target이 기본 종료 제한에 걸리지 않도록 app target smoke 기본 wait budget을 12초로 늘렸다.
- Phase 126에서 AppKit `NSWindow` content size 변경이 `drawRect`와 PTY resize 경로를 지나 shell `stty size`에 반영되는 `native-window-resize` target을 자동화했다.
- Phase 060에서 app 내부 direct `less` basic quit workflow를 자동화했다.
- Phase 105에서 app 내부 direct `less` search workflow를 자동화했다.
- Phase 106에서 app 내부 direct `less +F` follow mode append workflow를 자동화했다.
- interactive key workflow는 아직 남아 있다.

권장 다음 작업:

- `htop` mouse 또는 setup 내부 특정 설정값 변경 workflow를 앱 내부에서 수행한다.
- 사용자별 shell/plugin 설정이 포함된 `fzf` integration workflow를 앱 내부에서 수행한다.
- `git log` pager workflow는 현재 자동 smoke target에서 quit/page/search/horizontal/mark 경로를 확인한다.
- `less` direct quit/search/follow와 mouse wheel workflow는 현재 자동 smoke target에서 확인한다.

## Priority 3

### Full xterm Compatibility Coverage

상태: `not supported`

중요한 이유:

- xterm compatibility는 넓은 장기 목표이며 단일 acceptance criterion으로 다루면 안 된다.

권장 다음 작업:

- code-level로 검증된 modifier key variants 중 Up key 전체 modifier matrix, Control+F5, Control+Option+Right 외 나머지 navigation/function 조합을 앱별 runtime smoke로 검증한다.
- 기타 NRCS/locale-specific charset 같은 charset 동작을 별도 검토한다.
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

- `vim`: clean vim edit/write/quit, mouse left press, window split key chord, PTY/grid resize redraw workflow, native AppKit window resize -> PTY size smoke는 통과했다.
- `emacs -nw`: local verification environment의 PATH에서 `emacs`를 찾지 못했다.
- `tmux`: `tmux 3.6b` version smoke, attached session workflow smoke, split-pane workflow smoke, pane resize smoke, copy mode buffer smoke, mouse wheel smoke, split pane 내부 nested vim resize workflow, native AppKit window resize -> PTY size smoke는 통과했다.
- `tmux` 안의 `vim`: clean nested edit/write/quit workflow와 split pane 내부 resize 후 edit/write/quit workflow smoke는 통과했다.
- `claude` 또는 `claude-code`: `2.1.170 (Claude Code)` version smoke와 help output smoke는 통과했다.
- `codex` / `codex-cli`: `codex-cli 0.139.0` version smoke와 help output smoke는 통과했다.

중요한 이유:

- Phase 009-018을 완료해도 특정 앱이 문제 없이 실행된다고 보증할 수는 없다.
- `TERM=xterm-256color`를 선언하는 이상 앱별 terminal capability 기대치와 실제 구현이 어긋날 수 있다.
- `tmux`와 editor, agent-style CLI는 DSR/DA, key encoding, resize, alternate screen, paste, mouse reporting 같은 여러 기능을 조합해서 사용한다.
- Phase 107에서 app 내부 clean Vim의 `<C-W>s` window split key chord workflow를 자동화했다.
- Phase 114에서 app 내부 clean Vim 실행 중 PTY/grid resize 후 redraw/input workflow를 자동화했다.
- Phase 126에서 AppKit window content size 변경이 app resize path를 통해 PTY size로 반영되는지 자동화했다.

권장 다음 작업:

- Phase 019에서 앱별 smoke workflow를 정의한다.
- 통과한 workflow만 `matrix.md`에 `supported`로 표시한다.
- 실패는 앱 이름이 아니라 구체적인 sequence/input/rendering gap으로 분해한다.
- 자세한 판단 기준은 `docs/compatibility/app-readiness.md`를 따른다.

## Resolved

### Device Status Report 응답

상태: `supported`

Phase 011에서 `CSI 5 n`, `CSI 6 n` parser action, core response queue, app PTY response path를 구현했다. 현재 지원 범위는 `CSI 5n`의 `ESC[0n` 응답과 `CSI 6n`의 1-based cursor position report 응답이다.

### 8-bit C1 CSI Introducer

상태: `supported`

Phase 096에서 8-bit C1 `0x9B` CSI introducer가 `ESC [` 형태와 같은 parser/state 경로를 사용한다는 자동 evidence를 추가했다. 현재 cursor movement, SGR, DSR 대표 경로가 `0x9B` 입력에서도 처리된다. 다른 C1 control family 전체 지원은 full xterm compatibility gap으로 남긴다.

### 8-bit C1 SS2/SS3 Single Shift

상태: `supported`

Phase 097에서 8-bit C1 `0x8E` SS2와 `0x8F` SS3가 기존 `ESC N`, `ESC O` single shift 경로와 같은 G2/G3 charset mapping을 사용하도록 구현하고 자동 evidence를 추가했다. 다른 C1 control family 전체 지원은 full xterm compatibility gap으로 남긴다.

### 8-bit C1 OSC/ST

상태: `supported`

Phase 098에서 8-bit C1 `0x9D` OSC introducer와 `0x9C` ST terminator가 기존 `ESC ]` 및 `ESC \` 경로와 같은 OSC title, OSC 52 query-deny 처리를 사용하도록 구현하고 자동 evidence를 추가했다. 다른 C1 control family 전체 지원은 full xterm compatibility gap으로 남긴다.

### Native Modified Function Key Runtime Evidence

상태: `partially supported`

Phase 127에서 `native-control-f5-key` app target smoke를 추가했다. synthetic Control+F5 `NSEvent`가 `TerminalView keyDown:` path를 지나 xterm-style modified function key sequence `ESC [ 15 ; 5 ~`로 PTY에 전달되는 것을 shell raw readback marker `native-control-f5-key:1b5b31353b357e`로 확인했다. Phase 128에서 `native-shift-option-up-key` app target smoke를 추가해 synthetic Shift+Option+Up `NSEvent`가 modified navigation key sequence `ESC [ 1 ; 4 A`로 PTY에 전달되는 것을 marker `native-shift-option-up-key:1b5b313b3441`로 확인했다. Phase 129에서 `native-control-option-right-key` app target smoke를 추가해 synthetic Control+Option+Right `NSEvent`가 modified navigation key sequence `ESC [ 1 ; 7 C`로 PTY에 전달되는 것을 marker `native-control-option-right-key:1b5b313b3743`으로 확인했다. Phase 130에서 `native-up-modifier-matrix-key` app target smoke를 추가해 synthetic Up key `NSEvent` 7개가 Shift/Option/Control 전체 modifier 조합별 sequence `ESC [ 1 ; 2 A`부터 `ESC [ 1 ; 8 A`까지 PTY에 전달되는 것을 per-modifier marker로 확인했다. Shift/Option/Control 전체 조합은 code-level test가 있으며, 전체 navigation/function key runtime matrix는 Full xterm Compatibility Coverage gap으로 계속 추적한다.

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

Phase 026에서 Shift, Option, Control modifier bit를 legacy 및 SGR mouse report code에 반영했다. Option은 xterm Meta modifier bit로 인코딩한다. Runtime `vim`/`less` mouse smoke evidence는 Phase 100-101에서 추가했다. native GUI event에서 시작되는 end-to-end workflow는 Mouse Reporting gap으로 남긴다.

### Mode-gated SGR Mouse Report App Smoke

상태: `partially supported`

Phase 045에서 `scripts/run-app-target-smokes.sh`에 `mouse-sgr-report` target을 추가했다. local verification environment에서 app 내부 PTY로 mouse reporting mode와 SGR mouse mode를 켠 뒤 smoke hook이 terminal buffer mode를 확인하고 SGR left press report를 썼다. shell readback marker `mouse-sgr-report:1b5b3c303b333b324d`를 확인했다. Phase 124에서 `native-mouse-sgr-report` target을 추가해 synthetic `NSEvent`가 `TerminalView`의 `mouseDown:` selector를 지나 PTY에 SGR left press bytes를 쓰는 workflow를 확인했다. Phase 100에서 `vim-mouse-left-press` target을 추가해 clean `vim`의 `<LeftMouse>` mapping이 smoke hook의 SGR left press를 받고 marker를 출력하는 workflow를 확인했다. Phase 101에서 `less-mouse-wheel-down` target을 추가해 `less --mouse --wheel-lines=10`에서 wheel-down reports 이후 `less-mouse-line-045`가 snapshot에 나타나는 workflow를 확인했다. 실제 물리 mouse input 또는 OS-level event injection에서 시작되는 end-to-end mouse workflow는 Mouse Reporting gap으로 계속 추적한다.

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

### Shell Home Startup and Exit Notice

상태: `supported`

Phase 118에서 `scripts/run-app-target-smokes.sh`에 `shell-home`과 `shell-exit-notice` target을 추가했다. local verification environment에서 app 내부 login shell이 `$HOME`에서 시작한다는 marker `shell-home:${HOME}`와 shell 종료 뒤 terminal buffer에 표시되는 `[Shell process exited]` marker를 확인한다.

### Representative CLI Version App Smoke

상태: `supported`

Phase 034에서 `scripts/run-app-target-smokes.sh`에 `tmux -V`, `htop --version`, `claude --version`, `codex-cli --version` target을 추가했다. Phase 055에서 현재 local verification environment의 `codex --version` fallback을 추가했다. Phase 056에서 attached `tmux new-session` workflow를 추가했고, Phase 057에서 `tmux split-window` workflow를 추가했다. Phase 104에서 detached tmux split pane resize height 비교 workflow를 추가했다. Phase 108에서 tmux copy mode buffer workflow를 추가했다. Phase 109에서 tmux mouse mode wheel workflow를 추가했다. Phase 122에서 tmux pane history를 이전 출력으로 이동하는 workflow에 맞춰 `tmux-mouse-wheel` target을 `wheel-up-20` report로 안정화했다. Phase 113에서 attached tmux split pane 내부 clean vim pane resize 후 edit/write/quit workflow를 추가했다. Phase 120에서 긴 follow-up/snapshot target의 기본 wait budget을 12초로 늘렸다. Phase 112에서 `claude --help`와 `codex`/`codex-cli --help` output smoke를 추가했다. `tmux-version`, `tmux-attached-session`, `tmux-split-pane`, `tmux-pane-resize`, `tmux-copy-mode`, `tmux-mouse-wheel`, `tmux-split-vim-resize`, `htop-version`, `claude-version`, `claude-help`, `codex-version`/`codex-cli-version`, `codex-help`/`codex-cli-help`가 app 내부 PTY snapshot smoke를 통과한다. 실제 interactive workflow는 대표 CLI/TUI Application Certification gap으로 계속 추적한다.

### DEC Special Graphics Charset

상태: `partially supported`

Phase 035에서 `ESC ( 0`과 `ESC ( B` G0 charset switching을 구현했다. Phase 051에서 `ESC ) 0`, `ESC ) B`, `SO`, `SI` 기반 G1 DEC Special Graphics locking shift를 구현했다. Phase 052에서 `ESC * 0`, `ESC * B`, `ESC + 0`, `ESC + B`, `ESC n`, `ESC o` 기반 G2/G3 DEC Special Graphics locking shift를 구현했다. Phase 053에서 `ESC N`, `ESC O` 기반 G2/G3 single shift를 구현했다. Phase 097에서 8-bit C1 `0x8E` SS2와 `0x8F` SS3 기반 G2/G3 single shift를 구현했다. Phase 054에서 `ESC ( A`, `ESC ) A`, `ESC * A`, `ESC + A` 기반 British NRCS `#` -> `£` mapping을 구현했다. Phase 065에서 `ESC ~`, `ESC }`, `ESC |` 기반 right-side G-set locking shift를 구현했다. Phase 066에서 `ESC ( K`, `ESC ) K`, `ESC * K`, `ESC + K` 기반 German NRCS mapping을 구현했고, Phase 067에서 `ESC ( C`, `ESC ) C`, `ESC * C`, `ESC + C` 기반 Finnish NRCS mapping을 구현했다. Phase 068에서 `ESC ( R`, `ESC ( f`, `ESC ) R`, `ESC ) f`, `ESC * R`, `ESC * f`, `ESC + R`, `ESC + f` 기반 French NRCS mapping을 구현했다. Phase 069에서 `ESC ( Y`, `ESC ) Y`, `ESC * Y`, `ESC + Y` 기반 Italian NRCS mapping을 구현했다. Phase 070에서 `ESC ( Z`, `ESC ) Z`, `ESC * Z`, `ESC + Z` 기반 Spanish NRCS mapping을 구현했다. Phase 071에서 `ESC ( 4`, `ESC ) 4`, `ESC * 4`, `ESC + 4` 기반 Dutch NRCS mapping을 구현했다. Phase 072에서 `ESC ( H`, `ESC ( 7`, `ESC ) H`, `ESC ) 7`, `ESC * H`, `ESC * 7`, `ESC + H`, `ESC + 7` 기반 Swedish NRCS mapping을 구현했다. Phase 073에서 ``ESC ( ` ``, `ESC ( E`, `ESC ( 6`, ``ESC ) ` ``, `ESC ) E`, `ESC ) 6`, ``ESC * ` ``, `ESC * E`, `ESC * 6`, ``ESC + ` ``, `ESC + E`, `ESC + 6` 기반 Norwegian/Danish NRCS mapping을 구현했다. Phase 074에서 `ESC ( Q`, `ESC ( 9`, `ESC ) Q`, `ESC ) 9`, `ESC * Q`, `ESC * 9`, `ESC + Q`, `ESC + 9` 기반 French Canadian NRCS mapping을 구현했다. Phase 075에서 `ESC ( =`, `ESC ) =`, `ESC * =`, `ESC + =` 기반 Swiss NRCS mapping을 구현했다. Phase 076에서 `ESC ( % 6`, `ESC ) % 6`, `ESC * % 6`, `ESC + % 6` 기반 Portuguese NRCS mapping을 구현했다. Phase 077에서 `ESC ( " >`, `ESC ) " >`, `ESC * " >`, `ESC + " >` 기반 Greek NRCS mapping을 구현했다. Phase 078에서 `ESC ( % =`, `ESC ) % =`, `ESC * % =`, `ESC + % =` 기반 Hebrew NRCS mapping을 구현했다. Phase 079에서 `ESC ( % 2`, `ESC ) % 2`, `ESC * % 2`, `ESC + % 2` 기반 Turkish NRCS mapping을 구현했다. Phase 080에서 `ESC ( & 5`, `ESC ) & 5`, `ESC * & 5`, `ESC + & 5` 기반 Russian NRCS mapping을 구현했다. Phase 081에서 `ESC ( % 3`, `ESC ) % 3`, `ESC * % 3`, `ESC + % 3` 기반 SCS NRCS mapping을 구현했다. Phase 082에서 `ESC ( J`, `ESC ) J`, `ESC * J`, `ESC + J` 기반 JIS Roman mapping을 구현했다. Phase 083에서 `ESC ( I`, `ESC ) I`, `ESC * I`, `ESC + I` 기반 JIS Katakana mapping을 구현했다. Phase 084에서 `ESC ( % 5`, `ESC ) % 5`, `ESC * % 5`, `ESC + % 5` 기반 DEC Supplemental Graphics mapping을 구현했다. Phase 099에서 `ESC ( <`, `ESC ) <`, `ESC * <`, `ESC + <` DEC Supplemental UPSS alias를 같은 mapping으로 처리했다. Phase 085에서 `ESC ( >`, `ESC ) >`, `ESC * >`, `ESC + >` 기반 DEC Technical mapping을 구현했다. Phase 086에서 `ESC ( & 4`, `ESC ) & 4`, `ESC * & 4`, `ESC + & 4` 기반 DEC Cyrillic mapping을 구현했다. Phase 087에서 `ESC ( " ?`, `ESC ) " ?`, `ESC * " ?`, `ESC + " ?` 기반 DEC Greek Supplemental mapping을 구현했다. Phase 088에서 `ESC ( " 4`, `ESC ) " 4`, `ESC * " 4`, `ESC + " 4` 기반 DEC Hebrew Supplemental mapping을 구현했다. Phase 089에서 `ESC ( % 0`, `ESC ) % 0`, `ESC * % 0`, `ESC + % 0` 기반 DEC Turkish Supplemental mapping을 구현했다. Phase 090에서 `ESC - A`, `ESC . A`, `ESC / A` 기반 ISO Latin-1 Supplemental 96-character GR mapping을 구현했다. Phase 091에서 `ESC - B`, `ESC . B`, `ESC / B` 기반 ISO Latin-2 Supplemental 96-character GR mapping을 구현했다. Phase 092에서 `ESC - F`, `ESC . F`, `ESC / F` 기반 ISO Greek Supplemental 96-character GR mapping을 구현했다. Phase 093에서 `ESC - H`, `ESC . H`, `ESC / H` 기반 ISO Hebrew Supplemental 96-character GR mapping을 구현했다. Phase 094에서 `ESC - L`, `ESC . L`, `ESC / L` 기반 ISO Latin-Cyrillic Supplemental 96-character GR mapping을 구현했다. Phase 095에서 `ESC - M`, `ESC . M`, `ESC / M` 기반 ISO Latin-5 Supplemental 96-character GR mapping을 구현했다. 대표 DEC Special Graphics line drawing 문자는 Unicode box drawing 문자로 매핑한다. raw 8-bit non-UTF-8 byte stream, DEC Technical PUA glyph fidelity, undocumented 96-character supplemental designation, 기타 charset은 full xterm compatibility gap으로 남긴다.

### vttest App Runtime Menu Smoke

상태: `partially supported`

Phase 036에서 `scripts/run-app-target-smokes.sh`에 `vttest-menu` target을 추가했다. local verification environment에서 app 내부 PTY로 `vttest`를 실행하고 시작 메뉴 snapshot marker `VT100 test program`을 확인했다. 전체 interactive menu suite 결과는 vttest Runtime Coverage gap으로 계속 추적한다.

### htop App Runtime Snapshot Smoke

상태: `partially supported`

Phase 038에서 `scripts/run-app-target-smokes.sh`에 `htop-runtime` target을 추가했다. Phase 102에서 `CSI d`와 deferred autowrap을 구현해 app 내부 PTY의 `htop` meter layout 깨짐을 수정했고, full-screen redraw snapshot marker를 `Tasks:`로 강화했다. Phase 110에서 AppKit renderer의 cell width 계산을 실제 terminal font 측정 기반으로 바꿔 visual layout 밀림을 수정했다. Phase 115에서 `htop-runtime`을 `Tasks:`, `Load average:`, `PID USER`, `Command`, `F10Quit` 다중 marker 검증으로 강화했다. Phase 119에서 배경색이 있는 trailing space를 styled snapshot에 보존하고 AppKit renderer가 span background를 cell 크기로 먼저 칠하도록 수정해 meter/header 배경 기반 layout 표시를 보강했다. Phase 125에서 erase/insert/delete 계열 blank cell이 현재 SGR 배경색을 보존하도록 수정해 `xterm-256color`의 background color erase 기대 동작과 `htop` meter/header background rendering을 보강했다. Phase 103에서 F5 tree toggle snapshot을 자동화했다. Phase 111에서 F2 Setup 화면 진입 snapshot을 자동화했다. Phase 121에서 임시 `HTOPRC` 기반 setup save workflow를 자동화했다. mouse와 setup 내부 특정 설정값 변경 workflow는 Environment-dependent App Smoke Target gap으로 계속 추적한다.

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

Phase 047에서 `scripts/run-app-target-smokes.sh`에 `fzf-preview` target을 추가했다. local verification environment에서 app 내부 PTY로 `printf "alpha\nbeta\n" | fzf --preview "printf preview:{}"`를 실행하고 query input `b` 뒤 preview pane marker `preview:beta`를 확인했다. Phase 062에서 zsh key binding source 후 Ctrl-T widget으로 `phase-fzf-shell-target` 경로를 command line에 삽입하고 shell marker `fzf-shell:phase-fzf-shell-target`를 확인했다. Phase 063에서 Alt-C directory widget으로 `phase-fzf-alt-c-target` directory를 선택하고 shell marker `fzf-alt-c:phase-fzf-alt-c-target`를 확인했다. Phase 064에서 Ctrl-R history widget으로 주입한 history command를 선택하고 shell marker `fzf-history-ok`를 확인했다. 사용자별 shell/plugin integration은 Environment-dependent App Smoke Target gap으로 계속 추적한다.

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

Phase 049에서 `scripts/run-app-target-smokes.sh`에 `git-pager-horizontal-quit` target을 추가했다. local verification environment에서 app 내부 PTY로 `git log --pretty=format:"%H %s" -100 --color=never | less -S`를 실행하고 follow-up Right Arrow sequence `ESC [ C`와 `q` 입력으로 shell marker `git-pager-horizontal-ok`를 확인했다.

### git Pager Mark App Smoke

상태: `partially supported`

Phase 050에서 `scripts/run-app-target-smokes.sh`에 `git-pager-mark-quit` target을 추가했다. local verification environment에서 app 내부 PTY로 `git log --oneline --graph --decorate -100 --color=never | less`를 실행하고 follow-up `m a`, `' a`, `q` 입력으로 shell marker `git-pager-mark-ok`를 확인했다.

### htop Quit App Smoke

상태: `partially supported`

Phase 041에서 `scripts/run-app-target-smokes.sh`에 `htop-quit` target을 추가했다. local verification environment에서 app 내부 PTY로 `htop`을 실행하고 follow-up `q` 입력 뒤 shell marker `htop-quit-ok`를 확인했다.

### htop F10 Function Key App Smoke

상태: `partially supported`

Phase 044에서 `scripts/run-app-target-smokes.sh`에 `htop-f10-quit` target을 추가했다. local verification environment에서 app 내부 PTY로 `htop`을 실행하고 follow-up F10 sequence `ESC [ 21 ~` 입력 뒤 shell marker `htop-f10-ok`를 확인했다. Phase 061에서 F1 help sequence `ESC O P`와 `q q` 입력 뒤 shell marker `htop-f1-ok`를 확인했다. Phase 103에서 F5 sequence `ESC [ 15 ~` 입력 뒤 process tree marker `├─`를 확인했다. Phase 111에서 F2 sequence `ESC O Q` 입력 뒤 Setup marker `[Setup]`을 확인했다. mouse와 setup 내부 설정 변경/저장 workflow는 Environment-dependent App Smoke Target gap으로 계속 추적한다.
