use std::path::PathBuf;
use clap::{Parser, Subcommand};
use serde::Deserialize;
use hashes::Hashes;

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Decode { value: String },
    Info { torrent: PathBuf },
}


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
    pieces: Hashes,

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

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Decode { value } => {
            unimplemented!("serde_bencode -> serde_json::Value is borked");
        }
        Command::Info { torrent } => {
            let dot_torrent = std::fs::read(torrent)?;
            let t: Torrent = serde_bencode::from_bytes(&dot_torrent)?;
            println!("Tracker URL: {}", t.announce);
            println!("{:?}", t);
        }
    }

    Ok(())
}


// TODO: Readmore on this section
// https://serde.rs/impl-deserialize.html

mod hashes {
    use serde::de::{self, Deserialize, Deserializer, Visitor};
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
}
