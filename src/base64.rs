pub fn base64_char_to_val(c: char) -> Option<u8> {
    match c {
        'A'..='Z' => Some(c as u8 - 'A' as u8),
        'a'..='z' => Some(c as u8 - 'a' as u8 + 26),
        '0'..='9' => Some(c as u8 - '0' as u8 + 52),
        '+' => Some(62),
        '/' => Some(63),
        _ => None,
    }
}








pub fn decode_base64(encoded: &str) -> Result<Vec<u8>, &'static str> {
    let encoded = encoded.trim_end_matches('=');


    // if encoded.len() % 4 != 0 {
        // return Err("Invalid Base64 string length");
    // }



    let mut decoded_bytes: Vec<u8> = Vec::new();
    let mut buffer = 0u32;
    let mut bits_collected = 0;



    for c in encoded.chars() {
        let val = match base64_char_to_val(c) {
            Some(val) => val,
            None => return Err("Invalid character in Base64 string"),
        };
        buffer = (buffer << 6) | (val as u32);
        bits_collected += 6;

        if bits_collected >= 8 {
            bits_collected -= 8;
            decoded_bytes.push((buffer >> bits_collected) as u8);
            buffer &= (1 << bits_collected) - 1;
        }
    }

    Ok(decoded_bytes)
}












pub fn is_base64(s: &str) -> bool {
    if s.len() % 4 != 0 {
        return false;
    }
    s.chars().all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=')
}
