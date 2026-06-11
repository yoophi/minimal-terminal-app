# Phase 117 - Repository-relative App Smoke Path Stabilization

## Purpose

app 내부 login shell이 home directory에서 시작해도 repository-relative app target smoke가 안정적으로 실행되도록 만든다.

원격 변경으로 shell 시작 위치가 home으로 바뀌면서 기존 repo-relative `git log` smoke가 `fatal: not a git repository`로 실패했고, 상대 경로 helper script를 실행하던 `tmux-split-vim-resize`도 script를 찾지 못했다.

## Scope

- `scripts/run-app-target-smokes.sh`의 git target들이 `git -C <repo>`를 사용하도록 수정한다.
- `git-log`, `git-pager-quit`, `git-pager-page-quit`, `git-pager-search-quit`, `git-pager-horizontal-quit`, `git-pager-mark-quit`를 포함한다.
- generated helper script를 실행하는 target은 absolute path를 사용한다.
- git pager workflow 자체의 coverage는 유지한다.

## Proposed Work Breakdown

1. 실패 snapshot에서 shell cwd가 home인지 확인한다.
2. smoke script 실행 시 repo path를 캡처한다.
3. 모든 git target command에 `git -C "${repo_path}"`를 적용한다.
4. `tmux-split-vim-resize` helper script 실행 경로를 absolute path로 바꾼다.
5. 전체 app target smoke를 다시 실행한다.

## Acceptance Criteria

- [done] `git-log` target이 app shell cwd에 의존하지 않는다.
- [done] git pager target들이 app shell cwd에 의존하지 않는다.
- [done] `tmux-split-vim-resize` helper script target이 app shell cwd에 의존하지 않는다.
- [done] `scripts/run-app-target-smokes.sh`가 전체 통과한다.

## Non-goals

- 사용자 git config 전체 조합 인증
- 외부 repository workflow 인증
- git pager의 모든 less key binding 인증

## Result

상태: repository-relative app smoke path stabilization 완료.

검증 대상:

- `git-log`
- `git-pager-quit`
- `git-pager-page-quit`
- `git-pager-search-quit`
- `git-pager-horizontal-quit`
- `git-pager-mark-quit`
- `tmux-split-vim-resize`
