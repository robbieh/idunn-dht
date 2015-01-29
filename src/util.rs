pub fn parse_hex_bytes(bytes: &[u8]) -> Option<usize> {
    let mut result: usize = 0;
    for &b in bytes.iter() {
        let d = if b'0' <= b && b <= b'9' { b - b'0' }
            else if b'A' <= b && b <= b'F' { (b - b'A') + 0xa }
            else if b'a' <= b && b <= b'f' { (b - b'a') + 0xa }
            else { return None; };
        result = (result * 16) + (d as usize);
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_bytes() {
        assert_eq!(Some(164), parse_hex_bytes("a4".as_bytes()));
    }
}
