# 터미널 호환성 매트릭스

이 문서는 터미널 동작을 구현 상태와 근거 기준으로 추적한다. 범위를 의도적으로 작게 유지하고, 지원하지 않거나 확인되지 않은 동작은 숨기지 않고 명시한다.

CSI 용어와 준수 이유는 [csi.md](csi.md)를 참고한다. 호환성 판단에 사용하는 표준과 테스트 근거는 [standards-and-tests.md](standards-and-tests.md)를 참고한다. 테스트 목표, 방법, 범위는 [test-strategy.md](test-strategy.md)를 참고한다. 대표 CLI/TUI 애플리케이션 실행 보증 기준은 [app-readiness.md](app-readiness.md)를 참고한다.

## 상태 값

- `supported`: 구현되어 있으며 자동 테스트 또는 기록된 런타임 smoke 결과가 있다.
- `partially supported`: 일반적인 경우는 구현되어 있지만 알려진 빈틈이 남아 있다.
- `parsed but ignored`: parser가 sequence 종류를 인식하지만 의도적으로 terminal state를 바꾸지 않는다.
- `ignored safely`: 지원하지 않는 입력을 크래시나 state 손상 없이 무시한다.
- `not supported`: 구현되어 있지 않다.
- `unknown`: 아직 확인하지 않았다.

## ANSI/VT Sequence 매트릭스

| 분류 | Sequence / 동작 | 상태 | 근거 | 메모 |
| --- | --- | --- | --- | --- |
| 출력 가능한 텍스트 | UTF-8 printable stream | supported | `terminal_core::tests::writes_printable_text_and_tracks_cursor`, `tests/fixtures.rs::parser_golden_fixtures`, `tests/compatibility.rs::basic_terminal_sequences_have_core_evidence` | `TerminalState::append_bytes` 경로에서 `vte` byte parser를 사용한다. |
| C0 control | CR, LF, Tab, Backspace | supported | `terminal_core::tests::carriage_return_rewrites_current_line`, `terminal_core::tests::newline_moves_to_next_row`, `terminal_core::tests::backspace_removes_whole_wide_character`, `tests/compatibility.rs::basic_terminal_sequences_have_core_evidence` | 현재 grid 모델에서 Backspace는 직전 cell을 지운다. |
| CSI cursor movement | `A`, `B`, `C`, `D`, `G`, `H`, `f` | supported | `parser::tests::parses_cursor_position`, `tests/fixtures.rs::parser_golden_fixtures`, `tests/compatibility.rs::basic_terminal_sequences_have_core_evidence` | cursor 위치는 grid 경계 안으로 clamp된다. |
| CSI erase | `J`, `K` mode 0/1/2, `J` mode 3 | supported | `parser::tests::parses_erase_modes`, `terminal_core::tests::csi_clear_line_removes_stale_prompt_text`, `terminal_core::tests::csi_clear_entire_line_removes_text_on_both_sides` | `J` mode 3은 scrollback clear가 아니라 전체 화면 clear로 처리한다. |
| ESC cursor save/restore | `ESC 7`, `ESC 8` | supported | `parser::tests::parses_save_and_restore_cursor`, `terminal_core::tests::saves_and_restores_cursor` | CSI `s`/`u`도 처리한다. |
| Character set designation | `ESC ( X`, `ESC ) X`, `ESC * X`, `ESC + X` | parsed but ignored | `parser::tests::skips_charset_designation` | 현재 grid는 Unicode 문자를 직접 저장하며 alternate character set은 구현하지 않는다. |
| OSC title | `OSC ... BEL` | ignored safely | `parser::tests::skips_osc_until_bel`, `tests/fixtures.rs::parser_golden_fixtures` | window title 업데이트로 연결하지 않았다. |
| SGR attribute | reset, bold, italic, underline, inverse | supported | `parser::tests::parses_sgr_parameters`, `terminal_core::tests::stores_sgr_style_per_cell`, `tests/compatibility.rs::style_sequences_have_snapshot_evidence` | renderer는 style span을 사용한다. |
| SGR color | 8/16 color, 256 color, truecolor foreground/background | supported | `terminal_core::tests::stores_extended_sgr_colors`, `tests/fixtures.rs::sgr_golden_fixture`, `tests/compatibility.rs::style_sequences_have_snapshot_evidence` | theme 편집 UI는 범위 밖이다. |
| Alternate screen | DEC private `?47`, `?1047`, `?1049` | supported | `parser::tests::parses_alternate_screen_private_modes`, `terminal_core::tests::alternate_screen_restores_main_screen`, `tests/fixtures.rs::alternate_screen_golden_fixture` | main screen state를 저장하고 복원한다. |
| Cursor visibility | DEC private `?25 h/l` | supported | `parser::tests::parses_tui_private_modes`, `terminal_core::tests::tracks_tui_modes`, `tests/compatibility.rs::tui_private_modes_and_editing_sequences_have_core_evidence` | App renderer는 hidden cursor 상태에서 cursor block을 그리지 않는다. |
| Bracketed paste | DEC private `?2004 h/l` | supported | `parser::tests::parses_tui_private_modes`, `terminal_core::tests::tracks_tui_modes` | mode가 켜져 있으면 paste byte를 `ESC[200~`와 `ESC[201~`로 감싼다. |
| Application cursor keys | DEC private `?1 h/l` | supported | `parser::tests::parses_tui_private_modes`, `terminal_core::tests::tracks_tui_modes`, `input::tests::encodes_application_cursor_keys_for_tui_modes` | 이 mode에서는 arrow key를 `ESC O[A-D]`로 보낸다. |
| Scroll region | `CSI top;bottom r` | supported | `parser::tests::parses_tui_editing_sequences`, `terminal_core::tests::handles_scroll_region_newline`, `terminal_core::tests::handles_insert_and_delete_lines_in_scroll_region` | region 범위는 grid 높이에 맞춰 normalize된다. |
| CSI insert/delete chars | `@`, `P`, `X` | supported | `parser::tests::parses_tui_editing_sequences`, `terminal_core::tests::handles_insert_delete_and_erase_characters` | 대표 TUI redraw 동작을 검증한다. |
| CSI insert/delete lines | `L`, `M` | supported | `parser::tests::parses_tui_editing_sequences`, `terminal_core::tests::handles_insert_and_delete_lines_in_scroll_region` | active scroll region을 반영한다. |
| Device status report | `CSI 5n`, `CSI 6n` 및 관련 응답 | supported | `parser::tests::parses_device_status_reports`, `state::tests::queues_device_status_report_responses`, `state::tests::queues_cursor_position_report_responses`, `tests/compatibility.rs::device_status_reports_have_core_evidence` | `CSI 5n`은 `ESC[0n`, `CSI 6n`은 1-based cursor position report로 응답한다. |
| Mouse reporting | DEC private mouse mode와 SGR mouse event encoding | partially supported | `parser::tests::parses_tui_private_modes`, `terminal_core::tests::tracks_tui_modes`, `mouse::tests::encodes_sgr_mouse_press`, `docs/compatibility/known-gaps.md` | SGR press/release/drag/wheel encoding을 지원한다. legacy mouse encoding과 runtime app smoke는 gap으로 남긴다. |
| Cursor style | `CSI Ps SP q` 및 관련 cursor style sequence | supported | `parser::tests::parses_cursor_style_sequences`, `state::tests::tracks_cursor_style_mode`, `tests/compatibility.rs::cursor_style_sequences_have_core_evidence` | block, bar, underline을 구분한다. blinking/steady 차이는 현재 렌더링하지 않는다. |
| Full xterm compatibility | 넓은 xterm 동작 집합 | not supported | `docs/compatibility/known-gaps.md` | 단일 기능이 아니라 장기 umbrella gap으로 추적한다. |

## Runtime Behavior 매트릭스

| 동작 | 상태 | 근거 | 메모 |
| --- | --- | --- | --- |
| zsh login shell output | supported | Phase 002 runtime verification, `pty::spawn_login_shell` 구현 | macOS `forkpty`를 통해 shell을 실행한다. |
| stale text 없는 prompt redraw | supported | `terminal_core::tests::carriage_return_rewrites_current_line`, `terminal_core::tests::csi_clear_line_removes_stale_prompt_text` | 흔한 CR + clear-line prompt redraw 패턴을 처리한다. |
| Unicode wide character layout | supported | `terminal_core::tests::wide_characters_advance_cursor_by_two_cells`, `terminal_core::tests::backspace_removes_whole_wide_character` | 프로젝트 내부 width table을 사용한다. |
| Korean IME composition | supported | `composition::tests::*`, `input::tests::passes_confirmed_ime_text_as_utf8`, Phase 005 smoke notes | AppKit 변경 후에는 수동 앱 smoke 확인이 필요하다. |
| Paste | supported | `TerminalView::paste_text_from_clipboard` app paste path, `paste::tests::wraps_bracketed_paste_bytes` | pasteboard 자동 테스트는 아직 없다. |
| Bracketed paste | supported | `terminal_core::tests::tracks_tui_modes`, `paste::tests::wraps_bracketed_paste_bytes` | editor 통합은 runtime smoke로 확인해야 한다. |
| Selection and copy | supported | `selection::tests::*`, Phase 006 smoke notes | 현재 보이는 snapshot 기준이다. scrollback을 가로지르는 selection은 후속 작업이다. |
| Scrollback keyboard/wheel navigation | supported | `terminal_core::tests::scrolling_records_scrollback_length`, app scrollback offset 처리 | App-level GUI 동작은 수동 smoke로 확인한다. |
| Resize grid and PTY | supported | `terminal_core::tests::resize_preserves_visible_content_and_clamps_cursor`, app `PtyWriter::resize` path | pixel/glyph layout은 고정 폭 상수를 사용한다. |
| Alternate screen restore | supported | `terminal_core::tests::alternate_screen_restores_main_screen`, `tests/fixtures.rs::alternate_screen_golden_fixture` | TUI 종료 후 main screen이 복원되어야 한다. |

## App Smoke 매트릭스

| 대상 | 상태 | 근거 | 반복 실행 명령 |
| --- | --- | --- | --- |
| zsh prompt | supported | Phase 002와 Phase 003 runtime notes | 앱을 실행하고 prompt를 확인한다. |
| ANSI style output | supported | Phase 004 runtime note | `printf '\033[1;31mred\033[0m \033[4munder\033[0m\n'` |
| Korean input | supported | Phase 005 runtime note | `echo 한글`; `printf 'abc한글123\n'` |
| Selection/copy | supported | Phase 006 runtime note | `printf 'one\ntwo\n한글\n'` |
| `less` | partially supported | Phase 007 first compatibility expansion, `docs/compatibility/known-gaps.md` | `printf 'one\ntwo\nthree\n' &#124; less` |
| `vim` | partially supported | Phase 007 first compatibility expansion, `docs/compatibility/known-gaps.md` | `vim /tmp/minimal-terminal-smoke.txt` |
| `top` | partially supported | Phase 007 smoke target, `docs/compatibility/known-gaps.md` | `top` |
| `htop` | unknown | `docs/compatibility/known-gaps.md` | 로컬 설치 여부에 따라 달라진다. |
| `fzf` | unknown | `docs/compatibility/known-gaps.md` | 로컬 설치 여부에 따라 달라진다. |
| `git log --oneline --graph --decorate` | unknown | `docs/compatibility/known-gaps.md` | history가 있는 repository에서 확인해야 한다. |
