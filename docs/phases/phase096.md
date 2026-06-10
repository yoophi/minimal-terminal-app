# Phase 096 - 8-bit C1 CSI Introducer Evidence

## Goal

8-bit C1 `0x9B` CSI introducer가 `ESC [` CSI와 같은 core parser/state 경로로 처리된다는 evidence를 고정한다.

## Scope

1. parser test에서 `0x9B` cursor movement, SGR, DSR dispatch를 검증한다.
2. state test에서 `0x9B` CSI가 grid mutation과 response queue에 반영되는지 검증한다.
3. compatibility integration test에 대표 evidence를 추가한다.
4. CSI 문서, compatibility matrix, known gap 문서를 갱신한다.

## Non-goals

- 다른 8-bit C1 control family 전체 지원은 다루지 않는다.
- raw non-UTF-8 printable byte stream decoding 정책은 다루지 않는다.
- xterm 전체 control sequence coverage 완료를 선언하지 않는다.

## Reference

- xterm control sequence 문서는 CSI를 7-bit `ESC [`와 8-bit `0x9B` 형태로 설명한다: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html>

## Completion

- [done] `parser::tests::parses_8_bit_c1_csi_sequences`가 추가되어 있다.
- [done] `state::tests::handles_8_bit_c1_csi_sequences`가 추가되어 있다.
- [done] `tests/compatibility.rs::c1_csi_sequences_have_core_evidence`가 추가되어 있다.
- [done] compatibility 문서가 8-bit C1 CSI evidence를 반영한다.

## Notes

현재 구현은 `vte::Parser`를 통해 `0x9B` CSI introducer를 기존 CSI dispatch로 전달한다. 이 phase는 새 parser branch를 추가하기보다 이미 동작하는 경로를 자동 테스트와 문서 evidence로 고정한다.
