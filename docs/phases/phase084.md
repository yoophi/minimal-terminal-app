# Phase 084 - DEC Supplemental Graphics Charset

## 목표

JIS Katakana 이후 남아 있는 xterm-documented 94-character charset coverage를 DEC Supplemental Graphics까지 확장한다.

## 범위

1. `Charset::DecSupplementalGraphics`를 추가한다.
2. `ESC ( % 5`, `ESC ) % 5`, `ESC * % 5`, `ESC + % 5` designation을 처리한다.
3. DEC Supplemental Graphics replacement characters를 매핑한다.
4. G0, G2 single shift, right-side G1 locking shift 경로를 parser test로 검증한다.
5. state rendering test와 compatibility 문서를 갱신한다.

## 비범위

- `ESC ( <` DEC Supplemental / UPSS ambiguity는 별도 phase로 남긴다.
- 96-character supplemental set designation `ESC - C`, `ESC . C`, `ESC / C`는 다루지 않는다.
- raw 8-bit non-UTF-8 byte stream 처리는 다루지 않는다.

## 판단 근거

- xterm control sequence 문서는 `C = % 5`를 DEC Supplemental Graphics, VT300 항목으로 기록한다: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html>
- xterm 410 release source의 `charsets.dat` `map_DEC_Supp_Graphic` 테이블은 Latin-1 GR 기본 매핑 위에 `U+0152`, `U+0178`, `U+0153`, `U+00FF` 등의 예외와 undefined 항목을 기록한다: <https://invisible-island.net/datafiles/release/xterm.tar.gz>

## Acceptance Criteria

- [done] parser가 DEC Supplemental Graphics designation을 G0-G3에 적용한다.
- [done] DEC Supplemental Graphics replacement character mapping이 parser/state test로 검증되어 있다.
- [done] matrix와 known gap이 DEC Supplemental Graphics evidence를 반영한다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## 결과

- DEC Supplemental Graphics는 `!`-`~`의 기본 Latin-1 GR 매핑을 사용한다.
- xterm table의 예외 항목 `(`, `W`, `]`, `_`, `w`, `}`는 지정된 Unicode 문자로 매핑한다.
- xterm table에서 undefined인 항목은 undefined marker `␦`로 매핑한다.
- G0-G3 designation에서 `% 5` intermediate/final sequence를 DEC Supplemental Graphics로 처리한다.
