use serde::{Deserialize, Serialize};

use peers::Peers;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TrackerRequest {
    pub info_hash: [u8; 20],
    pub peer_id: String,
    pub port: u16,
    pub uploaded: usize,
    pub downloaded: usize,
    pub left: usize,
    pub compact: u8,
}


#[derive(Deserialize, Clone, Debug)]
pub struct TrackerResponse {
    interval: usize,
    peers: Peers,
}


mod peers {
    use serde::de::{self, Deserialize, Deserializer, Visitor};
    use serde::ser::{Serialize, Serializer};
    use std::{fmt, u16};
    use std::net::{Ipv4Addr, SocketAddrV4};

    // tuple struct 
    #[derive(Debug, Clone)]
    pub struct Peers(pub Vec<SocketAddrV4>);
    // unit struct
    // Unit structs are useful when you need to implement a trait on something, but don’t need to store any data inside it
    // https://levelup.gitconnected.com/rust-unit-structs-explained-4ad2307efa72
    // https://www.reddit.com/r/rust/comments/lmekck/unit_struct_usage/
    struct PeersVisitor;


    impl<'de> Visitor<'de> for PeersVisitor {
        type Value = Peers;


        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("6 bytes the first 4 bytes are the peer's IP address and the last two are the peer's port number")
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            // first check if the length of the array of u8 is divisible equally by 6
            if v.len() % 6 != 0 {
                return Err(E::custom(format!("length is {}", v.len())));
            }
            Ok(Peers(
                // create an iterator that will loop over the array, splitting the array by 6 items
                // This means that each array will be of size 6 or will have 6 items
                v.chunks_exact(6)
                    // for each array of 6 elements do return the follwing
                    .map(|slice_6| {
                        // create a new SocketAddrV4 instance
                        SocketAddrV4::new(
                            // grab the first four items of the slice and pass each element as an
                            // arguremennt to the ipv4addr so that it can create an ip
                            Ipv4Addr::new(slice_6[0], slice_6[1], slice_6[2], slice_6[3]),
                            // create a u16 in big endian from the rest of the slice items
                            u16::from_be_bytes([slice_6[4], slice_6[5]]),
                        )
                    })
                    // after that return a vector
                    .collect(),
            ))
        }
    }

    impl<'de> Deserialize<'de> for Peers {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_bytes(PeersVisitor)
        }
    }

    impl Serialize for Peers {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut single_slice = Vec::with_capacity(6 * self.0.len());
            for peer in &self.0 {
                single_slice.extend(peer.ip().octets());
                single_slice.extend(peer.port().to_be_bytes());
            }
            serializer.serialize_bytes(&single_slice)
        }
    }
}
