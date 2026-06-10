pub(crate) const LEFT_BUTTON: u16 = 0;
pub(crate) const LEFT_RELEASE: u16 = 3;
pub(crate) const LEFT_DRAG: u16 = 32;
pub(crate) const WHEEL_UP: u16 = 64;
pub(crate) const WHEEL_DOWN: u16 = 65;
pub(crate) const SHIFT_MODIFIER: u16 = 4;
pub(crate) const META_MODIFIER: u16 = 8;
pub(crate) const CONTROL_MODIFIER: u16 = 16;

pub(crate) fn sgr_mouse_report(
    code: u16,
    modifiers: u16,
    row: usize,
    col: usize,
    release: bool,
) -> Vec<u8> {
    let final_byte = if release { 'm' } else { 'M' };
    format!(
        "\x1b[<{};{};{}{}",
        code + modifiers,
        col + 1,
        row + 1,
        final_byte
    )
    .into_bytes()
}

pub(crate) fn legacy_mouse_report(
    code: u16,
    modifiers: u16,
    row: usize,
    col: usize,
    release: bool,
) -> Vec<u8> {
    let code = if release { LEFT_RELEASE } else { code } + modifiers;
    let encoded_code = legacy_mouse_byte(code);
    let encoded_col = legacy_mouse_byte((col + 1).min(223) as u16);
    let encoded_row = legacy_mouse_byte((row + 1).min(223) as u16);
    vec![0x1b, b'[', b'M', encoded_code, encoded_col, encoded_row]
}

fn legacy_mouse_byte(value: u16) -> u8 {
    (value.min(223) + 32) as u8
}

#[cfg(test)]
mod tests {
    #[test]
    fn encodes_sgr_mouse_press() {
        assert_eq!(
            super::sgr_mouse_report(super::LEFT_BUTTON, 0, 1, 2, false),
            b"\x1b[<0;3;2M".to_vec()
        );
    }

    #[test]
    fn encodes_sgr_mouse_release() {
        assert_eq!(
            super::sgr_mouse_report(super::LEFT_BUTTON, 0, 1, 2, true),
            b"\x1b[<0;3;2m".to_vec()
        );
    }

    #[test]
    fn encodes_sgr_mouse_wheel() {
        assert_eq!(
            super::sgr_mouse_report(super::WHEEL_UP, 0, 0, 0, false),
            b"\x1b[<64;1;1M".to_vec()
        );
    }

    #[test]
    fn encodes_sgr_mouse_modifiers() {
        assert_eq!(
            super::sgr_mouse_report(
                super::LEFT_BUTTON,
                super::SHIFT_MODIFIER | super::CONTROL_MODIFIER,
                1,
                2,
                false,
            ),
            b"\x1b[<20;3;2M".to_vec()
        );
    }

    #[test]
    fn encodes_legacy_mouse_press() {
        assert_eq!(
            super::legacy_mouse_report(super::LEFT_BUTTON, 0, 1, 2, false),
            vec![0x1b, b'[', b'M', 32, 35, 34]
        );
    }

    #[test]
    fn encodes_legacy_mouse_release() {
        assert_eq!(
            super::legacy_mouse_report(super::LEFT_BUTTON, 0, 1, 2, true),
            vec![0x1b, b'[', b'M', 35, 35, 34]
        );
    }

    #[test]
    fn encodes_legacy_mouse_wheel() {
        assert_eq!(
            super::legacy_mouse_report(super::WHEEL_DOWN, 0, 0, 0, false),
            vec![0x1b, b'[', b'M', 97, 33, 33]
        );
    }

    #[test]
    fn encodes_legacy_mouse_modifiers() {
        assert_eq!(
            super::legacy_mouse_report(
                super::LEFT_BUTTON,
                super::META_MODIFIER | super::CONTROL_MODIFIER,
                1,
                2,
                false,
            ),
            vec![0x1b, b'[', b'M', 56, 35, 34]
        );
    }
}
