# 터미널 호환성 판단 근거

이 문서는 CSI와 terminal compatibility를 판단할 때 어떤 표준과 테스트 도구를 기준으로 삼을지 정리한다. 결론부터 말하면, CSI 준수율을 수치로 산정하는 공식 RFC나 단일 공식 테스트 suite는 없다. 이 프로젝트는 표준 문서, de facto reference, 실제 테스트 evidence를 함께 사용한다.

## 공식 RFC는 없다

CSI와 ANSI/VT terminal control sequence는 IETF RFC 계열 표준이 아니다. 따라서 "CSI 준수율"을 정의하는 공식 RFC나 RFC 기반 점수 체계는 없다.

터미널 호환성은 보통 다음 기준을 조합해 판단한다.

- ECMA-48 / ISO/IEC 6429 같은 control function 표준
- DEC VT 계열 terminal 문서
- xterm control sequence 문서
- `vttest` 같은 de facto 테스트 프로그램
- 실제 shell/TUI 앱 smoke test
- 프로젝트 자체 compatibility matrix와 regression test

## 공식 표준 문서

### ECMA-48

ECMA-48은 CSI를 포함한 control function과 coded representation을 정의하는 핵심 표준이다. CSI의 기본 형식, parameter, intermediate byte, final byte 같은 구조를 이해할 때 가장 먼저 참조할 수 있다.

- ECMA-48 standard page: <https://ecma-international.org/publications-and-standards/standards/ecma-48/>
- ECMA-48 PDF: <https://ecma-international.org/wp-content/uploads/ECMA-48_5th_edition_june_1991.pdf>

### ISO/IEC 6429

ISO/IEC 6429는 ECMA-48과 대응되는 국제 표준이다. 공개 접근성 면에서는 ECMA-48 PDF가 더 실용적이므로, 이 프로젝트에서는 ECMA-48을 우선 링크한다.

## 역사적/실무 기준 문서

### DEC VT100/VT 계열 문서

DEC VT100/VT220 계열 문서는 실제 terminal emulator 호환성의 역사적 기준이다. ANSI mode, cursor movement, screen erase, line erase 등 많은 기본 동작이 이 계열 문서를 통해 널리 퍼졌다.

- VT100 User Guide, Chapter 3: <https://vt100.net/docs/vt100-ug/chapter3.html>

### xterm Control Sequences

현대 terminal emulator 구현에서는 xterm control sequence 문서가 매우 중요한 실무 기준이다. ECMA-48, DEC VT 계열 동작, xterm 확장 sequence가 함께 정리되어 있다.

- XTerm Control Sequences: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html>

이 프로젝트가 `TERM=xterm-256color`를 사용한다면, xterm 계열 동작과 완전히 같지는 않더라도 대표적인 xterm-compatible behavior를 일정 수준 제공해야 한다.

## 테스트 도구

### vttest

`vttest`는 VT100/VT220/VT420/xterm 계열 호환성을 확인하는 대표적인 테스트 프로그램이다. 공식 표준 테스트 suite는 아니지만, terminal emulator 구현에서 사실상 표준에 가까운 검증 도구로 쓰인다.

- VTTEST: <https://invisible-island.net/vttest/>
- vttest manpage: <https://xterm.dev/manpage-vttest/>

`vttest` 결과는 향후 compatibility matrix의 app/runtime evidence로 기록할 수 있다. 단, 이 프로젝트의 초기 MVP에서는 모든 `vttest` 항목 통과를 완료 기준으로 삼지 않는다.

## 이 프로젝트의 판단 방식

공식 "준수율"이 없기 때문에 이 프로젝트는 다음 방식으로 호환성을 판단한다.

1. Sequence별 지원 상태를 `docs/compatibility/matrix.md`에 기록한다.
2. `supported`라고 표시한 항목은 자동 테스트 또는 runtime smoke evidence를 연결한다.
3. 지원하지 않는 항목은 `docs/compatibility/known-gaps.md`에 gap으로 기록한다.
4. GUI/runtime에서만 확인 가능한 동작은 `docs/compatibility/smoke-tests.md` 절차로 반복 확인한다.
5. 새 sequence를 구현할 때는 가능한 한 parser/state fixture를 먼저 추가한다.

## 준수율 대신 추적할 지표

수치형 CSI 준수율 대신 다음 지표를 사용한다.

- ECMA-48 기본 CSI 형식 처리 여부
- DEC VT100/VT220 계열 주요 sequence 처리 여부
- xterm extension 중 실제 사용 빈도가 높은 sequence 처리 여부
- `vttest` 또는 유사 테스트 도구의 통과/실패 항목
- `less`, `vim`, `top`, `fzf` 같은 실제 TUI smoke 결과
- known gap의 우선순위와 감소 추세

이 방식이 안정성 우선 MVP에 더 적합하다. 구현하지 않은 기능을 추상적인 퍼센트로 숨기지 않고, 실제 sequence와 evidence 단위로 추적할 수 있기 때문이다.

