pub(crate) const LEFT_BUTTON: u16 = 0;
pub(crate) const LEFT_DRAG: u16 = 32;
pub(crate) const WHEEL_UP: u16 = 64;
pub(crate) const WHEEL_DOWN: u16 = 65;

pub(crate) fn sgr_mouse_report(code: u16, row: usize, col: usize, release: bool) -> Vec<u8> {
    let final_byte = if release { 'm' } else { 'M' };
    format!("\x1b[<{};{};{}{}", code, col + 1, row + 1, final_byte).into_bytes()
}

#[cfg(test)]
mod tests {
    #[test]
    fn encodes_sgr_mouse_press() {
        assert_eq!(
            super::sgr_mouse_report(super::LEFT_BUTTON, 1, 2, false),
            b"\x1b[<0;3;2M".to_vec()
        );
    }

    #[test]
    fn encodes_sgr_mouse_release() {
        assert_eq!(
            super::sgr_mouse_report(super::LEFT_BUTTON, 1, 2, true),
            b"\x1b[<0;3;2m".to_vec()
        );
    }

    #[test]
    fn encodes_sgr_mouse_wheel() {
        assert_eq!(
            super::sgr_mouse_report(super::WHEEL_UP, 0, 0, false),
            b"\x1b[<64;1;1M".to_vec()
        );
    }
}
