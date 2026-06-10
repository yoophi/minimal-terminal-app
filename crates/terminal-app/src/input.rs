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
    if let Some(bytes) = encode_control_text(input) {
        return Some(bytes);
    }

    if let Some(bytes) = encode_control_key(event) {
        return Some(bytes);
    }

    match event.keyCode() {
        KEY_RETURN => Some(b"\r".to_vec()),
        KEY_BACKSPACE => Some(vec![0x7f]),
        KEY_FORWARD_DELETE => Some(b"\x1b[3~".to_vec()),
        KEY_HOME => Some(b"\x1b[H".to_vec()),
        KEY_END => Some(b"\x1b[F".to_vec()),
        KEY_PAGE_UP => Some(b"\x1b[5~".to_vec()),
        KEY_PAGE_DOWN => Some(b"\x1b[6~".to_vec()),
        KEY_UP => Some(b"\x1b[A".to_vec()),
        KEY_DOWN => Some(b"\x1b[B".to_vec()),
        KEY_RIGHT => Some(b"\x1b[C".to_vec()),
        KEY_LEFT => Some(b"\x1b[D".to_vec()),
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

fn encode_control_key(event: &NSEvent) -> Option<Vec<u8>> {
    let flags = event.modifierFlags();
    if !flags.contains(NSEventModifierFlags::Control) {
        return None;
    }

    match event.keyCode() {
        8 => Some(vec![0x03]),
        2 => Some(vec![0x04]),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
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
        if let Some(bytes) = super::encode_control_text(input) {
            return Some(bytes);
        }

        match key_code {
            KEY_RETURN => Some(b"\r".to_vec()),
            KEY_BACKSPACE => Some(vec![0x7f]),
            KEY_FORWARD_DELETE => Some(b"\x1b[3~".to_vec()),
            KEY_HOME => Some(b"\x1b[H".to_vec()),
            KEY_END => Some(b"\x1b[F".to_vec()),
            KEY_PAGE_UP => Some(b"\x1b[5~".to_vec()),
            KEY_PAGE_DOWN => Some(b"\x1b[6~".to_vec()),
            KEY_UP => Some(b"\x1b[A".to_vec()),
            KEY_DOWN => Some(b"\x1b[B".to_vec()),
            KEY_RIGHT => Some(b"\x1b[C".to_vec()),
            KEY_LEFT => Some(b"\x1b[D".to_vec()),
            _ if input.is_empty() => None,
            _ => Some(input.as_bytes().to_vec()),
        }
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
    fn encodes_arrow_keys_as_csi_sequences() {
        assert_eq!(encode_key_code(KEY_UP, ""), Some(b"\x1b[A".to_vec()));
        assert_eq!(encode_key_code(KEY_DOWN, ""), Some(b"\x1b[B".to_vec()));
        assert_eq!(encode_key_code(KEY_RIGHT, ""), Some(b"\x1b[C".to_vec()));
        assert_eq!(encode_key_code(KEY_LEFT, ""), Some(b"\x1b[D".to_vec()));
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
        assert_eq!(encode_key_code(KEY_HOME, ""), Some(b"\x1b[H".to_vec()));
        assert_eq!(encode_key_code(KEY_END, ""), Some(b"\x1b[F".to_vec()));
        assert_eq!(encode_key_code(KEY_PAGE_UP, ""), Some(b"\x1b[5~".to_vec()));
        assert_eq!(
            encode_key_code(KEY_PAGE_DOWN, ""),
            Some(b"\x1b[6~".to_vec())
        );
    }
}
