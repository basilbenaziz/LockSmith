/// The function `decode_url` decodes a URL-encoded string by replacing percent-encoded characters with
/// their corresponding ASCII characters.
/// 
/// Arguments:
/// 
/// * `encoded`: The function `decode_url` you provided is meant to decode a URL-encoded string. It
/// replaces `%XX` sequences with their corresponding ASCII characters and decodes the `+` character as
/// a space.
/// 
/// Returns:
/// 
/// The `decode_url` function returns a `Result<String, &'static str>`.
pub fn decode_url(encoded: &str) -> Result<String, &'static str> {
    let mut decoded = String::new();
    let mut chars = encoded.chars();

    while let Some(c) = chars.next() {
        if c == '%' {
            let high = chars.next().ok_or("Invalid URL encoding")?;
            let low = chars.next().ok_or("Invalid URL encoding")?;
            let high_val = crate::hex::hex_char_to_val(high).ok_or("Invalid URL encoding")?;
            let low_val = crate::hex::hex_char_to_val(low).ok_or("Invalid URL encoding")?;
            decoded.push((high_val << 4 | low_val) as char);
        } else if c == '+' {
            decoded.push(' ');
        } else {
            decoded.push(c);
        }
    }

    Ok(decoded)
}



/// The function `is_url_encoded` checks if a given string is URL encoded by looking for the presence of
/// '%' or '+' characters.
/// 
/// Arguments:
/// 
/// * `s`: The parameter `s` is a reference to a string slice (`&str`) that represents the input string
/// to be checked for URL encoding.
/// 
/// Returns:
/// 
/// The function `is_url_encoded` returns a boolean value indicating whether the input string `s`
/// contains any URL-encoded characters, specifically the characters '%' or '+'.
pub fn is_url_encoded(s: &str) -> bool {
    s.chars().any(|c| c == '%' || c == '+')
}
