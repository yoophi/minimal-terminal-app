# Phase 007: TUI Compatibility Expansion

## Purpose

Phase 007의 목적은 `less`, `vim`, `top`, `htop`, `fzf` 같은 full-screen TUI 앱이 더 안정적으로 동작하도록 terminal compatibility를 확장하는 것이다.

Phase 004에서 `vte` parser와 SGR style rendering을 도입했고, alternate screen의 최소 처리를 만들었다. 그러나 TUI 앱은 cursor visibility, bracketed paste, alternate screen 세부 동작, erase/scroll region, mouse reporting 등 다양한 xterm sequence에 의존한다.

Phase 007은 "모든 TUI 앱 완전 지원"을 선언하는 단계가 아니라, 대표 TUI 앱을 기준으로 부족한 terminal behavior를 우선순위화하고 구현하는 단계다.

## Scope

Phase 007에서 다룰 작업은 다음과 같다.

1. 대표 TUI 앱 smoke scenario 정의
2. cursor visibility mode 처리
3. bracketed paste mode 처리
4. alternate screen 동작 보강
5. common xterm private mode 처리 범위 확대
6. scroll region과 screen clearing behavior 보강
7. TUI 앱 종료 후 main screen 복원 검증

## Candidate TUI Apps

우선 smoke target:

- `less`
- `vim`
- `top`
- `htop` if installed
- `fzf` if installed

각 앱의 기대 동작:

- alternate screen 진입
- 화면 전체 redraw
- cursor movement
- color/style 표시
- 종료 후 이전 shell prompt 복원
- 입력이 앱 내부 command로 전달

## Proposed Work Breakdown

### Step 1. TUI smoke scenarios

수동 smoke command 예시:

```sh
printf 'one\ntwo\nthree\n' | less
vim /tmp/minimal-terminal-smoke.txt
top
```

완료 기준:

- 각 앱별 expected behavior와 known issue를 문서화한다.

### Step 2. Cursor visibility

- `CSI ? 25 h/l` 처리
- `TerminalSnapshot`에 cursor visibility 추가
- renderer가 hidden cursor 상태를 반영

완료 기준:

- TUI 앱이 cursor를 숨긴 경우 block cursor가 표시되지 않는다.
- mode reset 후 cursor가 다시 표시된다.

### Step 3. Bracketed paste mode

- `CSI ? 2004 h/l` mode state 추가
- paste 시 mode가 켜져 있으면 `ESC[200~` / `ESC[201~` wrapper 적용
- mode가 꺼져 있으면 기존 paste behavior 유지

완료 기준:

- shell/editor가 bracketed paste mode를 사용할 때 paste boundary를 받을 수 있다.

### Step 4. Alternate screen refinement

- `?1049h` 진입 시 cursor save/restore semantics 확인
- alternate screen에서 scrollback을 main scrollback과 분리할지 정책 정리
- TUI 종료 후 main screen 복원 회귀 테스트 강화

완료 기준:

- `less`/`vim` 종료 후 shell screen이 복원된다.
- alternate screen 출력이 main scrollback을 과도하게 오염시키지 않는다.

### Step 5. xterm sequence expansion

우선 후보:

- erase in display/line edge cases
- cursor style sequence
- device status report 일부
- scroll region `CSI top;bottom r`
- insert/delete line
- insert/delete character

완료 기준:

- smoke target에서 발견된 실제 부족 sequence부터 구현한다.
- 구현 sequence는 fixture/golden test로 고정한다.

## Non-goals

Phase 007에서 하지 않을 작업:

- 모든 xterm sequence 완전 지원
- mouse reporting 전체 지원
- terminal multiplexer 수준의 session management
- GPU renderer
- sixel/graphics protocol

## Risks

### Broad compatibility scope

TUI 앱 호환성은 끝이 없는 작업이 될 수 있다.

대응:

- 대표 앱 smoke scenario 기준으로 범위를 제한한다.
- 실제로 부족한 sequence만 우선 구현한다.
- Phase 008의 compatibility matrix로 장기 추적한다.

### Shell/editor configuration variance

사용자 환경에 따라 `vim`, `less`, `fzf` 설정이 다를 수 있다.

대응:

- 기본 설치 상태의 최소 command를 기준으로 smoke test를 정의한다.
- local config 영향을 받는 항목은 문서에 표시한다.

## Acceptance Criteria

Phase 007 완료 기준:

- `less` smoke scenario가 동작한다.
- `vim` smoke scenario가 최소 수준으로 동작한다.
- TUI 종료 후 main screen이 복원된다.
- cursor visibility mode가 처리된다.
- bracketed paste mode가 처리된다.
- Phase 007에서 추가한 sequence가 fixture/golden test로 검증된다.
- `cargo test`가 통과한다.
- 알려진 미지원 항목이 Phase 008 matrix 후보로 기록된다.

## Implementation Update - 2026-06-10

Status: implementation complete for the first compatibility expansion. 장기 호환성 검증은 Phase 008 matrix에서 계속 추적한다.

구현된 내용:

- terminal-core에 `TerminalModes`를 추가했다.
- `CSI ? 25 h/l` cursor visibility mode를 처리한다.
- renderer가 hidden cursor 상태를 반영해 cursor block을 그리지 않는다.
- `CSI ? 2004 h/l` bracketed paste mode를 처리한다.
- bracketed paste mode가 켜진 상태에서 paste하면 `ESC[200~` / `ESC[201~` wrapper를 적용한다.
- `CSI ? 1 h/l` application cursor keys mode를 처리한다.
- application cursor keys mode에서 arrow key를 `ESC OA`, `ESC OB`, `ESC OC`, `ESC OD`로 보낸다.
- scroll region `CSI top;bottom r`을 처리한다.
- TUI에서 자주 쓰는 editing sequence를 추가했다.
  - `CSI @` insert blank characters
  - `CSI P` delete characters
  - `CSI X` erase characters
  - `CSI L` insert lines
  - `CSI M` delete lines
- scroll region 내부 newline/scroll 동작을 grid/state 테스트로 고정했다.

관련 커밋:

- `02ca228 Expand TUI terminal modes`
- `5368cab Support TUI editing and scroll region sequences`

검증:

- `cargo test`
- parser private mode 테스트
- TUI editing sequence parser 테스트
- cursor visibility, bracketed paste, application cursor mode state 테스트
- scroll region, insert/delete line, insert/delete/erase character state 테스트
- 앱 번들 빌드와 런타임 시작 확인

Phase 008 matrix에 넘길 반복 smoke target:

```sh
printf 'one\ntwo\nthree\n' | less
vim /tmp/minimal-terminal-smoke.txt
top
```

확인 포인트:

- alternate screen 진입과 종료 후 main screen 복원
- full-screen redraw
- cursor visibility 전환
- arrow key 입력
- bracketed paste boundary
- color/style rendering 유지
- scroll region을 사용하는 화면 갱신이 깨지지 않는지 확인

Known follow-up candidates:

- mouse reporting
- device status report 응답
- cursor style sequence
- full xterm compatibility coverage
- 자동화 가능한 compatibility fixture 확대
