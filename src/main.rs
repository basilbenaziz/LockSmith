// mod base64;
mod base64;
use base64::Base64;

mod hex;
use hex::Hex;




fn main() {
    // let mut encoded = Base64::new("VmpGYVYyRXhXWGxVV0d4VVlUSm9VVlZyVWtKUFVUMDk=".to_string());
    // print!("{}\n", encoded.base64_decode());

    // let mut encoded = Hex::new("3638203639203230203734203638203635203732203635".to_string());
    // print!("{}\n", encoded.hex_decode());


    // let mut b = "VmpGYVYyRXhXWGxVV0d4VVlUSm9VVlZyVWtKUFVUMDk=".to_string();  //Base64  
    // let mut b = "6869207468657265".to_string(); //Hex
    let mut b = "61476b676447686c636d553d".to_string(); //Base64 and then Hex
    // let mut b = "4e6a45674e4463674e6d49674e6a63674e6a51674e4463674e6a67674e6d4d674e6a4d674e6d51674e5455674d32513d".to_string(); //Base64 and then Hex

    while Base64::is_base64(b.clone()) || Hex::is_hex(b.clone()) {
        if Base64::is_base64(b.clone()) {
            let b_clone = b.clone();
            let mut encoded = Base64::new(b_clone);
            b = encoded.base64_decode();
        } else if Hex::is_hex(b.clone()) {
            let b_clone = b.clone();
            let mut encoded = Hex::new(b_clone);
            b = encoded.hex_decode();
        }
    }

}
