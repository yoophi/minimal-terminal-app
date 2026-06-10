# Phase 098 - 8-bit C1 OSC/ST

## Goal

8-bit C1 `0x9D` OSC introducer와 `0x9C` ST terminator가 기존 `ESC ]` OSC와 `ESC \` ST 경로를 재사용하도록 지원한다.

## Scope

1. raw C1 `0x9D`를 `ESC ]` OSC 경로로 정규화한다.
2. raw C1 `0x9C`를 `ESC \` ST 경로로 정규화한다.
3. OSC title update와 OSC 52 query-deny 대표 경로를 parser/state/integration test로 검증한다.
4. compatibility matrix와 known gap 문서를 갱신한다.

## Non-goals

- 다른 8-bit C1 control family 전체 지원은 다루지 않는다.
- OSC 52 clipboard readback permission UI는 다루지 않는다.
- raw non-UTF-8 printable byte stream decoding 정책은 다루지 않는다.

## Reference

- xterm control sequence 문서는 OSC와 ST를 7-bit escape sequence와 8-bit C1 control 형태로 설명한다: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html>

## Completion

- [done] `parser::tests::parses_8_bit_c1_osc_with_st`가 추가되어 있다.
- [done] `state::tests::handles_8_bit_c1_osc_with_st`가 추가되어 있다.
- [done] `tests/compatibility.rs::c1_osc_sequences_have_core_evidence`가 추가되어 있다.
- [done] compatibility 문서가 8-bit C1 OSC/ST evidence를 반영한다.

## Notes

현재 구현은 raw `0x9D`, `0x9C` byte를 parser 진입 전 각각 `ESC ]`, `ESC \`로 정규화한다. OSC payload parsing과 보안 정책은 기존 OSC title/OSC 52 경로를 그대로 사용한다.
