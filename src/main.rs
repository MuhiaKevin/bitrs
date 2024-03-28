pub mod parse; // parse here is the folder

// parse => is the folder
// parse_bencode => is the module 
// decode_bencoded_value => is the function being imported
use parse::parse_bencode::decode_bencoded_value;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];

        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.0);

    } else {
        println!("unkown command: {:?}", command);
    }
}
