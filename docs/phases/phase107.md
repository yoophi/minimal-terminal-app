# Phase 107 - vim Window Split Key Chord App Smoke

## 목표

app 내부 PTY에서 clean `vim`의 complex key chord workflow를 자동 smoke로 고정한다.

## 범위

- `scripts/run-app-target-smokes.sh`에 `vim-window-split` target을 추가한다.
- clean `vim --clean -Nu NONE -n`을 실행한다.
- follow-up input으로 `<C-W>s` window split key chord를 보낸다.
- Vim 내부에서 `winnr('$')` 값을 marker file에 써 window count가 2인지 확인한다.
- compatibility matrix, smoke test 문서, app readiness, README를 새 evidence에 맞게 갱신한다.

## 제외 범위

- Vim plugin 환경
- window resize 후 redraw
- 더 복잡한 chord 조합과 mapping 충돌

## 완료 기준

- `scripts/run-app-target-smokes.sh`에서 `vim-window-split`이 통과한다.
- app snapshot에 `vim-split-count:2` marker가 남는다.
- compatibility 문서가 clean Vim complex key chord evidence를 반영한다.

## 결과

상태: 구현 완료.

- local verification environment에서 app 내부 PTY로 clean Vim을 실행하고 `<C-W>s` split 뒤 Vim window count가 2임을 확인했다.
- `vim`의 남은 대표 gap은 resize workflow와 plugin 환경으로 좁혔다.
