# Phase 087 - DEC Greek Supplemental Charset

## 목표

DEC Cyrillic 이후 남아 있는 xterm-documented 94-character charset coverage를 DEC Greek Supplemental까지 확장한다.

## 범위

1. `Charset::DecGreekSupplemental`을 추가한다.
2. `ESC ( " ?`, `ESC ) " ?`, `ESC * " ?`, `ESC + " ?` designation을 처리한다.
3. DEC Greek Supplemental replacement characters를 매핑한다.
4. G0, G2 single shift, right-side G1 locking shift 경로를 parser test로 검증한다.
5. state rendering test와 compatibility 문서를 갱신한다.

## 비범위

- ISO Greek supplemental 96-character set은 다루지 않는다.
- Greek NRCS와의 통합/별칭 처리는 다루지 않는다.
- raw 8-bit non-UTF-8 byte stream 처리는 다루지 않는다.

## 판단 근거

- xterm control sequence 문서는 `C = " ?`를 DEC Greek, VT500 항목으로 기록한다: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html>
- xterm 410 release source의 `charsets.dat` `map_DEC_Greek_Supp` 테이블은 DEC Greek Supplemental 94-character mapping을 Greek, Latin-1, undefined/reserved 항목으로 기록한다: <https://invisible-island.net/datafiles/release/xterm.tar.gz>

## Acceptance Criteria

- [done] parser가 DEC Greek Supplemental designation을 G0-G3에 적용한다.
- [done] DEC Greek Supplemental replacement character mapping이 parser/state test로 검증되어 있다.
- [done] matrix와 known gap이 DEC Greek Supplemental evidence를 반영한다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## 결과

- DEC Greek Supplemental의 Greek 대문자/소문자, tonos/dialytika 문자, 일부 Latin-1 보충 문자를 Unicode 문자로 처리한다.
- xterm table에서 undefined/reserved인 항목은 undefined marker `␦`로 매핑한다.
- G0-G3 designation에서 `" ?` intermediate/final sequence를 DEC Greek Supplemental로 처리한다.
