use std::path::PathBuf;
use sha1::{Digest, Sha1};
pub use hashes::Hashes;

use serde::{Deserialize, Serialize};

// https://medium.com/@2018.itsuki/detail-guide-to-serialization-and-deserialization-with-serde-in-rust-4fa70a6a8c4b

// https://www.baeldung.com/cs/serialization-deserialization

// Serialization is the process of converting an object’s state to a byte stream.
// This byte stream can then be saved to a file, sent over a network, or stored in a database

// Deserializiton involves taking a byte stream and converting it back into an object.
#[derive(Debug, Clone, Deserialize)]
pub struct Torrent {
    pub announce: String,
    pub info: Info,
}


impl Torrent {
    pub fn open(torrent: PathBuf) -> Self {
        let dot_torrent = std::fs::read(torrent).unwrap();
        let t: Torrent = serde_bencode::from_bytes(&dot_torrent).unwrap();
        t
    }

    pub fn info_hash(&self) -> [u8; 20] {
        let info_bencode = serde_bencode::to_bytes(&self.info).expect("re-encode info section");
        let mut hasher = Sha1::new();

        hasher.update(&info_bencode);
        hasher.finalize().into()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Info {
   pub name: String,

    #[serde(rename = "piece length")]
    pub plength: usize,
    pub pieces: Hashes,

    #[serde(flatten)]
    pub keys: Keys,
}



#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Keys{
    SingleFile{
        length: usize,
    },
    MultiFile{
        files: Vec<File>,
    },
}



#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct File {
    pub length: usize,
    pub path: Vec<String>,
}




pub fn decode_bencoded_value(encoded_value: &str) -> (serde_json::Value, &str) {
    match encoded_value.chars().next() {
        Some('i') => {
            if let Some((n, rest)) =  encoded_value
                .split_at(1).1
                    .split_once('e')
                    .and_then(|(digits, rest)| {
                            let n = digits.parse::<i64>().ok()?;
                            Some((n, rest))
                            })
            {
                // TODO: read more on the .into() bcoz i dont understand it
                // https://dev.to/richardbray/type-conversion-in-rust-as-from-and-into-493l
                // https://www.reddit.com/r/rust/comments/p69c9w/what_does_the_into_method_do/
                // https://doc.rust-lang.org/rust-by-example/conversion/from_into.html
                return  (n.into(), rest);
            }
        }

        Some('d') => {
            let mut dict = serde_json::Map::new(); // create an empty vec/list
            let mut rest = encoded_value.split_at(1).1; // get rest of list from the letter 'l',

            // first check that rest &str is not empty and it doesnt start with e
            while !rest.is_empty() && !rest.starts_with('e') {
                let (k, remainder) = decode_bencoded_value(rest);

                // use shadowing to replace k with something else 
                // check to see if the k is of type enum variant string and return the string 
                // if not panic the whole program and exit
                let k = match k { 
                    serde_json::Value::String(k) => k,
                        _ => panic!("Dict keys must be of type string")
                };

                let (v, remainder) = decode_bencoded_value(remainder);
                dict.insert(k, v); // saves decoded values of the list into a vector
                rest = remainder;
            }

            return (serde_json::Value::Object(dict.into()), &rest[1..])
        }

        Some('l') => {
            // Example  of list li25e3:fooe
            let mut values = Vec::new(); // create an empty vec/list
            let mut rest = encoded_value.split_at(1).1; // get rest of list from the letter 'l',
                                                        // in this case it will  i25e3:fooe
                                                        // println!("REST of List: {}",rest.starts_with('e'));

                                                        // first check that rest &str is not empty and it doesnt start with e
            while !rest.is_empty() && !rest.starts_with('e') {
                let (v, remainder) = decode_bencoded_value(rest);
                values.push(v); // saves decoded values of the list into a vector
                rest = remainder;
            }

            return (values.into(), &rest[1..])
        }

        Some('0'..='9') => {
            if let Some((len, rest)) = encoded_value.split_once(':') { // get the length of string
                                                                       // and the string separately
                if let Ok(len) = len.parse::<usize>() { // convert the length of the string to u16
                    return (rest[..len].to_string().into(), &rest[len..])
                } 
            }
        }

        _ => {}
    }

    panic!("Unhandled encoded value: {}", encoded_value)
}




// TODO: Readmore on this section
// https://serde.rs/impl-deserialize.html
mod hashes {
    use serde::de::{self, Deserialize, Deserializer, Visitor};
    use serde::ser::{Serialize, Serializer};
    use std::fmt;

    // tuple struct 
    #[derive(Debug, Clone)]
    pub struct Hashes(pub Vec<[u8; 20]>);
    // unit struct
    // Unit structs are useful when you need to implement a trait on something, but don’t need to store any data inside it
    // https://levelup.gitconnected.com/rust-unit-structs-explained-4ad2307efa72
    // https://www.reddit.com/r/rust/comments/lmekck/unit_struct_usage/
    struct HashesVisitor;


    impl<'de> Visitor<'de> for HashesVisitor {
        type Value = Hashes;


        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a byte string whose length is a multiple of 20")
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if v.len() % 20 != 0 {
                return Err(E::custom(format!("length is {}", v.len())));
            }
            // TODO: use array_chunks when stable
            Ok(Hashes(
                v.chunks_exact(20)
                    .map(|slice_20| slice_20.try_into().expect("guaranteed to be length 20"))
                    .collect(),
            ))
        }
    }

    impl<'de> Deserialize<'de> for Hashes {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_bytes(HashesVisitor)
        }
    }



    impl Serialize for Hashes {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let single_slice = self.0.concat();
            serializer.serialize_bytes(&single_slice)
        }
    }
}
