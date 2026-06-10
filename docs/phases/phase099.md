# Phase 099 - DEC Supplemental UPSS Alias

## Goal

`ESC ( <`, `ESC ) <`, `ESC * <`, `ESC + <` designation을 DEC Supplemental Graphics alias로 처리해 기존 DEC Supplemental mapping을 재사용한다.

## Scope

1. G0-G3 charset designation에서 final byte `<`를 `Charset::DecSupplementalGraphics`로 연결한다.
2. parser/state test로 G0, G1 right-side shift, G2 single shift, G3 right-side shift 경로를 검증한다.
3. compatibility integration test와 문서 evidence를 갱신한다.

## Non-goals

- DCS DECAUPSS user-preferred supplemental set assignment는 다루지 않는다.
- DEC Supplemental Graphics mapping 자체를 변경하지 않는다.
- raw 8-bit non-UTF-8 printable byte stream decoding 정책은 다루지 않는다.

## Reference

- xterm source에는 DEC UPSS가 user preferred supplemental set으로 별도 처리되며, 기본 supplemental mapping과 연결되는 경로가 있다: <https://invisible-island.net/datafiles/release/xterm.tar.gz>
- 이 프로젝트는 현재 DCS DECAUPSS assignment를 구현하지 않으므로 `<` designation을 DEC Supplemental Graphics alias로 고정한다.

## Completion

- [done] `parser::tests::maps_dec_supplemental_upss_alias_charset`가 추가되어 있다.
- [done] `state::tests::renders_dec_supplemental_upss_alias_charset`가 추가되어 있다.
- [done] `tests/compatibility.rs::dec_supplemental_upss_alias_has_core_evidence`가 추가되어 있다.
- [done] compatibility matrix와 known gap이 DEC Supplemental UPSS alias evidence를 반영한다.

## Notes

이 phase는 `ESC ( <` ambiguity를 제거하지만, runtime에서 UPSS를 다른 supplemental set으로 재지정하는 기능은 제공하지 않는다. 그 기능은 별도 DCS/UPSS phase로 분리한다.
