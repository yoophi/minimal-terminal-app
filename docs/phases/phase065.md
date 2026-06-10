# Phase 065 - Right-side G-set Locking Shift

## 목표

G0-G3 designation, GL locking shift, single shift, British NRCS는 구현되어 있다. 남은 charset gap 중 하나인 right-side G-set locking shift를 code-level로 지원한다.

## 범위

1. parser가 `ESC ~`, `ESC }`, `ESC |`를 각각 G1, G2, G3의 GR locking shift로 처리한다.
2. U+00A0..U+00FF printable character를 GR 영역으로 보고 high bit를 제거한 GL 문자에 active right-side charset을 적용한다.
3. G1/G2/G3 DEC Special Graphics와 British NRCS 조합을 parser/state test로 검증한다.
4. matrix와 known gap 문서를 갱신한다.

## 비범위

- UTF-8 decoder 이전의 raw 8-bit non-UTF-8 byte stream 처리는 구현하지 않는다.
- 기타 NRCS/locale-specific charset은 별도 gap으로 남긴다.

## Acceptance Criteria

- [done] parser가 `ESC ~`, `ESC }`, `ESC |` right-side locking shift를 처리한다.
- [done] GR 영역 printable mapping이 parser/state test로 검증되어 있다.
- [done] matrix와 known gap에서 right-side G-set gap이 해소되어 있다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## 결과

- `ESC ~`는 G1, `ESC }`는 G2, `ESC |`는 G3을 GR 영역에 locking shift한다.
- UTF-8로 들어온 U+00A0..U+00FF 문자는 GR printable로 처리되어 high bit를 제거한 뒤 active right-side charset mapping을 적용한다.
