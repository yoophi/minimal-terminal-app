# 터미널 Smoke 테스트

이 문서는 `cargo test`만으로 검증하기 어려운 동작을 반복 확인하기 위한 수동 smoke 절차다. 특히 AppKit rendering, pasteboard 연동, macOS IME, full-screen TUI 프로그램은 실제 앱 실행으로 확인해야 한다.

## 준비

앱을 빌드하고 실행한다.

```sh
cargo test
scripts/bundle-macos-app.sh
open 'target/debug/Minimal Terminal.app'
```

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

## 실패 처리

Smoke 테스트가 실패하면 다음 순서로 반영한다.

1. `docs/compatibility/matrix.md`의 row를 추가하거나 갱신한다.
2. `docs/compatibility/known-gaps.md`에 구체적인 gap을 기록한다.
3. terminal-core로 축소 가능한 동작이면 parser/grid fixture를 추가한다.
4. GUI 전용 동작은 자동화 경로가 생길 때까지 이 문서에 남긴다.

