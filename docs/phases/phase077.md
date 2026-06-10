# Phase 077 - Greek NRCS Charset

## 목표

Portuguese NRCS 이후 VT500 계열 locale-specific charset coverage를 Greek NRCS까지 확장한다.

## 범위

1. `Charset::Greek`을 추가한다.
2. `ESC ( " >`, `ESC ) " >`, `ESC * " >`, `ESC + " >` designation을 처리한다.
3. Greek NRCS replacement characters를 매핑한다.
4. G0, G2 single shift, right-side G1 locking shift 경로를 parser test로 검증한다.
5. state rendering test와 compatibility 문서를 갱신한다.

## 비범위

- Greek 외 다른 VT500 NRCS/locale-specific charset은 별도 phase로 남긴다.
- DEC Greek supplemental 94-character set과 ISO Greek supplemental 96-character set은 다루지 않는다.
- raw 8-bit non-UTF-8 byte stream 처리는 다루지 않는다.
- keyboard layout과 입력 method는 다루지 않는다.

## 판단 근거

- xterm control sequence 문서는 Greek NRCS designation을 `C = " >`로 기록한다: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html>
- xterm 410 release source의 `charsets.dat` `map_NRCS_Greek` 테이블은 `a`-`x`를 Greek 대문자 `Α`-`Ω` 계열로 매핑하고 `y`, `z`를 undefined로 표시한다: <https://invisible-island.net/datafiles/release/xterm.tar.gz>

## Acceptance Criteria

- [done] parser가 Greek NRCS designation을 G0-G3에 적용한다.
- [done] Greek replacement character mapping이 parser/state test로 검증되어 있다.
- [done] matrix와 known gap이 Greek NRCS evidence를 반영한다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## 결과

- Greek NRCS에서 `a`-`x`가 `Α`, `Β`, `Γ`, `Δ`, `Ε`, `Ζ`, `Η`, `Θ`, `Ι`, `Κ`, `Λ`, `Μ`, `Ν`, `Χ`, `Ο`, `Π`, `Ρ`, `Σ`, `Τ`, `Υ`, `Φ`, `Ξ`, `Ψ`, `Ω`로 매핑된다.
- xterm table에서 undefined인 `y`, `z`는 undefined marker `␦`로 매핑한다.
- G0-G3 designation에서 `" >` intermediate/final 조합을 Greek NRCS로 처리한다.
