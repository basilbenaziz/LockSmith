

pub struct Base64 {
    encoded_str: String,
}

impl Base64 {
    /// The function `new` creates a new instance of the `Base64` struct with the provided encoded
    /// string.
    /// 
    /// Arguments:
    /// 
    /// * `encoded_str`: The `encoded_str` parameter in the `new` function is a `String` type that
    /// represents the base64 encoded string that will be used to create a new instance of the `Base64`
    /// struct.
    /// 
    /// Returns:
    /// 
    /// A new instance of the `Base64` struct with the `encoded_str` field set to the provided `String`
    /// value.
    pub fn new(encoded_str: String) -> Base64 {
        Base64 { encoded_str }
    }


    
    pub fn is_base64(input: String) -> bool {
        if input.len() % 4 != 0 {
            return false;
        }
        
        // Check if the input is a number
        if input.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        
        // Check if the input is purely hexadecimal
        if input.chars().all(|c| c.is_ascii_hexdigit()) {
            return false;
        }
    
        // Check if all characters in the encoded string are either ASCII
        // alphanumeric characters or one of the characters '+', '/', or '='.
        input.chars().all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=')
    }
    


/// The function `base64_decode` decodes a Base64 encoded string and returns the decoded string.
/// 
/// Returns:
/// 
/// The `base64_decode` function returns the decoded Base64 string as a `String`.
    pub fn base64_decode(&mut self) -> String {
        
        while  Base64::is_base64(self.encoded_str.clone()) {
            let base64_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
            let mut output = Vec::new();
            let mut buffer = 0u32;
            let mut bits_collected = 0;

            for c in self.encoded_str.chars() {
                if c == '=' {
                    break;
                }

                let value = match base64_chars.find(c) {
                    Some(val) => val as u32,
                    None => return "Invalid character in Base64 string".to_string(),
                };

                buffer = (buffer << 6) | value;
                bits_collected += 6;

                if bits_collected >= 8 {
                    bits_collected -= 8;
                    let byte = (buffer >> bits_collected) as u8;
                    output.push(byte);
                    buffer &= (1 << bits_collected) - 1;
                }
            }


            
            self.encoded_str = String::from_utf8(output).unwrap();
            print!("Decode Base64: {}\n", self.encoded_str);
        }
        
        return self.encoded_str.to_string();
        
    }

}
