# Phase 086 - DEC Cyrillic Charset

## 목표

DEC Technical 이후 남아 있는 xterm-documented 94-character charset coverage를 DEC Cyrillic까지 확장한다.

## 범위

1. `Charset::DecCyrillic`을 추가한다.
2. `ESC ( & 4`, `ESC ) & 4`, `ESC * & 4`, `ESC + & 4` designation을 처리한다.
3. DEC Cyrillic replacement characters를 매핑한다.
4. G0, G2 single shift, right-side G1 locking shift 경로를 parser test로 검증한다.
5. state rendering test와 compatibility 문서를 갱신한다.

## 비범위

- ISO Latin-Cyrillic supplemental 96-character set은 다루지 않는다.
- DEC Russian NRCS와의 통합/별칭 처리는 다루지 않는다.
- raw 8-bit non-UTF-8 byte stream 처리는 다루지 않는다.

## 판단 근거

- xterm control sequence 문서는 `C = & 4`를 DEC Cyrillic, VT500 항목으로 기록한다: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html>
- xterm 410 release source의 `charsets.dat` `map_DEC_Cyrillic` 테이블은 `@`-`_`를 Cyrillic 소문자, `` ` ``-`~`를 Cyrillic 대문자로 매핑하고 `!`-`?`를 undefined로 기록한다: <https://invisible-island.net/datafiles/release/xterm.tar.gz>

## Acceptance Criteria

- [done] parser가 DEC Cyrillic designation을 G0-G3에 적용한다.
- [done] DEC Cyrillic replacement character mapping이 parser/state test로 검증되어 있다.
- [done] matrix와 known gap이 DEC Cyrillic evidence를 반영한다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## 결과

- DEC Cyrillic에서 `@`-`_`는 Cyrillic 소문자로 매핑된다.
- DEC Cyrillic에서 `` ` ``-`~`는 Cyrillic 대문자로 매핑된다.
- xterm table에서 undefined인 `!`-`?`는 undefined marker `␦`로 매핑한다.
- G0-G3 designation에서 `& 4` intermediate/final sequence를 DEC Cyrillic로 처리한다.
