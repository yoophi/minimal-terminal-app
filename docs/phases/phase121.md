# Phase 121 - htop Setup Save App Smoke

## Purpose

`htop` setup 화면 진입 뒤 clean exit 시 설정 파일이 저장되는 workflow를 app 내부 PTY smoke로 확인한다.

## Scope

- 사용자 실제 `~/.config/htop/htoprc`를 건드리지 않도록 임시 `HTOPRC` 경로를 사용한다.
- app 내부 PTY에서 `htop`을 실행하고 F2 Setup, F10, `q` follow-up 입력을 보낸다.
- 임시 htoprc 파일이 생성되고 비어 있지 않은지 확인하는 marker를 출력한다.
- compatibility matrix, smoke-tests, known-gaps, README를 새 evidence에 맞게 갱신한다.

## Proposed Work Breakdown

1. `scripts/run-app-target-smokes.sh`에 `htop-setup-save` target을 추가한다.
2. target command에서 `mktemp -d`와 `HTOPRC=<temp>/htoprc`를 사용한다.
3. follow-up input으로 setup save/exit workflow를 수행한다.
4. htop 종료 후 htoprc 파일 존재와 non-empty 상태를 확인한다.
5. 관련 문서에 Phase 121 evidence를 연결한다.

## Acceptance Criteria

- [done] `htop-setup-save` app target smoke가 추가되어 있다.
- [done] smoke는 사용자 htop 설정 파일을 사용하지 않는다.
- [done] `htop-setup-save` marker가 app snapshot에서 확인된다.
- [done] htop setup save gap이 compatibility 문서에서 좁혀져 있다.

## Non-goals

- 특정 htop 설정 항목의 값 변경 검증
- htop setup 전체 navigation coverage
- htop mouse workflow 검증

## Result

상태: 구현 완료.

검증 marker:

- `htop-setup-save-ok`
