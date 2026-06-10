# Phase 082 - JIS Roman Charset

## 목표

SCS NRCS 이후 xterm-documented 94-character charset coverage를 JIS Roman까지 확장한다.

## 범위

1. `Charset::JisRoman`을 추가한다.
2. `ESC ( J`, `ESC ) J`, `ESC * J`, `ESC + J` designation을 처리한다.
3. JIS Roman replacement characters를 매핑한다.
4. G0, G2 single shift, right-side G1 locking shift 경로를 parser test로 검증한다.
5. state rendering test와 compatibility 문서를 갱신한다.

## 비범위

- JIS Katakana와 다른 Japanese charset은 별도 phase로 남긴다.
- raw 8-bit non-UTF-8 byte stream 처리는 다루지 않는다.
- keyboard layout과 입력 method는 다루지 않는다.

## 판단 근거

- xterm control sequence 문서는 `C = J`를 JIS-Roman, VT382 항목으로 기록한다: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html>
- xterm 410 release source의 `charsets.dat` `map_JIS_Roman` 테이블은 `\`를 `¥`, `~`를 `‾`로 매핑한다: <https://invisible-island.net/datafiles/release/xterm.tar.gz>

## Acceptance Criteria

- [done] parser가 JIS Roman designation을 G0-G3에 적용한다.
- [done] JIS Roman replacement character mapping이 parser/state test로 검증되어 있다.
- [done] matrix와 known gap이 JIS Roman evidence를 반영한다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## 결과

- JIS Roman에서 `\`, `~`가 각각 `¥`, `‾`로 매핑된다.
- G0-G3 designation에서 `J` final character를 JIS Roman으로 처리한다.
