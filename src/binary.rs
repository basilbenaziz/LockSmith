pub struct Binary {
    encoded_str: String,
}

impl Binary {
    pub fn new(data: String) -> Self {
        Binary { encoded_str: data }
    }

    pub fn is_binary(input: String) -> bool {
        input
            .replace(" ", "")
            .chars()
            .all(|c| c == '0' || c == '1')
    }

    

    pub fn binary_decode(&mut self) -> String {
        self.encoded_str = self.encoded_str.replace(" ", "");

        if !Self::is_binary(self.encoded_str.clone()) {
            return "Invalid character in binary string".to_string();
        };

        let mut decoded = String::new();
        for i in 0..self.encoded_str.len() / 8 {
            let byte = &self.encoded_str[i * 8..(i + 1) * 8];
            let byte = u8::from_str_radix(byte, 2).unwrap();
            decoded.push(byte as char);
        }
        self.encoded_str = decoded;
        print!("Decode Binary: {}\n", self.encoded_str);
        self.encoded_str.clone()




    }

}



