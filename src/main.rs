mod base64;
mod hex;
mod url;

fn main() {
    //Base64//SGVsbG8gV29ybGQh
    //Base64////Base64//U0dWc2JHOGdWMjl5YkdRaA==
    //HEX//68 69 20 6d 65
    let encoded_str = "U0dWc2JHOGdWMjl5YkdRaA=="; // Example input, replace with your string




    if base64::is_base64(encoded_str) {
        match base64::decode_base64(encoded_str) {
            Ok(decoded) => {
                match String::from_utf8(decoded) {
                    Ok(decoded_string) => println!("Decoded Base64 string: {}", decoded_string),
                    Err(_) => println!("Failed to convert decoded Base64 bytes to string"),
                }
            },
            Err(e) => println!("Error: {}", e),
        }




    } else if hex::is_hex(encoded_str) {
        match hex::decode_hex(encoded_str) {
            Ok(decoded) => {
                match String::from_utf8(decoded) {
                    Ok(decoded_string) => println!("Decoded Hex string: {}", decoded_string),
                    Err(_) => println!("Failed to convert decoded Hex bytes to string"),
                }
            },
            Err(e) => println!("Error: {}", e),
        }





    } else if url::is_url_encoded(encoded_str) {
        match url::decode_url(encoded_str) {
            Ok(decoded_string) => println!("Decoded URL string: {}", decoded_string),
            Err(e) => println!("Error: {}", e),
        }



    } else {
        println!("The string is neither valid Base64, Hex, nor URL encoded.");
    }



}
