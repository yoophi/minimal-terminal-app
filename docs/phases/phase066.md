# Phase 066 - German NRCS Charset

## 목표

British NRCS 이후 남아 있는 locale-specific charset gap을 줄이기 위해 German NRCS를 추가한다.

## 범위

1. `Charset::German`을 추가한다.
2. `ESC ( K`, `ESC ) K`, `ESC * K`, `ESC + K` designation을 처리한다.
3. German NRCS replacement characters를 매핑한다.
4. G0, G2 single shift, right-side G1 locking shift 경로를 parser test로 검증한다.
5. state rendering test와 compatibility 문서를 갱신한다.

## 비범위

- German 외 다른 NRCS/locale-specific charset은 별도 phase로 남긴다.
- keyboard layout과 입력 method는 다루지 않는다.

## Acceptance Criteria

- [done] parser가 German NRCS designation을 G0-G3에 적용한다.
- [done] German replacement character mapping이 parser/state test로 검증되어 있다.
- [done] matrix와 known gap이 German NRCS evidence를 반영한다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## 결과

- German NRCS에서 `@`, `[`, `\`, `]`, `{`, `|`, `}`, `~`가 각각 `§`, `Ä`, `Ö`, `Ü`, `ä`, `ö`, `ü`, `ß`로 매핑된다.
