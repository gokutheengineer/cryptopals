/// XOR two equal-length byte slices. Returns None if lengths differ.
pub fn fixed_xor(a: &[u8], b: &[u8]) -> Option<Vec<u8>> {
    if a.len() != b.len() {
        return None;
    }
    Some(a.iter().zip(b).map(|(x, y)| x ^ y).collect())
}

/// XOR every byte of `data` against a single `key` byte.
pub fn single_byte_xor(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|b| b ^ key).collect()
}

/// XOR `data` against a repeating `key` (Vigenere / repeating-key XOR).
pub fn repeating_key_xor(data: &[u8], key: &[u8]) -> Vec<u8> {
    assert!(!key.is_empty(), "key must not be empty");
    data.iter()
        .enumerate()
        .map(|(i, b)| b ^ key[i % key.len()])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_xor_basic() {
        let a = vec![0x1c, 0x01];
        let b = vec![0x68, 0x69];
        let result = fixed_xor(&a, &b).unwrap();
        assert_eq!(result, vec![0x74, 0x68]);
    }

    #[test]
    fn fixed_xor_length_mismatch() {
        assert!(fixed_xor(&[1, 2], &[1]).is_none());
    }

    #[test]
    fn repeating_key_xor_basic() {
        let data = b"ICE";
        let key = b"ICE";
        let result = repeating_key_xor(data, key);
        assert_eq!(result, vec![0, 0, 0]);
    }
}
