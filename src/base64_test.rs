

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

    /// The function `is_base64` checks if all characters in the encoded string are valid Base64
    /// characters.
    /// 
    /// Returns:
    /// 
    /// The `is_base64` method returns a boolean value indicating whether the encoded string is a valid
    /// Base64 string. It checks if the length of the encoded string is a multiple of 4 and if all
    /// characters in the string are either ASCII alphanumeric characters or one of the characters '+',
    /// '/', or '='. If these conditions are met, the method returns `true`, indicating that the string
    /// is a valid
    pub fn is_base64(&self) -> bool {
        if self.encoded_str.len() % 4 != 0 {
            return false;
        }
        // This code snippet is checking if all characters in the encoded string are either ASCII
        // alphanumeric characters or one of the characters '+', '/', or '='.
        self.encoded_str
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=')
    }



    pub fn decode_base64(&self) -> Option<String> {
        return Some("".to_string());
    }

}
