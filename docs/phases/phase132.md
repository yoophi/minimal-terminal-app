# Phase 132: Native Control Option Navigation Family Smoke

## 목적

Phase 130-131은 Up key와 F5 key의 modifier matrix runtime evidence를 추가했다. 이 phase는 native key smoke hook을 일반 parser로 정리하고, Control+Option 조합에서 navigation key family가 모두 xterm-style modified sequence로 PTY에 전달되는지 확인한다.

## 범위

- `MINIMAL_TERMINAL_SMOKE_NATIVE_KEY` 값을 suffix 기반으로 parsing해 `control-option-page-up` 같은 이름을 지원한다.
- 기존 `control-f5`, `shift-option-up`, F5/Up matrix target 이름이 계속 동작해야 한다.
- modified PageUp/PageDown이 native scrollback handler에 소비되지 않고 terminal input encoder로 내려가도록 보정한다.
- Control+Option Up/Down/Right/Left/Home/End/PageUp/PageDown/Delete synthetic `NSEvent`를 raw readback으로 확인한다.
- `scripts/run-app-target-smokes.sh`에 `native-control-option-navigation-family-key` target을 추가한다.
- 전체 suite 검증 중 흔들리는 `tmux-copy-mode` target은 source line이 pane에 나타난 뒤 copy-mode workflow를 실행하고 marker 출력 시간을 더 확보하도록 안정화한다.
- `fzf-preview` target은 preview pane marker가 snapshot에 반영될 시간을 더 확보한다.
- `git-log` target은 login shell readiness 편차를 흡수하도록 snapshot 시간을 더 확보한다.
- compatibility matrix, smoke-tests, known-gaps, README를 갱신한다.

## 완료 기준

- [done] generic native key smoke parser가 추가되어 있다.
- [done] 기존 native key smoke 이름과 새 navigation family 이름이 unit test로 고정되어 있다.
- [done] modified PageUp/PageDown이 native scrollback handler에 소비되지 않는 unit test가 추가되어 있다.
- [done] `native-control-option-navigation-family-key` app target smoke가 추가되어 있다.
- [done] app snapshot에 9개 Control+Option navigation sequence의 per-key marker가 모두 남는다.
- [done] `tmux-copy-mode` target이 source line 준비를 기다린 뒤 copy workflow를 수행하고 marker 출력 시간을 확보한다.
- [done] `fzf-preview` target의 snapshot delay가 preview marker 확인에 충분하도록 조정되어 있다.
- [done] `git-log` target의 snapshot delay가 command output marker 확인에 충분하도록 조정되어 있다.
- [done] `cargo test`가 통과한다.
- [done] `scripts/run-app-target-smokes.sh` 전체 suite가 통과한다.
- [done] 관련 compatibility 문서가 Control+Option navigation family runtime evidence를 반영한다.

## 비범위

- 모든 modifier와 모든 navigation key의 곱집합 runtime matrix
- 모든 function key의 전체 runtime matrix
- 실제 물리 keyboard event automation
