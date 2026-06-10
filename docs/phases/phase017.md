# Phase 017: Unknown App Smoke Resolution

## Purpose

Phase 017의 목적은 Phase 008 matrix에 남아 있는 `unknown` app smoke target을 해소하는 것이다.

현재 `htop`, `fzf`, `git log --oneline --graph --decorate`는 미확인 상태다. 이 phase에서는 설치 여부와 실행 결과를 기준으로 `supported`, `partially supported`, 구체 known gap 중 하나로 승격한다.

## Scope

Phase 017에서 다룰 작업:

1. `htop`, `fzf` 설치 여부 확인
2. history가 있는 git repository에서 `git log --oneline --graph --decorate` smoke 확인
3. 결과를 matrix에 반영
4. 실패 시 known gap으로 구체화
5. 필요한 core fixture 추가

## Proposed Work Breakdown

### Step 1. Environment Check

- `command -v htop`
- `command -v fzf`
- git history 존재 여부 확인

완료 기준:

- 실행 가능한 target과 환경 의존 target이 구분되어 있다.

### Step 2. Runtime Smoke

- `docs/compatibility/smoke-tests.md` 절차로 확인한다.
- pass/fail과 증상을 기록한다.

완료 기준:

- 각 target에 runtime evidence가 있다.

### Step 3. Matrix Update

- 통과하면 `supported` 또는 `partially supported`로 변경한다.
- 실패하면 `known-gaps.md`에 구체 증상과 필요한 sequence를 기록한다.

완료 기준:

- app smoke matrix에 `unknown`이 남지 않는다.

## Non-goals

- 도구 설치를 자동으로 수행하지 않는다.
- 모든 third-party TUI를 포괄하지 않는다.

## Risks

### Environment Variance

설치된 버전과 설정에 따라 결과가 달라질 수 있다.

대응:

- smoke 결과에 도구 버전과 macOS 환경을 기록한다.

## Acceptance Criteria

- `htop`, `fzf`, `git log --oneline --graph --decorate`의 matrix 상태가 `unknown`이 아니다. `done`
- 실패 항목은 known gap으로 구체화되어 있다. `done`
- 필요한 경우 replay fixture 또는 parser/state test가 추가되어 있다. `done`

## Implementation Update - 2026-06-10

Status: implementation complete for local environment resolution. Full interactive app smoke remains tracked in known gaps.

확인 결과:

- `htop`: local verification environment에 설치되어 있지 않다.
- `fzf`: `/opt/homebrew/bin/fzf`, version `0.73.1`; `printf 'alpha\nbeta\n' | fzf --filter alpha` 통과.
- `git`: `/usr/bin/git`; 현재 repository에서 `git log --oneline --graph --decorate -5 --no-color` 통과.

구현된 내용:

- app smoke matrix에서 `unknown` 상태를 제거했다.
- `htop`은 local environment unavailable로 `not supported`에 두고 known gap에 설치 의존성을 기록했다.
- `fzf`와 `git log --oneline --graph --decorate`는 command-level smoke 근거를 가진 `partially supported`로 갱신했다.
- interactive app smoke 미확인 범위를 known gap으로 구체화했다.

검증:

- `fzf --version`
- `printf 'alpha\nbeta\n' | fzf --filter alpha`
- `git log --oneline --graph --decorate -5 --no-color`
- `scripts/run-compatibility-core.sh`
- `cargo test`
