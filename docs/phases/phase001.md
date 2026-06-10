# Phase 001: Project Goal, Architecture, and Native Window Scaffold

## Summary

Phase 001은 macOS용 Rust 기반 터미널 에뮬레이터 프로젝트의 방향을 정하고, AppKit 기반 네이티브 앱의 최소 실행 기반을 만든 단계다.

이 단계의 산출물은 다음과 같다.

- 프로젝트 목표 문서화
- MVP 범위 정의
- 코드베이스 아키텍처 방향 정리
- Rust Cargo workspace 구성
- macOS AppKit 기반 빈 창 앱 구현
- `.app` 번들 생성 스크립트 작성
- 초기 README 작성
- GitHub 저장소 생성 및 첫 push

관련 커밋:

```text
9c1e897 Initial macOS terminal app scaffold
```

## Initial Goal

프로젝트의 최초 목표는 다음과 같이 정의했다.

```text
macOS에서 사용할 수 있는 안정성 우선의 터미널 에뮬레이터를 개발한다.
```

기본 조건은 다음과 같다.

- 개발 언어: Rust
- 대상 플랫폼: macOS
- 최우선 가치: 안정성

이 내용은 [GOAL.md](../../GOAL.md)에 정리했다.

## MVP Scope

MVP는 "macOS에서 기본 셸을 안정적으로 실행할 수 있는 단일 창 터미널 에뮬레이터"로 잡았다.

포함 기능으로는 다음을 정의했다.

- macOS 기본 셸 실행
- PTY 기반 프로세스 실행
- 키보드 입력 전달
- 표준 터미널 출력 표시
- ANSI escape sequence 기본 처리
- 고정폭 폰트 렌더링
- 스크롤백 지원
- 텍스트 선택
- 복사 및 붙여넣기
- 창 크기 변경 시 터미널 크기 동기화
- 프로세스 종료 상태 표시
- 비정상 종료 시 앱 크래시 방지

초기 제외 범위도 명시했다.

- 탭
- 분할 화면
- GPU 가속 렌더링
- 플러그인 시스템
- 원격 동기화
- 테마 편집 UI
- 복잡한 설정 화면
- Windows/Linux 지원

이 범위를 초기에 명시한 이유는 터미널 앱이 기능 범위가 쉽게 커지는 종류의 프로젝트이기 때문이다. 특히 탭, 분할, 테마, GPU 렌더링 같은 기능은 매력적이지만 안정적인 PTY 실행과 터미널 상태 모델이 잡히기 전에는 복잡도를 크게 높인다.

## Architecture Direction

아키텍처는 엄격한 hexagonal architecture를 그대로 적용하기보다, hexagonal-inspired modular monolith 방향으로 정했다.

핵심 원칙은 다음과 같다.

- 터미널 core는 OS와 UI에 의존하지 않는다.
- PTY와 macOS AppKit 코드는 어댑터 계층으로 격리한다.
- renderer는 terminal state를 읽기만 하고 직접 변경하지 않는다.
- 초기에는 과도한 trait 추상화를 만들지 않는다.
- 실제 테스트와 교체 가능성이 필요한 경계부터 추상화한다.

권장 크레이트 구조는 다음과 같이 설계했다.

```text
crates/
├── terminal-core/
├── terminal-pty/
├── terminal-renderer/
└── terminal-app/
```

Phase 001에서는 이 전체 구조를 모두 구현하지 않고, 실행 가능한 최소 기반인 `terminal-app`부터 만들었다. 이는 프로젝트 초기에는 "아키텍처 문서만 있고 실행 가능한 코드가 없는 상태"보다 "작지만 실제 실행되는 macOS 앱"이 더 유용하다고 판단했기 때문이다.

아키텍처 문서는 [ARCHITECTURE.md](../../ARCHITECTURE.md)에 정리했다.

## Implementation

Phase 001에서 작성한 핵심 코드는 다음과 같다.

```text
Cargo.toml
crates/terminal-app/Cargo.toml
crates/terminal-app/src/main.rs
scripts/bundle-macos-app.sh
```

### Cargo Workspace

루트 [Cargo.toml](../../Cargo.toml)은 workspace로 구성했다.

초기 workspace member는 하나다.

```text
crates/terminal-app
```

처음부터 workspace로 시작한 이유는 이후 `terminal-core`, `terminal-pty`, `terminal-renderer`를 분리할 가능성이 높기 때문이다.

### AppKit Binding Choice

초기 구현에서는 `cocoa` crate를 사용해 빈 창을 만드는 방식으로 시작했다.

하지만 `cargo check` 결과 `cocoa` crate에서 deprecated 경고가 다수 발생했다. 안정성 우선 프로젝트에서 시작부터 경고가 많은 상태를 남기지 않는 것이 좋다고 판단해 최신 계열인 `objc2`, `objc2-app-kit`, `objc2-foundation`으로 전환했다.

이 선택의 장점은 다음과 같다.

- 최신 Rust Objective-C binding 사용
- deprecated 경고 제거
- AppKit API와 memory/thread ownership을 더 명시적으로 다룰 수 있음

단점은 다음과 같다.

- 타입과 feature gate가 세밀해서 초기 진입 장벽이 높음
- `NSView`, `NSWindow`, `NSApplicationDelegate` 구현 시 Rust 타입 경계를 더 많이 신경 써야 함

### Native Empty Window

Phase 001의 앱은 다음 일을 수행한다.

- `NSApplication` 생성
- application delegate 설정
- `NSWindow` 생성
- 창 title을 `Minimal Terminal`로 설정
- 창 닫힘 시 앱 종료
- unbundled 실행 시 앱 활성화

초기에는 빈 창만 표시한다. 이 시점에는 PTY, 셸 실행, 렌더링, 입력 전달은 구현하지 않았다.

### App Bundle Script

`cargo run` 또는 바이너리 직접 실행은 AppKit 앱 테스트에는 한계가 있었다. 특히 백그라운드로 실행할 때 도구 세션 수명주기와 엮일 수 있고, macOS 앱처럼 launch services를 통해 실행되는 경로가 필요했다.

그래서 [scripts/bundle-macos-app.sh](../../scripts/bundle-macos-app.sh)를 추가했다.

이 스크립트는 다음을 수행한다.

- `cargo build -p terminal-app`
- `target/debug/Minimal Terminal.app` 생성
- `Contents/MacOS/terminal-app` 복사
- `Contents/Info.plist` 생성
- `CFBundleIdentifier` 설정

번들 식별자는 다음으로 정했다.

```text
dev.minimal-terminal.app
```

이 값은 Phase 002에서 Console.app 로그 필터링 기준으로도 사용된다.

## Verification

Phase 001에서 수행한 검증은 다음과 같다.

```bash
cargo check
cargo build -p terminal-app
scripts/bundle-macos-app.sh
open 'target/debug/Minimal Terminal.app'
```

검증 결과:

- `cargo check` 통과
- `cargo build` 통과
- `.app` 번들 생성 성공
- macOS 빈 창 표시 성공

## Key Decisions

### Start With a Native macOS App

터미널 에뮬레이터의 core는 플랫폼 독립적으로 만들 수 있지만, 이 프로젝트의 대상 플랫폼은 macOS다. 따라서 첫 실행 기반은 크로스플랫폼 GUI가 아니라 AppKit 네이티브 앱으로 만들었다.

이 결정의 이유는 다음과 같다.

- macOS PTY, input method, clipboard, window lifecycle과 직접 맞물릴 예정
- 안정성 우선 앱에서는 플랫폼 동작을 정확히 이해하는 것이 중요
- `.app` 번들, Console.app, LaunchServices 같은 macOS 개발 흐름을 초기에 확보할 필요가 있음

### Keep Phase 001 Small

Phase 001에서는 셸 실행을 넣지 않았다. 먼저 창 lifecycle과 번들 실행이 안정적으로 되는지 확인했다.

이 접근은 이후 문제가 생겼을 때 원인을 나누기 쉽게 한다.

- 창이 안 뜨는 문제
- 앱 lifecycle 문제
- PTY 문제
- 렌더링 문제
- 입력 처리 문제

이들을 한 번에 도입하지 않고 단계별로 쌓는 것이 회귀 분석에 유리하다.

## Issues and Lessons

### Deprecated Cocoa Binding

처음 사용한 `cocoa` crate는 동작했지만 deprecated 경고가 많았다.

교훈:

- 초기 scaffold에서도 경고 없는 상태를 유지하는 편이 좋다.
- 특히 장기 프로젝트에서는 시작점의 기술 부채가 이후 모든 작업의 기준이 된다.

### AppKit Requires Correct App Lifecycle

단순히 바이너리를 실행하는 것과 `.app` 번들로 실행하는 것은 다르다.

교훈:

- macOS 앱 검증은 가능한 한 `.app` 번들 실행 경로를 포함해야 한다.
- `Info.plist`, bundle identifier, activation policy는 이후 logging, permissions, app lifecycle에도 영향을 준다.

## Result

Phase 001 종료 시점에는 "빈 macOS 네이티브 창을 띄우는 Rust 앱"이 준비되었다.

이 단계는 실제 터미널 기능은 없지만, 이후 작업의 기반이 되는 다음 요소를 제공한다.

- Rust workspace
- AppKit 앱 lifecycle
- window 생성/종료 처리
- `.app` 번들 실행 경로
- 프로젝트 목표와 아키텍처 문서

