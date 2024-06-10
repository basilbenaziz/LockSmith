// mod base64;
mod base64;
use base64::Base64;




fn main() {
    let mut encoded = Base64::new("VmpGYVYyRXhXWGxVV0d4VVlUSm9VVlZyVWtKUFVUMDk=".to_string());
    print!("{}\n", encoded.base64_decode());
}
