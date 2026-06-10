# Phase 080 - Russian NRCS Charset

## 목표

Turkish NRCS 이후 xterm source-backed VT500 charset coverage를 Russian NRCS까지 확장한다.

## 범위

1. `Charset::Russian`을 추가한다.
2. `ESC ( & 5`, `ESC ) & 5`, `ESC * & 5`, `ESC + & 5` designation을 처리한다.
3. xterm source의 Russian NRCS replacement characters를 매핑한다.
4. G0, G2 single shift, right-side G1 locking shift 경로를 parser test로 검증한다.
5. state rendering test와 compatibility 문서를 갱신한다.

## 비범위

- Russian 외 다른 VT500 NRCS/locale-specific charset은 별도 phase로 남긴다.
- DEC Cyrillic supplemental 94-character set과 ISO Latin-Cyrillic supplemental 96-character set은 다루지 않는다.
- xterm 문서가 mapping documentation을 찾지 못했다고 명시한 항목이므로, 이 phase는 xterm 410 source-compatible mapping으로 제한한다.
- raw 8-bit non-UTF-8 byte stream 처리는 다루지 않는다.
- keyboard layout과 입력 method는 다루지 않는다.

## 판단 근거

- xterm control sequence 문서는 `C = & 5`를 DEC Russian, VT500 항목으로 기록하며 mapping documentation은 찾지 못했다고 설명한다: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html>
- xterm 410 release source의 `charsets.dat` `map_NRCS_Russian` 테이블은 KOI-7 기반으로 `` ` ``-`~` 일부를 Cyrillic capital letters로 매핑한다: <https://invisible-island.net/datafiles/release/xterm.tar.gz>

## Acceptance Criteria

- [done] parser가 Russian NRCS designation을 G0-G3에 적용한다.
- [done] Russian replacement character mapping이 parser/state test로 검증되어 있다.
- [done] matrix와 known gap이 Russian NRCS evidence를 반영한다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## 결과

- Russian NRCS에서 `` ` ``-`z`, `{`, `|`, `}`, `~`가 xterm 410 `map_NRCS_Russian` 테이블의 Cyrillic capital letters로 매핑된다.
- G0-G3 designation에서 `& 5` intermediate/final 조합을 Russian NRCS로 처리한다.
