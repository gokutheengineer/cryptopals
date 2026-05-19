use anyhow::{Context, Result};

/// Decode a hex string into raw bytes.
pub fn hex_to_bytes(s: &str) -> Result<Vec<u8>> {
    hex::decode(s).context("invalid hex string")
}

/// Encode raw bytes as a lowercase hex string.
pub fn bytes_to_hex(b: &[u8]) -> String {
    hex::encode(b)
}

/// Encode raw bytes as a base64 string.
pub fn bytes_to_base64(b: &[u8]) -> String {
    use base64::{engine::general_purpose::STANDARD, Engine};
    STANDARD.encode(b)
}

/// Decode a base64 string into raw bytes.
pub fn base64_to_bytes(s: &str) -> Result<Vec<u8>> {
    use base64::{engine::general_purpose::STANDARD, Engine};
    STANDARD.decode(s).context("invalid base64 string")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_roundtrip() {
        let original = b"hello cryptopals";
        let encoded = bytes_to_hex(original);
        let decoded = hex_to_bytes(&encoded).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn base64_roundtrip() {
        let original = b"hello cryptopals";
        let encoded = bytes_to_base64(original);
        let decoded = base64_to_bytes(&encoded).unwrap();
        assert_eq!(decoded, original);
    }
}
