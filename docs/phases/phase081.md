# Phase 081 - SCS NRCS Charset

## 목표

Russian NRCS 이후 xterm source-backed VT500 charset coverage를 SCS NRCS까지 확장한다.

## 범위

1. `Charset::SerboCroatian`을 추가한다.
2. `ESC ( % 3`, `ESC ) % 3`, `ESC * % 3`, `ESC + % 3` designation을 처리한다.
3. xterm source의 SCS/Serbo-Croatian replacement characters를 매핑한다.
4. G0, G2 single shift, right-side G1 locking shift 경로를 parser test로 검증한다.
5. state rendering test와 compatibility 문서를 갱신한다.

## 비범위

- SCS 외 다른 VT500 NRCS/locale-specific charset은 별도 phase로 남긴다.
- xterm 문서가 mapping documentation을 찾지 못했다고 명시한 항목이므로, 이 phase는 xterm 410 source-compatible mapping으로 제한한다.
- raw 8-bit non-UTF-8 byte stream 처리는 다루지 않는다.
- keyboard layout과 입력 method는 다루지 않는다.

## 판단 근거

- xterm control sequence 문서는 `C = % 3`을 SCS NRCS, VT500 항목으로 기록하며 mapping documentation은 찾지 못했다고 설명한다: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html>
- xterm 410 release source의 `charsets.dat` `map_NRCS_Serbo_Croatian` 테이블은 `@`, `[`, `\`, `]`, `^`, `` ` ``, `{`, `|`, `}`, `~`를 Serbo-Croatian replacement characters로 매핑한다: <https://invisible-island.net/datafiles/release/xterm.tar.gz>

## Acceptance Criteria

- [done] parser가 SCS NRCS designation을 G0-G3에 적용한다.
- [done] SCS replacement character mapping이 parser/state test로 검증되어 있다.
- [done] matrix와 known gap이 SCS NRCS evidence를 반영한다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## 결과

- SCS NRCS에서 `@`, `[`, `\`, `]`, `^`, `` ` ``, `{`, `|`, `}`, `~`가 각각 `Ž`, `Š`, `Đ`, `Ć`, `Č`, `ž`, `š`, `đ`, `ć`, `č`로 매핑된다.
- G0-G3 designation에서 `% 3` intermediate/final 조합을 SCS NRCS로 처리한다.
