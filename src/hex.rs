pub fn hex_char_to_val(c: char) -> Option<u8> {
    match c {
        '0'..='9' => Some(c as u8 - '0' as u8),
        'a'..='f' => Some(c as u8 - 'a' as u8 + 10),
        'A'..='F' => Some(c as u8 - 'A' as u8 + 10),
        _ => None,
    }
}

pub fn decode_hex(encoded: &str) -> Result<Vec<u8>, &'static str> {
    if encoded.len() % 2 != 0 {
        return Err("Invalid Hex string length");
    }

    let mut decoded_bytes = Vec::new();
    let mut chars = encoded.chars();

    while let (Some(high), Some(low)) = (chars.next(), chars.next()) {
        let high_val = match hex_char_to_val(high) {
            Some(val) => val,
            None => return Err("Invalid character in Hex string"),
        };
        let low_val = match hex_char_to_val(low) {
            Some(val) => val,
            None => return Err("Invalid character in Hex string"),
        };
        decoded_bytes.push((high_val << 4) | low_val);
    }

    Ok(decoded_bytes)
}

pub fn is_hex(s: &str) -> bool {
    if s.len() % 2 != 0 {
        return false;
    }
    s.chars().all(|c| c.is_ascii_hexdigit())
}
