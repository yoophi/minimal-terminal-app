# Logging

이 문서는 Minimal Terminal의 macOS 로그 설정과 Console.app에서 로그를 확인하는 방법을 정리한다.

## Bundle Identifier

앱 번들은 다음 identifier를 사용한다.

```text
dev.minimal-terminal.app
```

이 값은 [scripts/bundle-macos-app.sh](../scripts/bundle-macos-app.sh)에서 `CFBundleIdentifier`로 설정된다.

Console.app과 `log` CLI에서 앱 로그를 필터링할 때 이 값을 기준으로 사용한다.

## Recommended Log Identity

앱 코드에서 macOS Unified Logging을 사용할 때는 다음 값을 기본값으로 사용한다.

```text
subsystem: dev.minimal-terminal.app
category: app
```

향후 모듈별 로그가 필요하면 category를 나누어 사용한다.

```text
app
pty
renderer
terminal-core
input
```

## Console.app에서 확인하기

1. 앱을 번들로 빌드한다.

```bash
scripts/bundle-macos-app.sh
```

2. 앱을 실행한다.

```bash
open 'target/debug/Minimal Terminal.app'
```

3. macOS의 Console.app을 연다.

```bash
open -a Console
```

4. 검색창 또는 필터에 다음 값을 입력한다.

```text
dev.minimal-terminal.app
```

또는 프로세스 이름으로 필터링한다.

```text
terminal-app
```

## CLI로 실시간 로그 확인하기

Console.app 대신 터미널에서 실시간 로그를 확인할 수 있다.

subsystem 기준:

```bash
log stream --style compact --predicate 'subsystem == "dev.minimal-terminal.app"'
```

프로세스 기준:

```bash
log stream --style compact --predicate 'process == "terminal-app"'
```

최근 10분 로그 확인:

```bash
log show --last 10m --style compact --predicate 'subsystem == "dev.minimal-terminal.app"'
```

## Rust 코드에서 로그 남기기

macOS Console.app에서 안정적으로 확인하려면 stdout/stderr보다 macOS Unified Logging을 사용하는 것이 좋다.

향후 Rust 코드에서는 `os_log` 계열 크레이트 또는 `tracing`과 macOS logging bridge를 사용한다.

권장 방향:

```text
tracing macros
    -> macOS Unified Logging subscriber/layer
    -> Console.app
```

초기 구현 시에는 다음 로그 이벤트를 우선 남긴다.

- 앱 시작
- 메인 창 생성
- PTY 생성
- login shell 실행
- PTY read/write 오류
- 셸 프로세스 종료
- 예상하지 못한 복구 가능한 오류

## Log Level Policy

권장 로그 레벨은 다음과 같다.

```text
error: 앱 기능이 실패했지만 프로세스는 유지되는 오류
default: 앱 시작, 창 생성, 셸 실행 같은 주요 lifecycle 이벤트
info: 상세 lifecycle 또는 상태 이벤트
debug: 개발 중 상태 확인용 상세 이벤트
trace: 대량 출력 가능성이 있는 매우 상세한 이벤트
```

PTY 출력 바이트 전체를 기본 로그에 남기지 않는다. 대량 출력으로 Console.app과 시스템 로그 저장소에 부담을 줄 수 있다.

## Troubleshooting

Console.app에서 로그가 보이지 않으면 다음을 확인한다.

- 앱을 `cargo run`이 아니라 `.app` 번들로 실행했는지 확인한다.
- Console.app 검색어가 `dev.minimal-terminal.app` 또는 `terminal-app`인지 확인한다.
- `log stream --predicate 'process == "terminal-app"'`로 프로세스 기준 로그가 보이는지 확인한다.
- 앱이 실행 중인지 확인한다.

```bash
pgrep -fl terminal-app
```

현재 단계에서 앱 코드가 macOS Unified Logging을 직접 호출하지 않으면 subsystem 기준 로그는 보이지 않을 수 있다. 이 경우 프로세스 기준 필터를 먼저 사용한다.
