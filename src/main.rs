// version is the file parse_bencode.rs
mod parse_bencode;

// from file parse_bencode (parse_bencode.rs) import function decode_bencoded_value
use parse_bencode::decode_bencoded_value;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct Torrent {
    announce: String,
    info: Info,
}

#[derive(Debug, Clone, Deserialize)]
struct Info {
    name: String,

    #[serde(rename = "piece length")]
    plength: usize,
    pieces: Vec<u8>,

    #[serde(flatten)]
    keys: Keys,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum Keys{
    SingleFile{
        length: usize,
    },
    MultiFile{
        files: Vec<File>,
    },
}

#[derive(Debug, Clone, Deserialize)]
struct File {
    length: usize,
    path: Vec<String>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        //
        // let mut buffer = String::new();
        // std::io::stdin().read_line(&mut buffer).unwrap();
        //
        //
        // let decoded_value = decode_bencoded_value(&buffer);
        let decoded_value = decode_bencoded_value(&encoded_value);
        println!("{}", decoded_value.0);

    } else {
        println!("unkown command: {:?}", command);
    }
}

