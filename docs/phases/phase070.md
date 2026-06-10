# Phase 070 - Spanish NRCS Charset

## 목표

Italian NRCS 이후 locale-specific charset coverage를 Spanish NRCS까지 확장한다.

## 범위

1. `Charset::Spanish`를 추가한다.
2. `ESC ( Z`, `ESC ) Z`, `ESC * Z`, `ESC + Z` designation을 처리한다.
3. Spanish NRCS replacement characters를 매핑한다.
4. G0, G2 single shift, right-side G1 locking shift 경로를 parser test로 검증한다.
5. state rendering test와 compatibility 문서를 갱신한다.

## 비범위

- Spanish 외 다른 NRCS/locale-specific charset은 별도 phase로 남긴다.
- raw 8-bit non-UTF-8 byte stream 처리는 다루지 않는다.
- keyboard layout과 입력 method는 다루지 않는다.

## Acceptance Criteria

- [done] parser가 Spanish NRCS designation을 G0-G3에 적용한다.
- [done] Spanish replacement character mapping이 parser/state test로 검증되어 있다.
- [done] matrix와 known gap이 Spanish NRCS evidence를 반영한다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## 결과

- Spanish NRCS에서 `#`, `@`, `[`, `\`, `]`, `{`, `|`, `}`가 각각 `£`, `§`, `¡`, `Ñ`, `¿`, `°`, `ñ`, `ç`로 매핑된다.
