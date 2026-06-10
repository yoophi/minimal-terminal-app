# Phase 005: Korean IME and Text Editing Stability

## Purpose

Phase 005의 목적은 macOS native terminal app에서 한글 입력, 조합, 삭제, cursor 위치를 안정화하는 것이다.

Phase 004까지는 PTY 출력, ANSI/SGR style, xterm-256color 렌더링 기반을 만들었다. 그러나 안정성 우선 터미널로 사용하려면 일반 shell 입력이 먼저 믿을 수 있어야 한다. 특히 macOS에서 한글 입력은 단순 `keyDown:` 처리만으로 충분하지 않다. IME 조합 중인 preedit 문자열, 확정 문자열, Backspace/Delete, cursor 위치가 서로 어긋날 수 있기 때문이다.

## Scope

Phase 005에서 다룰 작업은 다음과 같다.

1. `NSTextInputClient` 기반 입력 경로 도입
2. 한글 IME composition/preedit 상태 모델 추가
3. 조합 중 문자열을 화면에 표시
4. 조합 확정 시 UTF-8 bytes를 PTY로 전달
5. 조합 중 Backspace/Delete 동작 안정화
6. 확정된 한글 wide character 삭제와 cursor 위치 회귀 테스트
7. 한글 입력과 ANSI style/rendering이 충돌하지 않도록 검증

## Why This Comes Before TUI Compatibility

TUI 앱 전체 호환성은 중요하지만 범위가 넓다. 반면 한글 입력 안정성은 기본 shell 사용성에 직접 영향을 준다.

우선순위 근거:

- 한글 입력이 불안정하면 일반 command 입력부터 신뢰하기 어렵다.
- IME composition은 AppKit view/input 구조에 영향을 주므로 나중에 붙이기보다 이 시점에 정리하는 것이 좋다.
- Phase 003의 Unicode width 처리와 Phase 004의 style span rendering 위에서 검증하기 적절하다.
- selection/copy, TUI 호환성은 안정적인 text input 위에 얹는 것이 더 안전하다.

## Proposed Work Breakdown

### Step 1. Inspect current input path

- `TerminalView.keyDown:` 흐름 확인
- `input.rs` encoder와 IME 확정 문자열 처리 확인
- 현재 `event.characters()` 기반 처리의 한계 정리

완료 기준:

- 기존 input path와 새 `NSTextInputClient` path의 책임 경계가 문서화되어 있다.

### Step 2. Add composition state

- terminal-app에 composition/preedit 상태 추가
- 조합 중 문자열, selected range, marked range를 저장
- PTY로 보내는 확정 입력과 화면에만 표시하는 조합 입력을 분리

완료 기준:

- 조합 중인 문자열이 PTY로 조기 전달되지 않는다.
- 확정된 문자열만 PTY로 전달된다.

### Step 3. Implement NSTextInputClient methods

예상 구현 대상:

- `hasMarkedText`
- `markedRange`
- `selectedRange`
- `setMarkedText:selectedRange:replacementRange:`
- `unmarkText`
- `insertText:replacementRange:`
- `validAttributesForMarkedText`
- `firstRectForCharacterRange:actualRange:`
- `characterIndexForPoint:`

완료 기준:

- macOS IME가 앱 view를 text input client로 인식한다.
- 한글 조합 중 preedit 문자열이 view state에 들어온다.
- 조합 확정 시 `insertText` 경로로 UTF-8 bytes가 PTY로 전달된다.

### Step 4. Render preedit text

- 현재 cursor 위치에 composition text overlay 표시
- terminal grid의 확정 content와 composition overlay를 분리
- cursor block과 preedit text가 겹치지 않도록 처리

완료 기준:

- 한글 조합 중인 글자가 화면에 보인다.
- 조합 확정 전에는 terminal grid content를 오염시키지 않는다.
- 조합 확정 후 shell input line에 한글이 표시된다.

### Step 5. Stabilize deletion behavior

- 조합 중 Backspace는 IME composition에 우선 위임
- 조합이 없을 때 Backspace/Delete는 기존 PTY input encoder 경로 유지
- 확정된 한글 삭제 시 grid/cursor wide character 처리가 깨지지 않는지 확인

완료 기준:

- 조합 중 Backspace가 조합 문자열을 수정한다.
- 조합이 없을 때 Backspace는 shell로 전달된다.
- 확정된 한글 삭제 후 cursor 위치가 shell 화면과 어긋나지 않는다.

### Step 6. Add tests and runtime scenarios

테스트:

- input encoder 기존 테스트 유지
- terminal-core wide character backspace 회귀 테스트 유지
- composition state 단위 테스트 추가

런타임 확인:

```sh
echo 한글
printf '가나다\n'
```

수동 IME 시나리오:

- `ㅎㅏㄴ` 조합 중 표시
- `한` 확정 후 shell input line 표시
- `한글` 입력 후 Backspace
- 한글과 ASCII 혼합 입력: `abc한글123`

## Non-goals

Phase 005에서 하지 않을 작업:

- TUI 앱 전체 호환성
- selection/copy UX 완성
- theme preference UI
- terminal tabs/splits
- 모든 IME 언어별 edge case 완전 보장

## Risks

### AppKit protocol complexity

`NSTextInputClient`는 Objective-C protocol method와 range handling이 복잡하다.

대응:

- 작은 composition state부터 만들고 view method를 연결한다.
- PTY write와 preedit rendering을 분리한다.

### Double input risk

`keyDown:`과 `insertText:`가 모두 PTY로 쓰면 같은 문자가 두 번 입력될 수 있다.

대응:

- IME 관련 text는 `NSTextInputClient` path에서 처리한다.
- keyDown path는 non-text control key와 fallback 중심으로 유지한다.

### Cursor mismatch risk

조합 overlay와 terminal cursor가 서로 다른 좌표계를 쓰면 표시가 어긋날 수 있다.

대응:

- terminal grid cursor 위치를 기준으로 overlay 좌표를 계산한다.
- wide character width helper를 공유하거나 동일 규칙으로 맞춘다.

## Acceptance Criteria

Phase 005 완료 기준:

- 한글 IME 조합 중 문자열이 화면에 표시된다.
- 조합 중 문자열이 확정 전 PTY로 전달되지 않는다.
- 조합 확정 후 UTF-8 문자열이 PTY로 전달된다.
- 조합 중 Backspace/Delete가 안정적으로 동작한다.
- 조합이 없을 때 기존 Backspace/Delete/PTX input behavior가 유지된다.
- 한글 wide character 입력/삭제 후 cursor 위치가 어긋나지 않는다.
- `cargo test`가 통과한다.
- 실제 앱에서 한글 입력/삭제 smoke scenario가 통과한다.

## Implementation Update - 2026-06-10

Status: implementation complete. 반복 검증은 Phase 008 compatibility matrix에 편입한다.

구현된 내용:

- `crates/terminal-app/src/composition.rs`에 IME composition state를 추가했다.
- `TerminalView`가 `NSTextInputClient`를 구현하도록 연결했다.
- printable text 입력은 `keyDown:`에서 직접 PTY로 보내지 않고 `interpretKeyEvents:`를 통해 AppKit text input path로 보낸다.
- `setMarkedText:selectedRange:replacementRange:`에서 조합 중 문자열을 view state에 저장한다.
- `insertText:replacementRange:`에서 확정 문자열만 UTF-8 bytes로 PTY에 전달한다.
- 조합 중 문자열은 terminal grid에 쓰지 않고 cursor 위치 위에 overlay로 렌더링한다.
- 조합 중 `deleteBackward:`는 composition을 우선 정리하고, 조합이 없는 일반 Backspace는 기존 PTY input path를 유지한다.
- wide character 삭제 회귀 테스트는 terminal-core의 기존 cursor/grid 테스트와 함께 유지한다.

런타임 안정성 보강:

- 최초 구현에서는 `NSTextInputClient` required method가 클래스 impl에는 있었지만 protocol impl 블록에는 등록되지 않아 앱 시작 시 abort가 발생했다.
- `attributedSubstringForProposedRange:actualRange:`와 `validAttributesForMarkedText`는 객체 반환 메서드이므로 `objc2`의 `method_id` 방식으로 등록했다.
- IME 관련 required selector를 `unsafe impl NSTextInputClient for TerminalView` 블록 안으로 이동해 `objc2` debug protocol verification을 통과하도록 수정했다.

관련 커밋:

- `3b79596 Add IME composition state`
- `76f9a0a Connect NSTextInputClient for IME input`
- `f320e07 Fix NSTextInputClient runtime registration`

검증:

- `cargo test`
- `scripts/bundle-macos-app.sh`
- `target/debug/Minimal Terminal.app/Contents/MacOS/terminal-app` 직접 실행 후 2초 이상 생존 확인

반복 확인할 수동 smoke scenario:

```sh
echo 한글
printf 'abc한글123\n'
```

확인 포인트:

- 한글 조합 중 preedit 문자열이 cursor 위치에 보인다.
- 확정 전에는 PTY로 조기 전달되지 않는다.
- 확정 후 shell input line에 한글이 표시된다.
- 한글과 ASCII를 섞어 입력한 뒤 Backspace/Delete를 눌러 cursor 위치가 어긋나지 않는지 확인한다.
