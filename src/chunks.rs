use std::io::{BufReader, Read};
use super::*;

use crate::chunk_type::ChunkType;
use crc::{Crc, CRC_32_ISO_HDLC};

struct Chunk {
    data: Vec<u8>,
    length: u32,
    chunk_type: ChunkType,
    crc: u32,
}

impl Chunk {
    fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let length = data.len() as u32;

        let bytes = &chunk_type
            .to_string()
            .as_bytes()
            .iter()
            .chain(data.as_slice().iter())
            .copied()
            .collect::<Vec<u8>>();
        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(&bytes);

        Chunk {
            chunk_type,
            data,
            length,
            crc,
        }
    }

    fn length(&self) -> u32 {
        self.data.len() as u32
    }

    fn data(&self) -> &[u8] {
        self.data.as_slice()
    }

    fn data_as_string(&self) -> Result<String> {
        match String::from_utf8(self.data.clone()) {
            Ok(string) => Ok(string),
            Err(_) => Err("Could not convert Data to String".into()),
        }
    }

    fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.to_string().as_bytes())
            .chain(self.data())
            .chain(self.crc().to_be_bytes().iter())
            .copied()
            .collect::<Vec<u8>>()
    }

    fn crc(&self) -> u32 {
        self.crc
    }
}

#[derive(Debug)]
struct ChunkTryFromLengthError;

impl std::fmt::Display for ChunkTryFromLengthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Length of message does not match provided length.")
    }
}


impl std::error::Error for ChunkTryFromLengthError {}

#[derive(Debug)]
struct ChunkTryFromCrcError;

impl std::fmt::Display for ChunkTryFromCrcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Provided Crc is not correct.")
    }
}

impl std::error::Error for ChunkTryFromCrcError {}

impl TryFrom<&[u8]> for Chunk {
    type Error = Box<dyn std::error::Error>;

    fn try_from(bytes: &[u8]) -> std::result::Result<Self, Self::Error> {
        let mut buffer: [u8; 4] = [0, 0, 0, 0];

        let bytes_length_without_crc = bytes.len()-4;
        let message_bytes = &bytes[8..bytes_length_without_crc];

        let mut chunk_reader = BufReader::new(&bytes[4..8]);

        chunk_reader.read_exact(&mut buffer)?;
        let chunk_type = ChunkType::try_from(buffer)?;

        let mut length_reader = BufReader::new(&bytes[0..4]);
        length_reader.read_exact(&mut buffer)?;

        let length = u32::from_be_bytes(buffer);

        let mut crc_reader = BufReader::new(&bytes[bytes_length_without_crc..]);
        crc_reader.read_exact(&mut buffer)?;

        let crc = u32::from_be_bytes(buffer);

        if length != message_bytes.len() as u32 {
            return Err(Box::new(ChunkTryFromLengthError));
        }

        if crc != Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(&bytes[4..bytes_length_without_crc]) {
            return Err(Box::new(ChunkTryFromCrcError));
        }

        Ok(Chunk {
            chunk_type,
            data: message_bytes.to_vec(),
            length,
            crc
        })
    }
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match String::from_utf8(self.data.clone()) {
            Ok(string) => write!(f, "{}", string),
            Err(_) => Err(std::fmt::Error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
