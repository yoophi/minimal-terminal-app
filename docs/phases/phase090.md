# Phase 090 - ISO Latin-1 Supplemental 96-Character Charset

## 목표

DEC Turkish Supplemental 이후 남아 있는 96-character supplemental charset coverage를 ISO Latin-1 Supplemental부터 확장한다.

## 범위

1. `Charset::IsoLatin1Supplemental`을 추가한다.
2. `ESC - A`, `ESC . A`, `ESC / A` designation을 각각 G1, G2, G3에 적용한다.
3. `ESC ~`, `ESC }`, `ESC |` right-side locking shift로 GR 영역 문자를 렌더링한다.
4. parser test, state rendering test, compatibility evidence test를 추가한다.
5. compatibility matrix와 known gap 문서를 갱신한다.

## 비범위

- G0에 96-character set을 지정하는 동작은 다루지 않는다.
- ISO Latin-2, ISO Latin-5, ISO Greek, ISO Hebrew, ISO Latin-Cyrillic mapping은 별도 phase로 남긴다.
- raw 8-bit non-UTF-8 byte stream 처리는 다루지 않는다.

## 판단 근거

- xterm control sequence 문서는 `ESC - C`, `ESC . C`, `ESC / C` 형식의 G1-G3 96-character set designation을 별도 범주로 다룬다: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html>
- xterm 410 release source의 `charsets.dat` `map_ISO_Latin_1` 항목은 ISO Latin-1 Supplemental을 96-character codepage로 기록한다: <https://invisible-island.net/datafiles/release/xterm.tar.gz>

## Acceptance Criteria

- [done] parser가 ISO Latin-1 Supplemental designation을 G1-G3에 적용한다.
- [done] right-side G-set locking shift 경로에서 ISO Latin-1 Supplemental GR 문자가 parser/state test로 검증되어 있다.
- [done] compatibility matrix와 known gap이 ISO Latin-1 Supplemental evidence를 반영한다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## 결과

- ISO Latin-1 Supplemental은 `0x20..0x7f` GL code를 `U+00A0..U+00FF` Unicode 문자로 매핑한다.
- `ESC - A`, `ESC . A`, `ESC / A`는 각각 G1, G2, G3 96-character designation으로 처리한다.
- 실제 출력 검증은 `ESC ~`, `ESC }`, `ESC |` right-side locking shift와 UTF-8 GR 입력을 통해 수행한다.
