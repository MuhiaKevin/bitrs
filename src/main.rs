use std::path::PathBuf;
use anyhow::Context;
use clap::{Parser, Subcommand};
use bitrs::torrent::{Keys, Torrent};
use bitrs::tracker::{TrackerRequest, TrackerResponse};


#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Decode { value: String },
    Info { torrent: PathBuf },
    Peers { torrent: PathBuf },
}

const PORT: u16 = 6881;
const PEER_ID: &'static str =  "00112233445566778899";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Decode { value: _ } => {
            unimplemented!("serde_bencode -> serde_json::Value is borked");
        }
        Command::Info { torrent } => {
            let t = Torrent::open(torrent);

            let info_hash = t.info_hash();
            
            println!("info_hash {}", hex::encode(&info_hash) );

            // for (index, piece_hash_bytes) in t.info.pieces.0.iter().enumerate() {
            //     println!("{}: {}",index+1, hex::encode(&piece_hash_bytes));
            // }

        }


        Command::Peers { torrent } => {
            let t = Torrent::open(torrent);

            let length = if let Keys::SingleFile { length } =  t.info.keys {
                length
            } else {
                todo!();
            };

            let info_hash = t.info_hash();

            let request = TrackerRequest {
                peer_id: PEER_ID.to_string(),
                port: PORT,
                uploaded: 0,
                downloaded: 0,
                left: length,
                compact: 1,
            };


            // create url params from the request object
            // Something like this => peer_id=00112233445566778899&port=6881&uploaded=0&downloaded=0&left=659554304&compact=1
            let url_params = serde_urlencoded::to_string(&request).context("url-encode tracker parameters")?;





            // build the whole url using the announce and the url params
            // something like this :
            //     http://bttracker.debian.org:6969/announce?peer_id=00112233445566778899&port=6881&uploaded=0&downloaded=0&left=659554304&compact=1&info_hash=%2b%66%98%00%93%bc%11%80%6f%ab %50%cb%3c%b4%18%35%b9%5a%03%62
            let tracker_url = format!( "{}?{}&info_hash={}", t.announce, url_params, &urlencode(&info_hash));
            println!("{tracker_url}");


            // make get request to the url
            let response = reqwest::get(tracker_url).await.context("query tracker")?;

            // convert the repsonse to bytes instead of the reqwest::Response object
            let response = response.bytes().await.context("fetch tracker response")?;


    
            // now using deserialize or convert the response in bytes to the trackerRepsonse struct
            // serde_bencode will use our wrapper deserializer to deserialize the peers attribute
            // from bytes to a Vec of Peers struct 
            let response: TrackerResponse = serde_bencode::from_bytes(&response).context("parse tracker response")?;
            

            println!("Interval {}", response.interval);

            // loop over the peers vec
            for peer in &response.peers.0 {
                println!("{}:{}", peer.ip(), peer.port());
            }
        }
    }

    Ok(())
}



// this will convert the slice of u8 info hash of size 20 to a hex string for url encoding
fn urlencode(t: &[u8; 20]) -> String {
    let mut encoded = String::with_capacity(3 * t.len());
    for &byte in t {
        encoded.push('%');
        encoded.push_str(&hex::encode(&[byte]));
    }
    encoded
}

