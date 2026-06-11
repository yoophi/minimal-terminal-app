# Phase 153 - htop Setup Tree Toggle App Smoke

## Purpose

`htop` Setup 내부에서 특정 설정값을 변경하고 저장하는 workflow를 app 내부 PTY smoke로 고정한다.

## Scope

- 사용자 실제 `~/.config/htop/htoprc`를 건드리지 않도록 임시 `HTOPRC` 경로를 사용한다.
- app 내부 PTY에서 `htop`을 실행하고 F2 Setup에 진입한다.
- application cursor key sequence로 Display options의 `Tree view` 항목으로 이동한다.
- Space로 `Tree view` 설정을 켜고 F10, `q`로 clean exit 한다.
- 종료 후 임시 htoprc에서 `tree_view=1` 또는 `.tree_view=1`을 확인한다.
- compatibility matrix, smoke-tests, known-gaps, app-readiness, README를 갱신한다.

## Proposed Work Breakdown

1. `scripts/run-app-target-smokes.sh`에 `htop-setup-toggle-tree` target을 추가한다.
2. target command에서 `mktemp -d`와 `HTOPRC=<temp>/htoprc`를 사용한다.
3. follow-up input으로 F2, application Right, application Down, Space, F10, `q`를 보낸다.
4. htop 종료 후 htoprc의 tree view 값이 켜졌는지 확인한다.
5. 관련 compatibility 문서에 Phase 153 evidence를 연결한다.

## Acceptance Criteria

- [done] `htop-setup-toggle-tree` app target smoke가 추가되어 있다.
- [done] smoke는 사용자 htop 설정 파일을 사용하지 않는다.
- [done] `htop-setup-toggle-tree` marker가 app snapshot에서 확인된다.
- [done] htop setup 내부 특정 설정값 변경 gap이 compatibility 문서에서 좁혀져 있다.

## Non-goals

- htop Setup 전체 navigation coverage
- htop 모든 설정 항목 변경 검증
- 실제 물리 keyboard event end-to-end automation

## Result

상태: 구현 완료.

검증 marker:

- `htop-setup-toggle-tree-ok`
