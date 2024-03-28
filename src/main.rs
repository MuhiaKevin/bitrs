// version is the file parse_bencode.rs
mod parse_bencode;

// from file parse_bencode (parse_bencode.rs) import function decode_bencoded_value
use parse_bencode::decode_bencoded_value;

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
