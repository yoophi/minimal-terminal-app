# 터미널 Smoke 테스트

이 문서는 `cargo test`만으로 검증하기 어려운 동작을 반복 확인하기 위한 수동 smoke 절차다. 특히 AppKit rendering, pasteboard 연동, macOS IME, full-screen TUI 프로그램은 실제 앱 실행으로 확인해야 한다.

## 준비

앱을 빌드하고 실행한다.

```sh
scripts/run-compatibility-core.sh
scripts/run-app-smoke.sh
scripts/bundle-macos-app.sh
open 'target/debug/Minimal Terminal.app'
```

`scripts/run-app-smoke.sh`는 로컬 macOS 세션에서 app bundle을 새로 만들고 binary를 직접 실행한 뒤 짧은 시간 생존 여부를 확인한다. GUI rendering, IME, pasteboard, 실제 TUI 조작은 아래 수동 절차로 계속 확인한다.

필요하면 로그를 함께 확인한다.

```sh
/usr/bin/log stream --style compact --predicate 'process == "terminal-app"'
```

각 실행 결과는 다음 항목으로 기록한다.

- 날짜
- macOS 버전
- shell
- 명령
- pass/fail
- 메모
- 필요 시 screenshot 경로

## 기본 Shell Smoke

명령:

```sh
echo phase008-shell
printf 'plain\nsecond\n'
```

기대 결과:

- prompt가 입력을 받는다.
- 출력이 한 번씩 순서대로 표시된다.
- 앱이 계속 반응한다.

## ANSI Style Smoke

명령:

```sh
printf '\033[1;31mred\033[0m \033[4munder\033[0m \033[38;5;196midx\033[0m \033[38;2;1;2;3mrgb\033[0m\n'
```

기대 결과:

- red/bold text가 보인다.
- underline text가 보인다.
- 256 color와 truecolor foreground가 기본 text와 구분되어 보인다.

## Korean IME Smoke

명령:

```sh
echo 한글
printf 'abc한글123\n'
```

기대 결과:

- 조합 중 preedit text가 cursor 위치에 표시된다.
- 조합이 확정되기 전에는 shell로 text가 전송되지 않는다.
- 조합 확정 후 shell input line에 한글이 표시된다.
- ASCII와 한글을 섞어 입력한 뒤 Backspace/Delete를 눌러도 cursor 위치가 눈에 띄게 어긋나지 않는다.

## Selection And Copy Smoke

명령:

```sh
printf 'one\ntwo\n한글\n'
```

기대 결과:

- mouse drag로 terminal text가 highlight된다.
- `Cmd-C`가 선택한 plain text를 macOS pasteboard에 복사한다.
- Command를 누르지 않은 `Ctrl-C`는 계속 shell interrupt로 전달된다.
- 한글을 복사할 때 wide character가 중복되거나 쪼개지지 않는다.

## Scrollback Smoke

명령:

```sh
seq 1 120
```

기대 결과:

- mouse wheel로 이전 출력을 볼 수 있다.
- PageUp/PageDown으로 scrollback을 이동할 수 있다.
- 새 입력을 시작하면 live bottom view로 돌아온다.

## TUI Smoke

명령:

```sh
printf 'one\ntwo\nthree\n' | less
vim /tmp/minimal-terminal-smoke.txt
top
```

기대 결과:

- full-screen redraw가 읽을 수 있는 형태로 표시된다.
- TUI 종료 후 shell 화면이 복원된다.
- TUI 안에서 arrow key가 동작한다.
- cursor visibility 전환 후 stale cursor block이 남지 않는다.
- bracketed paste를 켜는 editor에서 paste boundary가 동작한다.
- TUI 진입/종료 후에도 color/style rendering이 유지된다.

## 선택 App Smoke

로컬에 설치되어 있거나 확인 가능한 경우 다음 명령도 실행한다.

```sh
htop
fzf
git log --oneline --graph --decorate
```

기대 결과:

- 화면이 읽을 수 있는 형태로 갱신된다.
- arrow/navigation key가 프로그램 안에서 동작한다.
- 종료 후 shell 화면이 복원된다.
- 실패하면 `known-gaps.md`의 미확인 app smoke target을 구체적인 gap으로 승격한다.

## 자동 App Command Smoke

앱 내부 PTY가 shell command를 받고 terminal buffer에 출력을 반영하는지는 다음 script로 자동 확인한다.

```sh
scripts/run-app-command-smoke.sh
```

이 script는 app bundle을 만든 뒤 smoke 전용 환경 변수로 marker command를 PTY에 쓰고, retained terminal snapshot에서 marker 출력을 찾는다.

대표 command target의 app-internal smoke는 다음 script로 실행한다.

```sh
scripts/run-app-target-smokes.sh
```

현재 자동 target:

- `mouse-sgr-report`: mouse reporting / SGR mouse mode enable 뒤 app smoke hook이 보낸 SGR left press bytes 확인
- `fzf-filter`: `printf 'alpha\nbeta\n' | fzf --filter alpha`
- `fzf-interactive`: `printf 'alpha\nbeta\n' | fzf` 실행 후 query input `b`로 interactive redraw 확인
- `fzf-preview`: `printf 'alpha\nbeta\n' | fzf --preview ...` 실행 후 query input `b`로 preview pane 갱신 확인
- `fzf-select`: `printf 'alpha\nbeta\n' | fzf` 실행 후 query input `b`와 Enter로 선택 결과 확인
- `fzf-multi-select`: `printf 'alpha\nbeta\n' | fzf -m` 실행 후 query input `b`, Tab, Enter로 multi-select 결과 확인
- `fzf-shell-ctrl-t`: zsh fzf key binding source 후 Ctrl-T widget으로 파일 경로를 command line에 삽입하고 실행 확인
- `fzf-shell-alt-c`: zsh fzf key binding source 후 Alt-C widget으로 directory를 선택하고 cwd 변경 확인
- `vim-edit-write-quit`: `vim --clean -Nu NONE -n <tempfile>` 실행 후 insert, write, quit, shell 복원 확인
- `less-basic-quit`: `printf 'one\ntwo\nthree\n' | less` 실행 후 follow-up `q`로 pager 종료 확인
- `git-log`: `git log --oneline -1 --no-color`
- `git-pager-quit`: `git log ... | less` 실행 후 follow-up `q`로 pager 종료 확인
- `git-pager-page-quit`: `git log ... | less` 실행 후 follow-up Space와 `q`로 page navigation 뒤 pager 종료 확인
- `git-pager-search-quit`: `git log ... | less` 실행 후 follow-up `/Implement`, Enter, `q`로 search 뒤 pager 종료 확인
- `git-pager-horizontal-quit`: 긴 줄 형식의 `git log ... | less -S` 실행 후 follow-up Right Arrow와 `q`로 horizontal scroll 뒤 pager 종료 확인
- `git-pager-mark-quit`: `git log ... | less` 실행 후 follow-up `m a`, `' a`, `q`로 mark 뒤 pager 종료 확인
- `tmux-version`: `tmux -V`
- `tmux-attached-session`: attached `tmux new-session` 실행 후 pane 입력과 shell 복원 확인
- `tmux-split-pane`: attached tmux session에서 vertical split pane 생성, active pane 입력, shell 복원 확인
- `tmux-vim-edit-write-quit`: attached tmux session 안에서 clean vim 실행, write, quit, shell 복원 확인
- `htop-version`: `htop --version`
- `htop-runtime`: `htop` full-screen redraw snapshot
- `htop-quit`: `htop` 실행 후 follow-up `q`로 종료 확인
- `htop-f10-quit`: `htop` 실행 후 follow-up F10 sequence로 종료 확인
- `htop-f1-help-quit`: `htop` 실행 후 follow-up F1 sequence와 `q q`로 help와 htop 종료 확인
- `claude-version`: `claude --version`
- `codex-cli-version`: `codex-cli --version`이 설치된 경우 실행
- `codex-version`: `codex-cli`가 없고 `codex`가 설치된 경우 `codex --version` 실행
- `vttest-menu`: `vttest` 시작 메뉴 snapshot

범위:

- native app binary가 실행된다.
- app 내부 login shell이 command를 받는다.
- command output이 terminal buffer snapshot에 반영된다.

범위 밖:

- GUI focus 기반 keyboard automation
- `htop`, `fzf`, `git log` 같은 전체 interactive TUI 내부 key workflow
- mouse interaction workflow

## Representative App Certification Smoke

대표 CLI/TUI 앱은 단순 실행 여부가 아니라 workflow 단위로 확인한다.

### vim

```sh
vim /tmp/minimal-terminal-smoke.txt
```

확인 항목:

- insert/normal mode 전환
- text 입력과 cursor 이동
- search highlight
- paste
- `:q!` 종료 후 main screen 복원
- 자동 smoke: `vim-edit-write-quit`은 app 내부 PTY에서 tempfile 저장 후 shell marker를 확인한다.

### emacs -nw

```sh
emacs -nw
```

확인 항목:

- PATH에 설치되어 있는지
- text 입력과 cursor 이동
- Meta/Option key
- `C-x C-c` 종료 후 main screen 복원

### tmux

```sh
tmux new -s minimal-terminal-smoke
```

확인 항목:

- session 생성
- pane split
- pane 이동
- resize 후 redraw
- detach/exit

### tmux 안의 vim

```sh
tmux new -s minimal-terminal-nested 'vim /tmp/minimal-terminal-smoke.txt'
```

자동 smoke `tmux-vim-edit-write-quit`은 app 내부 PTY에서 tmux 안의 clean vim tempfile edit/write/quit와 shell 복원을 확인한다.

확인 항목:

- nested alternate screen
- cursor mode restore
- tmux 종료 후 shell 복원

### claude / codex-cli

```sh
claude --version
codex --version
```

확인 항목:

- version command 실행
- interactive prompt 진입
- multiline paste
- resize 중 redraw
- interrupt

인증, 네트워크, 계정 상태는 terminal compatibility 결과와 분리해 기록한다.

## vttest Smoke

로컬에 `vttest`가 설치된 경우 실행한다.

```sh
vttest
```

기록 형식:

- `vttest` 설치 경로와 버전 또는 package source
- 실행 날짜와 macOS 버전
- 통과/실패 menu 번호
- 실패 증상
- 관련 sequence family: cursor, erase, scrolling, reporting, character set, keyboard input, OSC

현재 local verification environment에서는 `/opt/homebrew/bin/vttest`와 `vttest -V` 결과 `VT100 test program, version 2.7 (20251205)`를 확인했다. menu 기반 runtime 결과는 아직 수동 smoke 대상으로 남긴다.

Phase 028에서 시작 메뉴 output을 `tests/tui_replay.rs::vttest_menu_replay_renders_menu_and_queues_da_response` fixture로 고정했다. 이 fixture는 full interactive result가 아니라 vttest 초기 화면과 primary DA response path에 대한 자동 evidence다.

## 실패 처리

Smoke 테스트가 실패하면 다음 순서로 반영한다.

1. `docs/compatibility/matrix.md`의 row를 추가하거나 갱신한다.
2. `docs/compatibility/known-gaps.md`에 구체적인 gap을 기록한다.
3. terminal-core로 축소 가능한 동작이면 parser/grid fixture를 추가한다.
4. GUI 전용 동작은 자동화 경로가 생길 때까지 이 문서에 남긴다.
