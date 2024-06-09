mod base64;







fn main() {
    // let mut encoded_str = "U0dWc2JHOGdVMjVuY0dWdWRGOTFjR1Z5ZG1sa1pXNTBhWFI1TG1OdmJTOWhhV1E9";
    let encoded_str = Some("VmpGYVYyRXhXWGxVV0d4VVlUSm9VVlZyVWtKUFVUMDk=".to_string());
    
    
    base64::decode_base64_string(encoded_str);

}
