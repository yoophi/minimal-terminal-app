# Phase 069 - Italian NRCS Charset

## 목표

French NRCS 이후 locale-specific charset coverage를 Italian NRCS까지 확장한다.

## 범위

1. `Charset::Italian`을 추가한다.
2. `ESC ( Y`, `ESC ) Y`, `ESC * Y`, `ESC + Y` designation을 처리한다.
3. Italian NRCS replacement characters를 매핑한다.
4. G0, G2 single shift, right-side G1 locking shift 경로를 parser test로 검증한다.
5. state rendering test와 compatibility 문서를 갱신한다.

## 비범위

- Italian 외 다른 NRCS/locale-specific charset은 별도 phase로 남긴다.
- raw 8-bit non-UTF-8 byte stream 처리는 다루지 않는다.
- keyboard layout과 입력 method는 다루지 않는다.

## Acceptance Criteria

- [done] parser가 Italian NRCS designation을 G0-G3에 적용한다.
- [done] Italian replacement character mapping이 parser/state test로 검증되어 있다.
- [done] matrix와 known gap이 Italian NRCS evidence를 반영한다.
- [done] `scripts/run-compatibility-core.sh`, `cargo test`, app smoke, command smoke, target smoke가 통과한다.

## 결과

- Italian NRCS에서 `#`, `@`, `[`, `\`, `]`, `` ` ``, `{`, `|`, `}`, `~`가 각각 `£`, `§`, `°`, `ç`, `é`, `ù`, `à`, `ò`, `è`, `ì`로 매핑된다.
