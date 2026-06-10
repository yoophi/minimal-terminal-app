# CSI란 무엇인가

CSI는 `Control Sequence Introducer`의 약자다. 터미널 입력 byte stream에서 cursor 이동, 화면 지우기, 색상 변경, scroll region 설정 같은 제어 명령이 시작된다는 표시로 쓰인다.

일반적으로 CSI는 다음 byte 조합으로 표현된다.

```text
ESC [
```

8-bit control code로는 `0x9B`도 CSI를 의미할 수 있다. 이 프로젝트는 `ESC [`와 `0x9B` 형태를 모두 parser/state test로 고정하되, 문서 예시는 읽기 쉬운 `ESC [` 형태를 기준으로 설명한다.

예를 들어 문서에서 `CSI 2J`라고 쓰면 실제 byte stream은 보통 다음과 같다.

```text
ESC [ 2 J
```

이는 "화면 전체 지우기" 계열의 명령이다.

## CSI Sequence 구조

CSI sequence는 보통 다음 구조를 가진다.

```text
CSI parameters intermediates final-byte
```

예시:

| 표기 | 실제 형태 | 의미 |
| --- | --- | --- |
| `CSI A` | `ESC [ A` | cursor를 위로 이동 |
| `CSI 10C` | `ESC [ 10 C` | cursor를 오른쪽으로 10칸 이동 |
| `CSI 2J` | `ESC [ 2 J` | 화면 지우기 |
| `CSI K` | `ESC [ K` | cursor부터 줄 끝까지 지우기 |
| `CSI 1;31m` | `ESC [ 1 ; 31 m` | bold + red text |
| `CSI ?25l` | `ESC [ ? 25 l` | cursor 숨김 |
| `CSI ?25h` | `ESC [ ? 25 h` | cursor 표시 |
| `CSI 2;4r` | `ESC [ 2 ; 4 r` | scroll region 설정 |

## 이 프로젝트에서 CSI를 다루는 이유

터미널 앱은 shell이나 TUI 프로그램이 보내는 byte stream을 화면 상태로 해석해야 한다. shell prompt, `less`, `vim`, `top` 같은 프로그램은 단순 텍스트만 출력하지 않고 CSI sequence를 사용해 화면을 계속 갱신한다.

CSI를 제대로 처리하지 않으면 다음 문제가 생긴다.

- shell prompt redraw 후 이전 글자가 남는다.
- Backspace, cursor 이동, 줄 편집이 화면과 실제 shell 상태 사이에서 어긋난다.
- ANSI color와 bold/underline 같은 style이 표시되지 않는다.
- `less`, `vim`, `top` 같은 full-screen TUI가 깨진 화면을 만든다.
- alternate screen 진입/복귀 후 기존 shell 화면이 복원되지 않는다.
- resize, scroll region, insert/delete line 처리에서 화면 일부가 밀리거나 사라진다.

따라서 CSI 지원은 "화면 꾸미기"가 아니라 terminal emulator의 기본 정확성과 안정성에 직접 연결된다.

## 왜 준수해야 하는가

CSI sequence는 사실상 terminal program과 terminal emulator 사이의 공통 계약이다. 많은 프로그램은 사용자의 terminal이 `xterm-256color` 또는 유사 terminal behavior를 제공한다고 가정하고 CSI 명령을 보낸다.

이 프로젝트는 PTY child process에 `TERM=xterm-256color`를 제공한다. 이 값을 제공한다는 것은 적어도 대표적인 xterm 계열 동작을 일정 수준 처리하겠다는 의미가 된다. CSI 동작을 과도하게 무시하거나 다르게 해석하면 앱은 스스로 선언한 terminal capability와 실제 동작이 어긋난다.

CSI 준수가 중요한 이유는 다음과 같다.

- **정확성**: 프로그램이 의도한 cursor 위치, 화면 지우기, style 변경을 실제 화면에 반영해야 한다.
- **상호운용성**: shell, editor, pager, monitor, fuzzy finder 등 다양한 terminal program과 같이 동작해야 한다.
- **안정성**: 알 수 없는 sequence를 받아도 크래시하지 않고 안전하게 무시하거나 복구해야 한다.
- **회귀 방지**: sequence별 status와 evidence를 남기면 새 기능이 기존 shell/TUI 동작을 깨뜨리는지 추적할 수 있다.
- **사용자 신뢰성**: 입력 위치와 화면 표시가 어긋나면 사용자는 명령을 잘못 입력하거나 잘못 복사할 수 있다.

## 준수의 범위

이 프로젝트는 모든 ANSI/VT/xterm sequence를 한 번에 완전 지원하는 것을 목표로 하지 않는다. 대신 다음 원칙으로 확장한다.

- 자주 쓰이는 CSI sequence부터 구현한다.
- 구현한 sequence는 parser/state test 또는 runtime smoke evidence와 연결한다.
- 지원하지 않는 sequence는 `docs/compatibility/matrix.md`와 `docs/compatibility/known-gaps.md`에 명시한다.
- 알 수 없는 sequence는 앱을 크래시시키지 않고 안전하게 무시하는 것을 기본값으로 둔다.
- TUI smoke test에서 실제 문제가 확인된 sequence를 우선순위로 올린다.

현재 CSI 지원 상태는 `docs/compatibility/matrix.md`에서 추적한다.

CSI와 terminal compatibility 판단에 사용하는 표준, reference, 테스트 도구는 `docs/compatibility/standards-and-tests.md`에 정리한다.
