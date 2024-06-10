// mod base64;

mod base64_test;
use base64_test::Base64;




fn main() {
    let encoded_str = Base64::new("VmpGYVYyRXhXWGxVV0d4VVlUSm9VVlZyVWtKUFVUMDk=".to_string());
    print!("{}\n", encoded_str.is_base64());
    
    
    // let mut encoded_str = "U0dWc2JHOGdVMjVuY0dWdWRGOTFjR1Z5ZG1sa1pXNTBhWFI1TG1OdmJTOWhhV1E9";
    // let encoded_str = Some("VmpGYVYyRXhXWGxVV0d4VVlUSm9VVlZyVWtKUFVUMDk=".to_string());    
    
    // base64::decode_base64_string(encoded_str);
}
