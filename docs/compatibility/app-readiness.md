# 대표 CLI/TUI 애플리케이션 실행 준비도

이 문서는 Phase 009-018을 모두 완료했을 때 `vim`, `emacs`, `tmux`, `claude`, `codex-cli` 같은 대표 CLI/TUI 애플리케이션을 문제 없이 실행할 수 있는지 판단하기 위한 근거를 기록한다.

결론부터 말하면 Phase 009-018은 대표 TUI 호환성을 크게 높이지만, 해당 앱들이 문제 없이 실행된다고 보증하기에는 부족하다. Phase 018은 `vttest`와 xterm reference를 기반으로 compatibility gap을 세분화하는 단계이며, full xterm compatibility 완료를 선언하는 단계가 아니다.

## 판단 기준

대표 CLI/TUI 애플리케이션 실행 가능성을 판단할 때는 다음 범위를 나눠서 본다.

1. 기본 PTY 실행과 입력
2. ANSI/VT 출력, SGR style, alternate screen
3. cursor movement, erase, scroll region, insert/delete
4. bracketed paste와 application cursor mode
5. DSR, cursor style, mouse reporting 같은 interactive query/event 기능
6. resize, SIGWINCH, nested TUI, terminal feature detection
7. 실제 앱별 smoke workflow

Phase 009-018은 1-5번의 많은 부분을 구현하거나 검증 대상으로 삼는다. 그러나 6-7번은 실제 애플리케이션을 대상으로 한 acceptance gate가 따로 필요하다.

## 애플리케이션별 예상 상태

| 대상 | Phase 019 확인 결과 | 남는 리스크 |
| --- | --- | --- |
| `vim` | replay evidence와 app-internal edit/write/quit, mouse left press, window split key chord smoke가 있어 `partially supported`. | resize, plugin redraw |
| `less` | replay evidence, app-internal basic quit/search/follow smoke, mouse wheel-down smoke가 있어 `partially supported`. | 장시간 follow/log rotation 같은 확장 workflow |
| `emacs -nw` | local PATH에서 `emacs` 미확인, `not supported`. | 설치 후 Meta/Option key, 복잡한 key chord, mouse 확인 필요 |
| `tmux` | `tmux 3.6b` version smoke, attached session workflow smoke, split-pane workflow smoke, pane resize height 비교 smoke, nested vim edit/write/quit smoke 통과, `partially supported`. | mouse mode, copy mode, split-pane 내부 nested TUI resize/interaction |
| `htop` | `htop 3.5.1` version smoke, `Tasks:` marker 기반 runtime layout smoke, quit/F10/F1 help/F5 tree smoke가 있어 `partially supported`. | mouse, setup/editing workflow |
| `claude` / `claude-code` | `2.1.170 (Claude Code)` version smoke 통과, `partially supported`. | raw input, paste, resize, spinner/redraw, Ctrl/Alt 조합, 인증/네트워크 분리 |
| `codex` / `codex-cli` | `codex-cli 0.139.0` version smoke 통과, `partially supported`. | raw input, paste, resize, redraw, Ctrl/Alt 조합, 인증/네트워크 분리 |

## 왜 별도 보증 단계가 필요한가

이 프로젝트는 `TERM=xterm-256color`를 설정한다. 따라서 많은 CLI/TUI 도구는 xterm 계열 terminal capability를 기대한다. 하지만 `xterm-256color`를 선언하는 것과 xterm의 모든 동작을 구현하는 것은 다르다.

특히 다음 항목은 앱별로 실패 양상이 다르게 나타날 수 있다.

- DA/secondary DA, DSR 같은 terminal response sequence
- Shift/Option/Control 조합 key encoding
- remaining modifier key variants와 runtime key workflow
- mouse reporting과 native selection 충돌
- alternate screen 안에서 다시 다른 TUI를 실행하는 nested workflow
- resize 후 cursor, pane, prompt redraw
- OSC 52 query/readback과 기타 xterm extension

따라서 Phase 018 이후에는 sequence family 중심의 compatibility 확장만으로 충분하다고 보지 않고, 실제 앱별 workflow를 통과 기준으로 삼아야 한다.

## Phase 018 이후 권장 작업

Phase 019에서는 대표 CLI/TUI application certification의 첫 기준을 마련했다.

권장 smoke target:

- `vim`
- `emacs -nw`
- `tmux`
- `tmux` 안의 `vim`
- `claude` 또는 `claude-code`
- `codex` 또는 `codex-cli`
- `fzf`
- `htop`
- `git log --oneline --graph --decorate`

각 target은 단순 실행 여부가 아니라 최소 workflow를 가져야 한다. 현재 version 또는 replay evidence만 있는 target은 `partially supported`로 둔다.

예시 workflow:

1. 실행한다.
2. 일반 텍스트를 입력한다.
3. cursor 이동을 확인한다.
4. paste를 수행한다.
5. window resize 후 화면이 깨지지 않는지 확인한다.
6. 필요한 경우 mouse 또는 selection을 확인한다.
7. 정상 종료 후 main screen이 복원되는지 확인한다.

## 보증 기준

대표 CLI/TUI 앱을 "지원한다"고 표시하려면 다음 조건을 만족해야 한다.

- `docs/compatibility/matrix.md`에 앱별 row가 있다.
- `docs/compatibility/smoke-tests.md`에 반복 가능한 workflow가 있다.
- 통과 결과 또는 실패 증상이 기록되어 있다.
- 실패 증상은 `docs/compatibility/known-gaps.md`의 구체 gap으로 연결되어 있다.
- 자동화 가능한 부분은 Rust test, fixture replay, 또는 smoke script로 고정되어 있다.

이 기준을 만족하기 전에는 `vim`, `emacs`, `tmux`, `claude`, `codex`/`codex-cli`에 대해 "문제 없이 실행된다"고 표현하지 않는다. 대신 `supported`, `partially supported`, `unknown`을 앱별 workflow 근거에 맞춰 사용한다.
