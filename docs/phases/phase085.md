# Phase 085 - DEC Technical Charset

## 목표

DEC Supplemental Graphics 이후 남아 있는 xterm-documented 94-character charset coverage를 DEC Technical까지 확장한다.

## 범위

1. `Charset::DecTechnical`을 추가한다.
2. `ESC ( >`, `ESC ) >`, `ESC * >`, `ESC + >` designation을 처리한다.
3. DEC Technical replacement characters를 매핑한다.
4. G0, G2 single shift, right-side G1 locking shift 경로를 parser test로 검증한다.
5. state rendering test와 compatibility 문서를 갱신한다.

## 비범위

- xterm private-use sigma 조각 glyph는 범용 Unicode glyph가 없으므로 undefined marker로 처리한다.
- raw 8-bit non-UTF-8 byte stream 처리는 다루지 않는다.
- 96-character supplemental set designation `ESC - C`, `ESC . C`, `ESC / C`는 다루지 않는다.

## 판단 근거

- xterm control sequence 문서는 `C = >`를 DEC Technical, VT300 항목으로 기록한다: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html>
- xterm 410 release source의 `charsets.dat` `map_DEC_Technical` 테이블은 DEC Technical 94-character mapping을 Unicode 코드포인트와 undefined/PUA 항목으로 기록한다: <https://invisible-island.net/datafiles/release/xterm.tar.gz>

## Acceptance Criteria

- [done] parser가 DEC Technical designation을 G0-G3에 적용한다.
- [done] DEC Technical replacement character mapping이 parser/state test로 검증되어 있다.
- [done] matrix와 known gap이 DEC Technical evidence를 반영한다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## 결과

- DEC Technical의 수학 기호, 괄호 조각, Greek 문자, 화살표 mapping을 Unicode 문자로 처리한다.
- xterm table에서 PUA 또는 undefined인 항목은 undefined marker `␦`로 매핑한다.
- G0-G3 designation에서 `>` final character를 DEC Technical로 처리한다.
