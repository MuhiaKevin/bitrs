use std::path::PathBuf;
use clap::{Parser, Subcommand};
use anyhow::Context;
use sha1::{Digest, Sha1};
use bitrs::torrent::{Keys, Torrent};
use bitrs::tracker::TrackerRequest;


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

const PORT: u16 = 6881;
const PEER_ID: &'static str =  "00112233445566778899";


fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Decode { value: _ } => {
            unimplemented!("serde_bencode -> serde_json::Value is borked");
        }
        Command::Info { torrent } => {

            let dot_torrent = std::fs::read(torrent)?;
            let t: Torrent = serde_bencode::from_bytes(&dot_torrent)?;
            println!("Tracker URL: {}", t.announce);


            let length = if let Keys::SingleFile { length } =  t.info.keys {
                length
            } else {
                todo!();
            };

            
            let info_bencode = serde_bencode::to_bytes(&t.info).context("re-encode info section")?;
            let mut hasher = Sha1::new();
           
            hasher.update(&info_bencode);
            let info_hash = hasher.finalize();
            
            // println!("info_hash {}", hex::encode(&info_hash) );

            // for (index, piece_hash_bytes) in t.info.pieces.0.iter().enumerate() {
            //     println!("{}: {}",index+1, hex::encode(&piece_hash_bytes));
            // }

            let request =  TrackerRequest { 
                info_hash: info_hash.into(),
                peer_id: PEER_ID.to_string(),
                port: PORT,
                uploaded: 0,
                downloaded: 0,
                left: length,
                compact: 1, 
            };

            println!("{:#?}", request);
        }
    }

    Ok(())
}

