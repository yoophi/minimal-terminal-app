# Phase 097 - 8-bit C1 SS2/SS3 Single Shift

## Goal

8-bit C1 `0x8E` SS2와 `0x8F` SS3가 기존 `ESC N`, `ESC O` single shift와 같은 G2/G3 charset mapping을 사용하도록 지원한다.

## Scope

1. raw C1 `0x8E`를 `ESC N` SS2 경로로 정규화한다.
2. raw C1 `0x8F`를 `ESC O` SS3 경로로 정규화한다.
3. DEC Special Graphics G2/G3 single shift parser/state test를 추가한다.
4. compatibility integration test와 문서 evidence를 갱신한다.

## Non-goals

- 다른 8-bit C1 control family 전체 지원은 다루지 않는다.
- raw non-UTF-8 printable byte stream decoding 정책은 다루지 않는다.
- G-set designation coverage 전체 완료를 선언하지 않는다.

## Reference

- xterm control sequence 문서는 SS2/SS3를 7-bit escape sequence와 8-bit C1 control 형태로 설명한다: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html>

## Completion

- [done] `parser::tests::maps_8_bit_c1_ss2_and_ss3_single_shift`가 추가되어 있다.
- [done] `state::tests::renders_8_bit_c1_ss2_and_ss3_single_shift`가 추가되어 있다.
- [done] `tests/compatibility.rs::c1_single_shift_charset_sequences_have_core_evidence`가 추가되어 있다.
- [done] compatibility 문서가 8-bit C1 SS2/SS3 evidence를 반영한다.

## Notes

현재 구현은 raw `0x8E`, `0x8F` byte를 parser 진입 전 각각 `ESC N`, `ESC O`로 정규화한다. 실제 charset 선택과 printable mapping은 기존 single shift 경로를 재사용한다.
