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

pub fn is_url_encoded(s: &str) -> bool {
    s.chars().any(|c| c == '%' || c == '+')
}
