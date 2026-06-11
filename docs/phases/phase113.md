# Phase 113 - tmux Split Pane Nested Vim Resize App Smoke

## 목표

app 내부 PTY에서 `tmux` split pane 내부의 nested `vim`이 pane resize 이후에도 edit/write/quit workflow를 유지하는지 자동 smoke로 고정한다.

## 범위

- `scripts/run-app-target-smokes.sh`에 `tmux-split-vim-resize` target을 추가한다.
- attached tmux session에서 top shell pane과 bottom clean `vim` pane을 만든다.
- 같은 tmux socket의 background command로 nested vim pane height를 변경한다.
- app PTY follow-up input으로 split pane 안의 clean `vim`에 text를 입력하고 write/quit한다.
- shell 복원 후 tempfile 내용과 resize 결과 marker를 확인한다.
- compatibility matrix, smoke test 문서, app readiness, known gaps, README를 새 evidence에 맞게 갱신한다.

## 제외 범위

- 사용자 tmux 설정이 포함된 prefix/key binding workflow
- nested TUI 내부 mouse forwarding
- split pane 안의 plugin-heavy vim 환경
- window resize/SIGWINCH end-to-end workflow

## 완료 기준

- `scripts/run-app-target-smokes.sh`에서 `tmux-split-vim-resize`가 통과한다.
- app snapshot에 `tmux-split-vim-resize-ok:hello from split tmux vim:resize-ok` marker가 남는다.
- compatibility 문서가 split pane 내부 nested vim resize/interaction evidence를 반영한다.

## 결과

상태: 구현 완료.

- local verification environment에서 app 내부 PTY로 tmux split pane 안의 clean vim을 실행했다.
- 같은 tmux socket에서 pane height 변경을 확인한 뒤 vim edit/write/quit와 shell 복원 marker를 확인했다.
- `tmux`의 남은 대표 gap은 native GUI mouse end-to-end, window resize/SIGWINCH, 사용자 tmux 설정이 포함된 key binding workflow로 좁혔다.
