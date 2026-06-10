# Phase 006: Selection, Copy, and Scrollback UX

## Purpose

Phase 006의 목적은 terminal output을 사용자가 안정적으로 선택하고 복사할 수 있게 만드는 것이다.

Phase 003에서 scrollback model과 keyboard/wheel navigation을 만들었고, Phase 004에서 style span snapshot을 추가했다. Phase 006에서는 이 기반 위에 selection/copy UX를 연결한다. 터미널은 출력 확인과 복사가 자주 필요한 도구이므로, 이 단계는 실사용성에 직접적인 영향을 준다.

## Scope

Phase 006에서 다룰 작업은 다음과 같다.

1. mouse drag 기반 text selection
2. grid coordinate 기반 selection range model
3. visible screen과 scrollback을 포함한 selection 처리
4. selected text highlight rendering
5. `Cmd-C` copy
6. selection 중 scrollback interaction 정책
7. Unicode wide character와 styled span selection 검증

## Design Direction

selection은 AppKit text view처럼 문자열 offset만 기준으로 처리하면 안 된다. 터미널은 grid/cell 좌표가 실제 표시 단위이므로 selection도 row/col 기반으로 모델링하는 것이 안전하다.

권장 모델:

```text
SelectionAnchor { row, col, source }
SelectionRange { start, end }
SelectionSource = VisibleScreen | Scrollback
```

copy 결과는 style이 아니라 plain text를 우선 제공한다. rich text copy는 후속 작업으로 분리한다.

## Proposed Work Breakdown

### Step 1. Define selection model

- terminal-app에 selection state 추가
- mouse down/drag/up 상태 분리
- view 좌표를 terminal row/col로 변환하는 helper 추가

완료 기준:

- 마우스 위치를 안정적으로 grid coordinate로 변환한다.
- selection start/end를 상태로 보관한다.

### Step 2. Extract selected text

- `TerminalSnapshot` 또는 `TerminalBuffer`에서 row/col range text 추출 API 추가
- line wrap, trailing blank trimming, newline 삽입 정책 정의
- wide continuation cell은 중복 복사하지 않는다.

완료 기준:

- 선택 범위가 plain text로 변환된다.
- 한글 wide character가 중복되거나 깨지지 않는다.

### Step 3. Render selection highlight

- selected cell range 배경 highlight 표시
- style-aware rendering과 selection background 충돌 방지
- cursor rendering과 selection highlight 우선순위 정의

완료 기준:

- 선택 영역이 화면에 명확히 표시된다.
- ANSI color text 위에서도 selection이 보인다.

### Step 4. Copy to pasteboard

- `Cmd-C` 처리 추가
- selection이 있을 때 selected text를 pasteboard에 저장
- selection이 없을 때는 shell interrupt인 Ctrl-C와 혼동하지 않도록 Command modifier만 앱 shortcut으로 처리

완료 기준:

- `Cmd-C`로 선택된 text가 macOS pasteboard에 들어간다.
- 기존 Ctrl-C PTY interrupt 동작은 유지된다.

### Step 5. Scrollback UX integration

- scrollback view에서 selection 가능하게 할지 정책 결정
- selection 중 wheel scroll 동작 정의
- scrollback offset이 바뀌어도 selection range가 가능한 한 일관되게 유지되도록 처리

완료 기준:

- visible screen selection과 scrollback selection의 동작이 문서화되어 있다.
- 최소한 현재 보이는 scrollback 화면에서 copy가 가능하다.

## Non-goals

Phase 006에서 하지 않을 작업:

- rich text/HTML copy
- URL detection/open
- search UI
- multiple selection
- rectangular/block selection
- TUI 앱 전체 호환성

## Risks

### Coordinate mismatch

font metrics, padding, line height, cell width가 selection 좌표에 직접 영향을 준다.

대응:

- renderer와 selection이 같은 `PADDING_X`, `PADDING_Y`, `CELL_WIDTH`, `LINE_HEIGHT` 기준을 사용한다.

### Scrollback complexity

visible screen과 scrollback을 하나의 selection model로 섞으면 복잡도가 커질 수 있다.

대응:

- Phase 006에서는 현재 보이는 snapshot 기준 selection을 우선 구현한다.
- 전체 scrollback spanning selection은 후속 확장으로 남긴다.

## Acceptance Criteria

Phase 006 완료 기준:

- mouse drag로 terminal text를 선택할 수 있다.
- 선택 영역이 highlight로 표시된다.
- `Cmd-C`로 선택 text를 pasteboard에 복사할 수 있다.
- Ctrl-C shell interrupt 동작이 유지된다.
- 한글 wide character가 selection/copy에서 깨지지 않는다.
- ANSI styled text를 선택해도 plain text copy가 정상 동작한다.
- `cargo test`가 통과한다.
- 실제 앱에서 selection/copy smoke scenario가 통과한다.

## Implementation Update - 2026-06-10

Status: implementation complete. 반복 검증은 Phase 008 compatibility matrix에 편입한다.

구현된 내용:

- `crates/terminal-app/src/selection.rs`에 grid coordinate 기반 selection state를 추가했다.
- `mouseDown:`, `mouseDragged:`, `mouseUp:`에서 view 좌표를 terminal row/col로 변환해 selection range를 갱신한다.
- 현재 보이는 `TerminalSnapshot` 기준으로 selected plain text를 추출한다.
- trailing blank를 정리하고, 여러 줄 선택 시 newline을 삽입한다.
- 한글 wide character selection/copy에서 continuation cell을 중복 복사하지 않도록 처리했다.
- 선택 영역 highlight를 text rendering보다 먼저 그려 ANSI styled text 위에서도 선택 상태가 보이게 했다.
- `Cmd-C`는 selection이 있을 때 macOS pasteboard에 plain text를 저장한다.
- `Ctrl-C`는 command modifier가 없으면 기존 PTY interrupt 입력 경로를 유지한다.

관련 커밋:

- `b0bbc76 Add terminal selection and copy`

검증:

- `cargo test`
- selection range normalization 테스트
- multi-line selected text 추출 테스트
- wide character copy 회귀 테스트
- 앱 번들 빌드와 런타임 시작 확인

반복 확인할 수동 smoke scenario:

```sh
printf 'one\ntwo\n한글\n'
```

확인 포인트:

- 마우스 drag로 출력 텍스트가 highlight된다.
- `Cmd-C` 후 다른 앱 또는 shell에 paste하면 선택한 plain text가 들어간다.
- `Ctrl-C`는 shell interrupt로 동작하고 copy shortcut과 충돌하지 않는다.
- styled/color text를 선택해도 plain text로 복사된다.
