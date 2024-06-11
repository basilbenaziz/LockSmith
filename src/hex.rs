pub struct Hex {
    encoded_str: String,
}

impl Hex {
    /// The function `new` creates a new instance of the `Hex` struct with the provided encoded
    /// string.
    /// 
    /// Arguments:
    /// 
    /// * `encoded_str`: The `encoded_str` parameter in the `new` function is a `String` type that
    /// represents the hex encoded string that will be used to create a new instance of the `Hex`
    /// struct.
    /// 
    /// Returns:
    /// 
    /// A new instance of the `Hex` struct with the `encoded_str` field set to the provided `String`
    /// value.
    pub fn new(encoded_str: String) -> Hex {
        Hex { encoded_str }
    }

    /// The function `is_hex` checks if all characters in the encoded string are valid hex
    /// characters.
    /// 
    /// Returns:
    /// 
    /// The `is_hex` method returns a boolean value indicating whether the encoded string is a valid
    /// hex string. It checks if all characters in the string are valid hex digits (0-9, a-f, A-F).
    /// If these conditions are met, the method returns `true`, indicating that the string is a valid
    /// hex string.
    // pub fn is_hex(input: String) -> bool {
    //     input
    //         .replace(" ", "")
    //         .chars()
    //         .all(|c| c.is_digit(16))
    // }
    pub fn is_hex(input: String) -> bool {
        let cleaned_input = input.replace(" ", "");
        if cleaned_input.chars().all(|c| c == '0' || c == '1') {
            return false; // It's binary
        }
        cleaned_input.chars().all(|c| c.is_digit(16))
    }



   /// The `hex_decode` function decodes a hexadecimal-encoded string by converting pairs of hexadecimal
   /// characters into their corresponding bytes.
   /// 
   /// Returns:
   /// 
   /// The `hex_decode` function returns the decoded string after processing the hexadecimal encoded
   /// input string. If the input string contains invalid characters for a hexadecimal string, it
   /// returns the message "Invalid character in hex string". If the decoded bytes cannot be converted
   /// to a valid UTF-8 string, it returns "Invalid UTF-8 sequence". Otherwise, it returns the decoded
   /// string.
    pub fn hex_decode(&mut self) -> String {
        self.encoded_str = self.encoded_str.replace(" ", ""); // Trim spaces from the input string

        if !Self::is_hex(self.encoded_str.clone()) {
            return "Invalid character in hex string".to_string();
        }

        let mut output = Vec::new();
        let mut chars = self.encoded_str.chars();

        while let (Some(high), Some(low)) = (chars.next(), chars.next()) {
            let high_digit = high.to_digit(16).unwrap() as u8;
            let low_digit = low.to_digit(16).unwrap() as u8;
            output.push((high_digit << 4) | low_digit);
        }

        self.encoded_str = String::from_utf8(output)
            .unwrap_or_else(|_| "Invalid UTF-8 sequence".to_string());

        print!("Decode Hex: {}\n", self.encoded_str);
        self.encoded_str.clone()
    }
    


}
