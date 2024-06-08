/// The function `hex_char_to_val` converts a hexadecimal character to its corresponding numerical
/// value.
/// 
/// Arguments:
/// 
/// * `c`: The function `hex_char_to_val` takes a single character `c` as input and returns an
/// `Option<u8>`. The function converts a hexadecimal character to its corresponding numerical value.
/// 
/// Returns:
/// 
/// The function `hex_char_to_val` takes a character as input and returns an `Option<u8>`. The function
/// matches the input character against different ranges of characters ('0' to '9', 'a' to 'f', 'A' to
/// 'F') and converts them to their corresponding hexadecimal values. If the input character is within
/// one of these ranges, the function returns `Some(value
pub fn hex_char_to_val(c: char) -> Option<u8> {
    match c {
        '0'..='9' => Some(c as u8 - '0' as u8),
        'a'..='f' => Some(c as u8 - 'a' as u8 + 10),
        'A'..='F' => Some(c as u8 - 'A' as u8 + 10),
        _ => None,
    }
}







/// The `decode_hex` function in Rust decodes a hexadecimal encoded string into a vector of bytes.
/// 
/// Arguments:
/// 
/// * `encoded`: The function `decode_hex` takes a hexadecimal encoded string as input and decodes it
/// into a vector of bytes. The input parameter `encoded` is the hexadecimal string that needs to be
/// decoded.
/// 
/// Returns:
/// 
/// The function `decode_hex` returns a `Result` enum with the success variant containing a `Vec<u8>`
/// representing the decoded bytes if the decoding process is successful, and the error variant
/// containing a `&'static str` with an error message if any issues occur during decoding.
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





/// The function `is_hex` in Rust checks if a given string is a valid hexadecimal representation.
/// 
/// Arguments:
/// 
/// * `s`: The parameter `s` is a reference to a string slice (`&str`) that represents a hexadecimal
/// string. The function `is_hex` checks if the input string is a valid hexadecimal string by ensuring
/// that it has an even length and that all characters are valid hexadecimal digits.
/// 
/// Returns:
/// 
/// The function `is_hex` returns a boolean value indicating whether the input string `s` is a valid
/// hexadecimal string.
pub fn is_hex(s: &str) -> bool {
    if s.len() % 2 != 0 {
        return false;
    }
    s.chars().all(|c| c.is_ascii_hexdigit())
}
