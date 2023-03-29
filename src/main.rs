use std::io::{BufReader, Read};

use crc::{Crc, CRC_32_ISO_HDLC};

mod args;
mod chunk_type;
mod chunks;
mod commads;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    let chunk_type = "RuSt".as_bytes();
    let message_bytes = "This is where your secret message will be!".as_bytes();

    let chunk_data: Vec<u8> = chunk_type
        .iter()
        .chain(message_bytes.iter())
        .copied()
        .collect();

    let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(&chunk_data);
    println!("CRC is: {}", crc);
    // todo!();
}
