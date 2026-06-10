# Phase 089 - DEC Turkish Supplemental Charset

## 목표

DEC Hebrew Supplemental 이후 남아 있는 xterm-documented 94-character charset coverage를 DEC Turkish Supplemental까지 확장한다.

## 범위

1. `Charset::DecTurkishSupplemental`을 추가한다.
2. `ESC ( % 0`, `ESC ) % 0`, `ESC * % 0`, `ESC + % 0` designation을 처리한다.
3. DEC Turkish Supplemental replacement characters를 매핑한다.
4. G0, G2 single shift, right-side G1 locking shift 경로를 parser test로 검증한다.
5. state rendering test와 compatibility 문서를 갱신한다.

## 비범위

- ISO Latin-5 supplemental 96-character set은 다루지 않는다.
- Turkish NRCS와의 통합/별칭 처리는 다루지 않는다.
- raw 8-bit non-UTF-8 byte stream 처리는 다루지 않는다.

## 판단 근거

- xterm control sequence 문서는 `C = % 0`을 DEC Turkish, VT500 항목으로 기록한다: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html>
- xterm 410 release source의 `charsets.dat` `map_DEC_Turkish_Supp` 테이블은 DEC Turkish Supplemental 94-character mapping을 Latin-1 기반 문자, Turkish 예외 문자, undefined/reserved 항목으로 기록한다: <https://invisible-island.net/datafiles/release/xterm.tar.gz>

## Acceptance Criteria

- [done] parser가 DEC Turkish Supplemental designation을 G0-G3에 적용한다.
- [done] DEC Turkish Supplemental replacement character mapping이 parser/state test로 검증되어 있다.
- [done] matrix와 known gap이 DEC Turkish Supplemental evidence를 반영한다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## 결과

- DEC Turkish Supplemental은 Latin-1 기반 mapping에 Turkish 예외 문자 `İ`, `ı`, `Ğ`, `ğ`, `Ş`, `ş` 등을 반영한다.
- xterm table에서 undefined/reserved인 항목은 undefined marker `␦`로 매핑한다.
- G0-G3 designation에서 `% 0` intermediate/final sequence를 DEC Turkish Supplemental로 처리한다.
