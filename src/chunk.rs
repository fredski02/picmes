use std::fmt::Display;

use crate::{Error, Result};
use crc::{Crc, CRC_32_ISO_HDLC};
pub const CHECK_SUM_32: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

use crate::chunk_type::ChunkType;

#[derive(Debug)]
pub enum ChunkError {
    InvalidInput(String),
    InvalidChunkType,
    InvalidCheckSum(u32, u32),
}

impl std::error::Error for ChunkError {}

impl Display for ChunkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput(s) => write!(f, "{}", s),
            Self::InvalidChunkType => write!(f, "A chunk contains an invliad chunk type"),
            Self::InvalidCheckSum(expected, actual) => write!(
                f,
                "The checksum should be '{}' but found '{}' instead",
                expected, actual
            ),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Chunk {
    pub chunk_data: Vec<u8>,
    pub chunk_type: ChunkType,
}

impl Chunk {
    pub const LEN_DATA_LENGTH: usize = 4;
    pub const CHUNK_TYPE_LENGTH: usize = 4;
    pub const CRC_LENGTH: usize = 4;

    pub const META_DATA_LENGTH: usize =
        Chunk::LEN_DATA_LENGTH + Chunk::CHUNK_TYPE_LENGTH + Chunk::CRC_LENGTH;

    pub fn new(chunk_type: ChunkType, chunk_data: Vec<u8>) -> Self {
        Self {
            chunk_type,
            chunk_data,
        }
    }

    pub fn length(&self) -> usize {
        self.chunk_data.len()
    }

    /// Chunk type
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn crc(&self) -> u32 {
        let b: Vec<u8> = self
            .chunk_type
            .bytes()
            .iter()
            .chain(self.chunk_data.iter())
            .copied()
            .collect();
        CHECK_SUM_32.checksum(&b)
    }

    /// Entire chunk represented as bytes
    pub fn as_bytes(&self) -> Vec<u8> {
        let data_length = self.chunk_data.len() as u32;
        data_length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.chunk_data.iter())
            .chain(self.crc().to_be_bytes().iter())
            .copied()
            .collect()
    }

    pub fn data_as_string(&self) -> Result<String> {
        let s = std::str::from_utf8(&self.chunk_data)?;
        Ok(s.to_string())
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self.data_as_string())?;
        Ok(())
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    // Chunk layout ( in order )
    // -- length - 4 bytes
    // -- chunk type - 4 bytes
    // -- data - N bytes
    // -- crc checksum - 4 bytes

    fn try_from(value: &[u8]) -> Result<Self> {
        if value.len() < Chunk::META_DATA_LENGTH {
            return Err(Box::new(ChunkError::InvalidInput(
                "Chunk is too small".to_string(),
            )));
        }

        let (data_length, rest) = value.split_at(Chunk::LEN_DATA_LENGTH);
        let data_length = u32::from_be_bytes(data_length.try_into()?) as usize;

        let (type_slice, rest) = rest.split_at(Chunk::CHUNK_TYPE_LENGTH);
        let chunk_type_b: [u8; 4] = type_slice.try_into()?;
        let chunk_type = ChunkType::try_from(chunk_type_b)?;

        if !chunk_type.is_valid() {
            return Err(Box::new(ChunkError::InvalidChunkType));
        }

        // good up to now
        let (data_slice, rest) = rest.split_at(data_length);
        let (crc_slice, _) = rest.split_at(Chunk::CRC_LENGTH);

     
        let new_chunk = Self {
            chunk_type,
            chunk_data: data_slice.into(),
        };

        let new_crc = new_chunk.crc();
        let expected_crc = u32::from_be_bytes(crc_slice.try_into()?);

        if new_crc != expected_crc {
            return Err(Box::new(ChunkError::InvalidCheckSum(expected_crc, new_crc)));
        }

        Ok(new_chunk)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data: Vec<u8> = "This is where your secret message will be!"
            .bytes()
            .collect();
        Chunk::new(chunk_type, data)
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
        let chunk_type = b"RuSt";
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
