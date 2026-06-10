# Phase 078 - Hebrew NRCS Charset

## 목표

Greek NRCS 이후 VT500 계열 locale-specific charset coverage를 Hebrew NRCS까지 확장한다.

## 범위

1. `Charset::Hebrew`를 추가한다.
2. `ESC ( % =`, `ESC ) % =`, `ESC * % =`, `ESC + % =` designation을 처리한다.
3. Hebrew NRCS replacement characters를 매핑한다.
4. G0, G2 single shift, right-side G1 locking shift 경로를 parser test로 검증한다.
5. state rendering test와 compatibility 문서를 갱신한다.

## 비범위

- Hebrew 외 다른 VT500 NRCS/locale-specific charset은 별도 phase로 남긴다.
- DEC Hebrew supplemental 94-character set과 ISO Hebrew supplemental 96-character set은 다루지 않는다.
- bidirectional text reordering은 다루지 않고 logical order rendering만 검증한다.
- raw 8-bit non-UTF-8 byte stream 처리는 다루지 않는다.
- keyboard layout과 입력 method는 다루지 않는다.

## 판단 근거

- xterm control sequence 문서는 Hebrew NRCS designation을 `C = % =`로 기록한다: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html>
- xterm 410 release source의 `charsets.dat` `map_NRCS_Hebrew` 테이블은 `` ` ``-`z`를 Hebrew letters `א`-`ת` 계열로 매핑한다: <https://invisible-island.net/datafiles/release/xterm.tar.gz>

## Acceptance Criteria

- [done] parser가 Hebrew NRCS designation을 G0-G3에 적용한다.
- [done] Hebrew replacement character mapping이 parser/state test로 검증되어 있다.
- [done] matrix와 known gap이 Hebrew NRCS evidence를 반영한다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## 결과

- Hebrew NRCS에서 `` ` ``-`z`가 `א`, `ב`, `ג`, `ד`, `ה`, `ו`, `ז`, `ח`, `ט`, `י`, `ך`, `כ`, `ל`, `ם`, `מ`, `ן`, `נ`, `ס`, `ע`, `ף`, `פ`, `ץ`, `צ`, `ק`, `ר`, `ש`, `ת`로 매핑된다.
- G0-G3 designation에서 `% =` intermediate/final 조합을 Hebrew NRCS로 처리한다.
