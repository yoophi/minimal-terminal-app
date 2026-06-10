pub(crate) fn bracketed_paste_bytes(input: &str) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(input.len() + 12);
    bytes.extend_from_slice(b"\x1b[200~");
    bytes.extend_from_slice(input.as_bytes());
    bytes.extend_from_slice(b"\x1b[201~");
    bytes
}

#[cfg(test)]
mod tests {
    #[test]
    fn wraps_bracketed_paste_bytes() {
        assert_eq!(
            super::bracketed_paste_bytes("hello"),
            b"\x1b[200~hello\x1b[201~".to_vec()
        );
    }

    #[test]
    fn preserves_utf8_paste_content() {
        assert_eq!(
            super::bracketed_paste_bytes("한글"),
            b"\x1b[200~\xed\x95\x9c\xea\xb8\x80\x1b[201~".to_vec()
        );
    }
}
