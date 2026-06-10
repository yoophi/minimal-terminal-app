use objc2_app_kit::{NSEvent, NSEventModifierFlags};

const KEY_RETURN: u16 = 36;
const KEY_BACKSPACE: u16 = 51;
const KEY_FORWARD_DELETE: u16 = 117;
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

    if flags.contains(NSEventModifierFlags::Option) {
        return encode_option_key(key_code, input);
    }

    match key_code {
        KEY_RETURN => Some(b"\r".to_vec()),
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

#[cfg(test)]
mod tests {
    use objc2_app_kit::NSEventModifierFlags;

    const KEY_RETURN: u16 = 36;
    const KEY_BACKSPACE: u16 = 51;
    const KEY_FORWARD_DELETE: u16 = 117;
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
    fn reserves_command_combinations_for_app_shortcuts() {
        assert_eq!(
            encode_modified_key(8, NSEventModifierFlags::Command, "c"),
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
}
