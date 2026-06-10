# Phase 092 - ISO Greek Supplemental 96-Character Charset

## 목표

ISO Latin-2 Supplemental 이후 xterm-documented 96-character supplemental charset coverage를 ISO Greek Supplemental까지 확장한다.

## 범위

1. `Charset::IsoGreekSupplemental`을 추가한다.
2. `ESC - F`, `ESC . F`, `ESC / F` designation을 각각 G1, G2, G3에 적용한다.
3. xterm `map_ISO_Greek_Supp`의 96개 mapping을 Unicode 문자 테이블로 반영한다.
4. parser test, state rendering test, compatibility evidence test를 추가한다.
5. compatibility matrix와 known gap 문서를 갱신한다.

## 비범위

- ISO Hebrew, ISO Latin-Cyrillic, ISO Latin-5 mapping은 별도 phase로 남긴다.
- raw 8-bit non-UTF-8 byte stream 처리는 다루지 않는다.
- DCS DECAUPSS user-preferred supplemental set assignment는 다루지 않는다.

## 판단 근거

- xterm control sequence 문서는 96-character G1-G3 designation에서 `C = F`를 ISO Greek Supplemental, VT500 항목으로 기록한다: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html>
- xterm 410 release source의 `charsets.dat` `map_ISO_Greek_Supp` 테이블은 ISO Greek Supplemental 96-character mapping을 Unicode code point와 undefined 항목으로 기록한다: <https://invisible-island.net/datafiles/release/xterm.tar.gz>

## Acceptance Criteria

- [done] parser가 ISO Greek Supplemental designation을 G1-G3에 적용한다.
- [done] ISO Greek Supplemental 96-character mapping이 parser/state test로 검증되어 있다.
- [done] compatibility matrix와 known gap이 ISO Greek Supplemental evidence를 반영한다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## 결과

- ISO Greek Supplemental은 `0x20..0x7f` GL code를 xterm `map_ISO_Greek_Supp` 기반 Unicode 문자로 매핑한다.
- xterm table에서 undefined인 항목은 undefined marker `␦`로 매핑한다.
- `ESC - F`, `ESC . F`, `ESC / F`는 각각 G1, G2, G3 96-character designation으로 처리한다.
- 실제 출력 검증은 `ESC ~`, `ESC }`, `ESC |` right-side locking shift와 UTF-8 GR 입력을 통해 수행한다.
