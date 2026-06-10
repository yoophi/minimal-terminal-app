# Phase 002: Login Shell Output and App Lifecycle Logging

## Summary

Phase 002는 빈 AppKit 창에 login shell 출력을 표시하기 위한 최소 경로를 구현한 단계다. 또한 Console.app에서 확인 가능한 app lifecycle 로그를 구간별로 추가했다.

이 단계의 산출물은 다음과 같다.

- macOS Unified Logging 연동
- PTY 기반 login shell 실행
- PTY 출력 reader thread
- 단순 terminal output buffer
- AppKit `TerminalView` 구현
- login shell 출력 렌더링
- 고정폭 폰트 기반 출력 표시
- 키보드 입력을 PTY로 전달
- Console.app 및 `/usr/bin/log` 사용 문서화

관련 커밋:

```text
67cdcb6 Add login shell output and lifecycle logging
7615aaf Document app log filtering commands
```

## Goal

Phase 002의 목표는 다음 질문에 답하는 것이다.

```text
빈 macOS 창 안에 login shell의 출력을 표시하려면 어떤 작업이 필요한가?
```

최소 구현 기준은 다음과 같다.

- 앱 실행 시 login shell을 생성한다.
- PTY를 통해 shell output을 읽는다.
- shell output을 화면에 표시한다.
- 키보드 입력을 shell로 전달한다.
- 앱 lifecycle과 PTY lifecycle을 Console.app에서 확인할 수 있다.

이 단계에서는 완전한 터미널 에뮬레이션을 목표로 하지 않았다. ANSI/VT 호환성, 정확한 grid/cursor 처리, 색상, scrollback, resize 동기화는 후속 단계로 남겼다.

## Required Work to Display Login Shell Output

login shell 출력을 화면에 표시하려면 다음 작업이 필요하다.

## 1. PTY에서 login shell 실행

터미널 앱은 일반 pipe가 아니라 PTY를 통해 셸을 실행해야 한다. 셸, vim, ssh, tmux 같은 프로그램은 자신이 terminal에 연결되어 있는지 여부에 따라 동작이 달라지기 때문이다.

Phase 002에서는 macOS의 `forkpty`를 사용했다.

구현 위치:

```text
crates/terminal-app/src/pty.rs
```

동작 흐름:

```text
spawn_login_shell
  -> forkpty
  -> child process에서 execl(shell, login_argv0)
  -> parent process에서 master fd 보관
```

login shell로 실행하기 위해 argv[0]에 `-` prefix를 붙인다.

예:

```text
shell: /bin/zsh
argv[0]: -zsh
```

이 방식은 shell에게 login shell로 동작하라고 알려주는 일반적인 Unix convention이다.

현재 shell 선택 순서:

```text
$SHELL
fallback: /bin/zsh
```

PTY child 환경에는 다음 값도 설정한다.

```text
TERM=xterm-256color
```

## 2. PTY 출력 읽기

PTY master fd에서 shell output을 읽기 위해 background thread를 생성했다.

이유:

- AppKit main thread를 blocking하면 UI가 멈춘다.
- PTY read는 데이터가 없으면 blocking될 수 있다.
- UI thread와 IO thread를 분리해야 안정성이 좋아진다.

구현 흐름:

```text
PTY master fd
  -> reader thread
  -> read bytes
  -> TerminalBuffer append
```

reader thread는 다음 상황을 로그로 남긴다.

- reader thread 시작
- EOF 도달
- read error
- terminal buffer lock poison
- thread 종료

## 3. Output Buffer로 변환

PTY에서 읽은 값은 raw bytes다. 화면에 표시하려면 최소한 문자열로 바꿔야 한다.

Phase 002에서는 [terminal_buffer.rs](../../crates/terminal-app/src/terminal_buffer.rs)에 `TerminalBuffer`를 만들었다.

현재 처리 방식:

- `String::from_utf8_lossy`로 UTF-8 변환
- `\n`은 line commit
- `\r`은 현재 줄 clear
- backspace/delete는 마지막 문자 제거
- tab은 공백 4개로 변환
- 일부 control character 무시
- ANSI escape sequence는 매우 단순하게 skip

현재 모델은 "진짜 터미널 grid"가 아니라 "화면에 보일 텍스트 로그에 가까운 버퍼"다.

이 선택의 이유:

- login shell prompt가 보이는지 빠르게 검증 가능
- PTY, reader, AppKit 렌더링 경로를 먼저 연결할 수 있음
- 이후 `terminal-core`로 교체할 수 있는 지점을 확인 가능

한계:

- cursor 위치를 정확히 처리하지 않음
- line wrap이 없음
- clear screen이 없음
- ANSI 색상/스타일이 없음
- alternate screen이 없음
- vim, less, tmux 같은 TUI 앱을 제대로 표시할 수 없음

## 4. AppKit View에서 렌더링

출력 버퍼를 화면에 표시하기 위해 `NSView` subclass인 `TerminalView`를 추가했다.

구현 위치:

```text
crates/terminal-app/src/terminal_view.rs
```

현재 `TerminalView`의 역할:

- 검은 배경 그리기
- `TerminalBuffer`에서 visible text 읽기
- 흰색 텍스트로 draw
- `NSFont::userFixedPitchFontOfSize`를 사용해 고정폭 폰트 적용
- keyDown 이벤트 수신
- keyDown characters를 PTY writer로 전달
- timer를 사용해 주기적으로 redraw 요청

렌더링 흐름:

```text
drawRect:
  -> draw black background
  -> set white foreground color
  -> set fixed-pitch font
  -> lock TerminalBuffer
  -> visible_text(200)
  -> draw NSString
```

현재 렌더링은 최소 구현이다. 고정폭 폰트는 적용했지만, 아직 cell 단위 renderer는 아니다. 즉 글자를 터미널 grid에 배치하는 것이 아니라 하나의 문자열 블록으로 그린다.

## 5. UI 갱신 트리거

PTY reader thread가 AppKit view를 직접 호출하면 안 된다. AppKit UI 작업은 main thread에서 수행되어야 한다.

Phase 002에서는 단순하고 안정적인 방법으로 `NSTimer`를 사용했다.

흐름:

```text
NSTimer
  -> redrawTimerFired:
  -> setNeedsDisplay(true)
  -> AppKit calls drawRect:
```

이 방식의 장점:

- reader thread에서 UI를 직접 만지지 않음
- 구현이 단순함
- 초기 MVP에서 충분히 동작 확인 가능

단점:

- 출력이 없을 때도 주기적으로 redraw 요청 가능
- 고출력 상황에서 효율적이지 않음
- 다음 단계에서는 main queue dispatch 또는 event-driven invalidation으로 개선 가능

## Current Runtime Flow

현재 전체 흐름은 다음과 같다.

```text
App launch
  -> NSApplicationDelegate.applicationDidFinishLaunching
  -> create NSWindow
  -> create TerminalBuffer
  -> forkpty + exec login shell
  -> create TerminalView
  -> attach TerminalView to NSWindow
  -> start redraw timer

login shell output
  -> PTY master fd
  -> reader thread
  -> TerminalBuffer
  -> TerminalView drawRect
  -> macOS window

keyboard input
  -> TerminalView keyDown:
  -> event.characters()
  -> PtyWriter.write_all
  -> PTY master fd
  -> login shell
```

## App Lifecycle Logging

Phase 002에서는 Console.app에서 확인 가능한 로그를 추가했다.

구현 위치:

```text
crates/terminal-app/src/logging.rs
```

사용한 subsystem:

```text
dev.minimal-terminal.app
```

사용한 category:

```text
app
pty
```

현재 lifecycle 로그:

```text
[dev.minimal-terminal.app:app] main started
[dev.minimal-terminal.app:app] application run loop starting
[dev.minimal-terminal.app:app] applicationDidFinishLaunching started
[dev.minimal-terminal.app:app] create_main_window started
[dev.minimal-terminal.app:app] create_main_window completed
[dev.minimal-terminal.app:pty] pty spawn requested
[dev.minimal-terminal.app:pty] pty spawn succeeded: child_pid=... shell=/bin/zsh
[dev.minimal-terminal.app:pty] pty reader thread starting
[dev.minimal-terminal.app:app] applicationDidFinishLaunching completed
[dev.minimal-terminal.app:app] windowWillClose
[dev.minimal-terminal.app:app] applicationWillTerminate
```

로그 레벨은 lifecycle 이벤트가 Console.app에서 잘 보이도록 `default`를 사용했다. 초기에는 `info`를 사용했지만, `log show` 기본 조회에서 확인이 어렵다는 점을 확인하고 `default`로 조정했다.

## Log Commands

우리 앱이 직접 남긴 로그만 확인하려면 subsystem으로 필터링한다.

```bash
/usr/bin/log stream --style compact --predicate 'subsystem == "dev.minimal-terminal.app"'
```

최근 10분 로그:

```bash
/usr/bin/log show --last 10m --style compact --predicate 'subsystem == "dev.minimal-terminal.app"'
```

앱 lifecycle 로그만 보기:

```bash
/usr/bin/log stream --style compact --predicate 'subsystem == "dev.minimal-terminal.app" AND category == "app"'
```

PTY 로그만 보기:

```bash
/usr/bin/log stream --style compact --predicate 'subsystem == "dev.minimal-terminal.app" AND category == "pty"'
```

프로세스 기준 필터:

```bash
/usr/bin/log stream --style compact --predicate 'process == "terminal-app"'
```

프로세스 기준 필터는 AppKit, CoreFoundation, LaunchServices, Metal 등 시스템 프레임워크 로그까지 같이 보여준다. 우리 앱 로그만 볼 때는 subsystem 필터가 더 적합하다.

상세 문서는 [docs/LOGGING.md](../LOGGING.md)에 정리했다.

## Verification

Phase 002에서 수행한 검증:

```bash
cargo check
scripts/bundle-macos-app.sh
open 'target/debug/Minimal Terminal.app'
/usr/bin/log show --last 2m --style compact --predicate 'subsystem == "dev.minimal-terminal.app"'
```

검증 결과:

- `cargo check` 통과
- `.app` 번들 재생성 성공
- 앱 실행 성공
- `terminal-app` 프로세스 실행 확인
- login shell child process 생성 확인
- lifecycle 로그가 subsystem 기준으로 표시됨

확인된 로그 예:

```text
terminal-app: [dev.minimal-terminal.app:app] main started
terminal-app: [dev.minimal-terminal.app:app] application run loop starting
terminal-app: [dev.minimal-terminal.app:app] applicationDidFinishLaunching started
terminal-app: [dev.minimal-terminal.app:app] create_main_window started
terminal-app: [dev.minimal-terminal.app:app] create_main_window completed
terminal-app: [dev.minimal-terminal.app:pty] pty spawn requested
terminal-app: [dev.minimal-terminal.app:pty] pty spawn succeeded: child_pid=30014 shell=/bin/zsh
terminal-app: [dev.minimal-terminal.app:pty] pty reader thread starting
terminal-app: [dev.minimal-terminal.app:app] applicationDidFinishLaunching completed
```

## Issues and Lessons

### `process == "terminal-app"` Is Too Broad

처음에는 프로세스 기준으로 로그를 봤다.

```bash
log stream --predicate 'process == "terminal-app"'
```

이 방식은 우리 앱 로그뿐 아니라 같은 프로세스에서 실행되는 macOS 프레임워크 로그까지 모두 보여준다.

교훈:

- 앱 자체 로그 확인에는 subsystem 필터가 더 좋다.
- 프로세스 필터는 시스템 프레임워크 문제까지 볼 때 사용한다.

### `/usr/bin/log` Path Is Safer

일부 셸 환경에서 `log` 명령이 alias나 함수와 충돌할 수 있다.

실제로 `log show ...` 실행 시 zsh에서 `too many arguments`가 발생할 수 있었다.

교훈:

- 문서에서는 `/usr/bin/log`를 명시한다.
- 디버깅 명령은 사용자의 shell customization 영향을 덜 받도록 작성한다.

### AppKit Drawing API Required Runtime Verification

처음에는 `NSString drawAtPoint:` 단일 인자 메시지를 사용했지만, 런타임에서 method not found panic이 발생했다.

수정:

```text
drawAtPoint:withAttributes:
```

교훈:

- Objective-C message send는 컴파일만으로 충분하지 않다.
- AppKit drawing path는 실제 실행 검증이 필요하다.

### PTY Reader Must Not Touch UI Directly

PTY reader thread는 background thread에서 동작한다. 여기서 AppKit view를 직접 갱신하면 thread-safety 문제가 생길 수 있다.

현재 해결:

- reader thread는 `TerminalBuffer`만 갱신
- `NSTimer`가 main thread에서 `setNeedsDisplay(true)` 호출

교훈:

- UI thread 규칙은 초기부터 지켜야 한다.
- 성능 최적화는 이후 event-driven redraw로 개선하되, correctness를 먼저 확보한다.

### Current Buffer Is Not a Terminal Emulator

현재 `TerminalBuffer`는 login shell prompt를 보여주기 위한 최소 버퍼다.

이 버퍼는 다음을 정확히 처리하지 못한다.

- cursor movement
- ANSI color
- clear screen
- alternate screen
- line wrapping
- scroll regions
- Unicode width
- resize
- full-screen TUI apps

교훈:

- Phase 002의 목적은 "PTY to screen pipeline" 검증이다.
- 실제 터미널 품질은 `terminal-core`의 grid/parser 구현이 필요하다.

## Remaining Work

다음 단계에서 필요한 작업:

- `terminal-core` crate 생성
- terminal grid/cell/cursor 모델 구현
- ANSI/VT parser 도입
- 고정폭 폰트 기반 cell 렌더링
- cursor 표시
- scrollback model
- PTY resize 처리
- 창 크기에서 rows/cols 계산
- keyboard special key encoding
- shell 종료 상태 표시
- 에러 로그와 사용자 표시 상태 분리

## Result

Phase 002 종료 시점에는 앱 창 안에 login shell 출력을 표시하는 최소 pipeline이 완성되었다.

현재 앱은 다음을 수행할 수 있다.

- macOS AppKit window 표시
- PTY 기반 login shell 실행
- shell output 읽기
- output을 화면에 표시
- 키 입력을 shell로 전달
- lifecycle과 PTY 상태를 Console.app에서 확인

이 단계는 아직 완전한 터미널 에뮬레이터는 아니지만, 이후 `terminal-core`와 renderer를 붙일 수 있는 실행 가능한 기반이다.
