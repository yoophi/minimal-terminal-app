use objc2_app_kit::{NSEvent, NSEventModifierFlags};

const KEY_RETURN: u16 = 36;
const KEY_KEYPAD_ENTER: u16 = 76;
const KEY_BACKSPACE: u16 = 51;
const KEY_FORWARD_DELETE: u16 = 117;
const KEY_F1: u16 = 122;
const KEY_F2: u16 = 120;
const KEY_F3: u16 = 99;
const KEY_F4: u16 = 118;
const KEY_F5: u16 = 96;
const KEY_F6: u16 = 97;
const KEY_F7: u16 = 98;
const KEY_F8: u16 = 100;
const KEY_F9: u16 = 101;
const KEY_F10: u16 = 109;
const KEY_F11: u16 = 103;
const KEY_F12: u16 = 111;
const KEY_HOME: u16 = 115;
const KEY_END: u16 = 119;
const KEY_PAGE_UP: u16 = 116;
const KEY_PAGE_DOWN: u16 = 121;
const KEY_LEFT: u16 = 123;
const KEY_RIGHT: u16 = 124;
const KEY_DOWN: u16 = 125;
const KEY_UP: u16 = 126;

pub fn encode_key_event(event: &NSEvent, input: &str) -> Option<Vec<u8>> {
    encode_key(event.keyCode(), event.modifierFlags(), input)
}

pub fn encode_application_cursor_key_event(event: &NSEvent) -> Option<Vec<u8>> {
    if modifier_parameter(event.modifierFlags()).is_some() {
        return None;
    }

    encode_application_cursor_key(event.keyCode())
}

fn encode_application_cursor_key(key_code: u16) -> Option<Vec<u8>> {
    match key_code {
        KEY_UP => Some(b"\x1bOA".to_vec()),
        KEY_DOWN => Some(b"\x1bOB".to_vec()),
        KEY_RIGHT => Some(b"\x1bOC".to_vec()),
        KEY_LEFT => Some(b"\x1bOD".to_vec()),
        _ => None,
    }
}

fn encode_key(key_code: u16, flags: NSEventModifierFlags, input: &str) -> Option<Vec<u8>> {
    if flags.contains(NSEventModifierFlags::Command) {
        return None;
    }

    if let Some(bytes) = encode_control_text(input) {
        return Some(bytes);
    }

    if let Some(bytes) = encode_control_key(key_code, flags) {
        return Some(bytes);
    }

    if is_option_word_navigation(key_code, flags) {
        return encode_option_key(key_code, input);
    }

    if let Some(bytes) = encode_modified_special_key(key_code, flags) {
        return Some(bytes);
    }

    if let Some(bytes) = encode_function_key(key_code, None) {
        return Some(bytes);
    }

    if flags.contains(NSEventModifierFlags::Option) {
        return encode_option_key(key_code, input);
    }

    match key_code {
        KEY_RETURN | KEY_KEYPAD_ENTER => Some(b"\r".to_vec()),
        KEY_BACKSPACE => Some(vec![0x7f]),
        KEY_FORWARD_DELETE => Some(b"\x1b[3~".to_vec()),
        KEY_HOME => Some(vec![0x01]),
        KEY_END => Some(vec![0x05]),
        KEY_PAGE_UP => Some(b"\x1b[5~".to_vec()),
        KEY_PAGE_DOWN => Some(b"\x1b[6~".to_vec()),
        KEY_UP => Some(vec![0x10]),
        KEY_DOWN => Some(vec![0x0e]),
        KEY_RIGHT => Some(vec![0x06]),
        KEY_LEFT => Some(vec![0x02]),
        _ if input.is_empty() => None,
        _ => Some(input.as_bytes().to_vec()),
    }
}

fn encode_option_key(key_code: u16, input: &str) -> Option<Vec<u8>> {
    match key_code {
        KEY_LEFT => Some(b"\x1bb".to_vec()),
        KEY_RIGHT => Some(b"\x1bf".to_vec()),
        KEY_BACKSPACE => Some(vec![0x1b, 0x7f]),
        _ if input.is_empty() => None,
        _ => Some(input.as_bytes().to_vec()),
    }
}

fn is_option_word_navigation(key_code: u16, flags: NSEventModifierFlags) -> bool {
    flags.contains(NSEventModifierFlags::Option)
        && !flags.intersects(NSEventModifierFlags::Shift | NSEventModifierFlags::Control)
        && matches!(key_code, KEY_LEFT | KEY_RIGHT | KEY_BACKSPACE)
}

fn encode_control_text(input: &str) -> Option<Vec<u8>> {
    match input {
        "\u{3}" => Some(vec![0x03]),
        "\u{4}" => Some(vec![0x04]),
        _ => None,
    }
}

fn encode_control_key(key_code: u16, flags: NSEventModifierFlags) -> Option<Vec<u8>> {
    if !flags.contains(NSEventModifierFlags::Control) {
        return None;
    }

    match key_code {
        8 => Some(vec![0x03]),
        2 => Some(vec![0x04]),
        _ => None,
    }
}

fn encode_modified_special_key(key_code: u16, flags: NSEventModifierFlags) -> Option<Vec<u8>> {
    let modifier = modifier_parameter(flags)?;

    match key_code {
        KEY_UP => Some(format!("\x1b[1;{modifier}A").into_bytes()),
        KEY_DOWN => Some(format!("\x1b[1;{modifier}B").into_bytes()),
        KEY_RIGHT => Some(format!("\x1b[1;{modifier}C").into_bytes()),
        KEY_LEFT => Some(format!("\x1b[1;{modifier}D").into_bytes()),
        KEY_HOME => Some(format!("\x1b[1;{modifier}H").into_bytes()),
        KEY_END => Some(format!("\x1b[1;{modifier}F").into_bytes()),
        KEY_FORWARD_DELETE => Some(format!("\x1b[3;{modifier}~").into_bytes()),
        KEY_PAGE_UP => Some(format!("\x1b[5;{modifier}~").into_bytes()),
        KEY_PAGE_DOWN => Some(format!("\x1b[6;{modifier}~").into_bytes()),
        _ => encode_function_key(key_code, Some(modifier)),
    }
}

fn modifier_parameter(flags: NSEventModifierFlags) -> Option<u8> {
    let mut value = 1;
    if flags.contains(NSEventModifierFlags::Shift) {
        value += 1;
    }
    if flags.contains(NSEventModifierFlags::Option) {
        value += 2;
    }
    if flags.contains(NSEventModifierFlags::Control) {
        value += 4;
    }

    (value > 1).then_some(value)
}

fn encode_function_key(key_code: u16, modifier: Option<u8>) -> Option<Vec<u8>> {
    match (key_code, modifier) {
        (KEY_F1, None) => Some(b"\x1bOP".to_vec()),
        (KEY_F2, None) => Some(b"\x1bOQ".to_vec()),
        (KEY_F3, None) => Some(b"\x1bOR".to_vec()),
        (KEY_F4, None) => Some(b"\x1bOS".to_vec()),
        (KEY_F1, Some(modifier)) => Some(format!("\x1b[1;{modifier}P").into_bytes()),
        (KEY_F2, Some(modifier)) => Some(format!("\x1b[1;{modifier}Q").into_bytes()),
        (KEY_F3, Some(modifier)) => Some(format!("\x1b[1;{modifier}R").into_bytes()),
        (KEY_F4, Some(modifier)) => Some(format!("\x1b[1;{modifier}S").into_bytes()),
        (KEY_F5, modifier) => encode_tilde_function_key(15, modifier),
        (KEY_F6, modifier) => encode_tilde_function_key(17, modifier),
        (KEY_F7, modifier) => encode_tilde_function_key(18, modifier),
        (KEY_F8, modifier) => encode_tilde_function_key(19, modifier),
        (KEY_F9, modifier) => encode_tilde_function_key(20, modifier),
        (KEY_F10, modifier) => encode_tilde_function_key(21, modifier),
        (KEY_F11, modifier) => encode_tilde_function_key(23, modifier),
        (KEY_F12, modifier) => encode_tilde_function_key(24, modifier),
        _ => None,
    }
}

fn encode_tilde_function_key(code: u8, modifier: Option<u8>) -> Option<Vec<u8>> {
    Some(match modifier {
        Some(modifier) => format!("\x1b[{code};{modifier}~").into_bytes(),
        None => format!("\x1b[{code}~").into_bytes(),
    })
}

#[cfg(test)]
mod tests {
    use objc2_app_kit::NSEventModifierFlags;

    const KEY_RETURN: u16 = 36;
    const KEY_KEYPAD_ENTER: u16 = 76;
    const KEY_BACKSPACE: u16 = 51;
    const KEY_FORWARD_DELETE: u16 = 117;
    const KEY_F1: u16 = 122;
    const KEY_F2: u16 = 120;
    const KEY_F5: u16 = 96;
    const KEY_F12: u16 = 111;
    const KEY_HOME: u16 = 115;
    const KEY_END: u16 = 119;
    const KEY_PAGE_UP: u16 = 116;
    const KEY_PAGE_DOWN: u16 = 121;
    const KEY_LEFT: u16 = 123;
    const KEY_RIGHT: u16 = 124;
    const KEY_DOWN: u16 = 125;
    const KEY_UP: u16 = 126;

    fn encode_key_code(key_code: u16, input: &str) -> Option<Vec<u8>> {
        super::encode_key(key_code, NSEventModifierFlags::empty(), input)
    }

    fn encode_modified_key(
        key_code: u16,
        flags: NSEventModifierFlags,
        input: &str,
    ) -> Option<Vec<u8>> {
        super::encode_key(key_code, flags, input)
    }

    #[test]
    fn encodes_return_as_carriage_return() {
        assert_eq!(encode_key_code(KEY_RETURN, ""), Some(b"\r".to_vec()));
        assert_eq!(encode_key_code(KEY_KEYPAD_ENTER, ""), Some(b"\r".to_vec()));
    }

    #[test]
    fn encodes_backspace_as_delete_byte() {
        assert_eq!(encode_key_code(KEY_BACKSPACE, ""), Some(vec![0x7f]));
    }

    #[test]
    fn encodes_arrow_keys_as_shell_line_editor_controls() {
        assert_eq!(encode_key_code(KEY_UP, ""), Some(vec![0x10]));
        assert_eq!(encode_key_code(KEY_DOWN, ""), Some(vec![0x0e]));
        assert_eq!(encode_key_code(KEY_RIGHT, ""), Some(vec![0x06]));
        assert_eq!(encode_key_code(KEY_LEFT, ""), Some(vec![0x02]));
    }

    #[test]
    fn encodes_control_text() {
        assert_eq!(encode_key_code(0, "\u{3}"), Some(vec![0x03]));
        assert_eq!(encode_key_code(0, "\u{4}"), Some(vec![0x04]));
    }

    #[test]
    fn encodes_control_key_combinations() {
        assert_eq!(
            encode_modified_key(8, NSEventModifierFlags::Control, ""),
            Some(vec![0x03])
        );
        assert_eq!(
            encode_modified_key(2, NSEventModifierFlags::Control, ""),
            Some(vec![0x04])
        );
    }

    #[test]
    fn encodes_navigation_keys() {
        assert_eq!(
            encode_key_code(KEY_FORWARD_DELETE, ""),
            Some(b"\x1b[3~".to_vec())
        );
        assert_eq!(encode_key_code(KEY_HOME, ""), Some(vec![0x01]));
        assert_eq!(encode_key_code(KEY_END, ""), Some(vec![0x05]));
        assert_eq!(encode_key_code(KEY_PAGE_UP, ""), Some(b"\x1b[5~".to_vec()));
        assert_eq!(
            encode_key_code(KEY_PAGE_DOWN, ""),
            Some(b"\x1b[6~".to_vec())
        );
    }

    #[test]
    fn encodes_function_keys() {
        assert_eq!(encode_key_code(KEY_F1, ""), Some(b"\x1bOP".to_vec()));
        assert_eq!(encode_key_code(KEY_F2, ""), Some(b"\x1bOQ".to_vec()));
        assert_eq!(encode_key_code(KEY_F5, ""), Some(b"\x1b[15~".to_vec()));
        assert_eq!(encode_key_code(KEY_F12, ""), Some(b"\x1b[24~".to_vec()));
    }

    #[test]
    fn encodes_modified_navigation_keys() {
        assert_eq!(
            encode_modified_key(KEY_UP, NSEventModifierFlags::Shift, ""),
            Some(b"\x1b[1;2A".to_vec())
        );
        assert_eq!(
            encode_modified_key(
                KEY_RIGHT,
                NSEventModifierFlags::Shift | NSEventModifierFlags::Option,
                ""
            ),
            Some(b"\x1b[1;4C".to_vec())
        );
        assert_eq!(
            encode_modified_key(KEY_LEFT, NSEventModifierFlags::Control, ""),
            Some(b"\x1b[1;5D".to_vec())
        );
        assert_eq!(
            encode_modified_key(
                KEY_FORWARD_DELETE,
                NSEventModifierFlags::Shift | NSEventModifierFlags::Option,
                ""
            ),
            Some(b"\x1b[3;4~".to_vec())
        );
    }

    #[test]
    fn encodes_modified_function_keys() {
        assert_eq!(
            encode_modified_key(KEY_F1, NSEventModifierFlags::Shift, ""),
            Some(b"\x1b[1;2P".to_vec())
        );
        assert_eq!(
            encode_modified_key(KEY_F12, NSEventModifierFlags::Control, ""),
            Some(b"\x1b[24;5~".to_vec())
        );
    }

    #[test]
    fn reserves_command_combinations_for_app_shortcuts() {
        assert_eq!(
            encode_modified_key(8, NSEventModifierFlags::Command, "c"),
            None
        );
        assert_eq!(
            encode_modified_key(KEY_BACKSPACE, NSEventModifierFlags::Command, ""),
            None
        );
    }

    #[test]
    fn encodes_option_as_meta_prefix() {
        assert_eq!(
            encode_modified_key(0, NSEventModifierFlags::Option, "x"),
            Some(b"x".to_vec())
        );
        assert_eq!(
            encode_modified_key(KEY_LEFT, NSEventModifierFlags::Option, ""),
            Some(b"\x1bb".to_vec())
        );
        assert_eq!(
            encode_modified_key(KEY_RIGHT, NSEventModifierFlags::Option, ""),
            Some(b"\x1bf".to_vec())
        );
    }

    #[test]
    fn passes_confirmed_ime_text_as_utf8() {
        assert_eq!(encode_key_code(0, "한글"), Some("한글".as_bytes().to_vec()));
    }

    #[test]
    fn encodes_application_cursor_keys_for_tui_modes() {
        assert_eq!(
            super::encode_application_cursor_key(KEY_UP),
            Some(b"\x1bOA".to_vec())
        );
        assert_eq!(
            super::encode_application_cursor_key(KEY_DOWN),
            Some(b"\x1bOB".to_vec())
        );
        assert_eq!(
            super::encode_application_cursor_key(KEY_RIGHT),
            Some(b"\x1bOC".to_vec())
        );
        assert_eq!(
            super::encode_application_cursor_key(KEY_LEFT),
            Some(b"\x1bOD".to_vec())
        );
    }
}
