# Phase 083 - JIS Katakana Charset

## 목표

JIS Roman 이후 xterm-documented 94-character charset coverage를 JIS Katakana까지 확장한다.

## 범위

1. `Charset::JisKatakana`를 추가한다.
2. `ESC ( I`, `ESC ) I`, `ESC * I`, `ESC + I` designation을 처리한다.
3. JIS Katakana replacement characters를 매핑한다.
4. G0, G2 single shift, right-side G1 locking shift 경로를 parser test로 검증한다.
5. state rendering test와 compatibility 문서를 갱신한다.

## 비범위

- JIS Roman 외 다른 Japanese charset은 별도 phase로 남긴다.
- raw 8-bit non-UTF-8 byte stream 처리는 다루지 않는다.
- keyboard layout과 입력 method는 다루지 않는다.

## 판단 근거

- xterm control sequence 문서는 `C = I`를 JIS Katakana, VT382 항목으로 기록한다: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html>
- xterm 410 release source의 `charsets.dat` `map_JIS_Katakana` 테이블은 `!`-`_`를 U+FF61-U+FF9F Halfwidth Katakana block으로 매핑하고, `` ` ``-`}`를 undefined로 표시한다: <https://invisible-island.net/datafiles/release/xterm.tar.gz>

## Acceptance Criteria

- [done] parser가 JIS Katakana designation을 G0-G3에 적용한다.
- [done] JIS Katakana replacement character mapping이 parser/state test로 검증되어 있다.
- [done] matrix와 known gap이 JIS Katakana evidence를 반영한다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## 결과

- JIS Katakana에서 `!`-`_`가 U+FF61-U+FF9F Halfwidth Katakana block으로 매핑된다.
- xterm table에서 undefined인 `` ` ``-`}`는 undefined marker `␦`로 매핑한다.
- G0-G3 designation에서 `I` final character를 JIS Katakana로 처리한다.
