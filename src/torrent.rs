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
