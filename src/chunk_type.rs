use std::{fmt::Display, str::FromStr};

#[derive(PartialEq, Eq, Debug)]
struct ChunkType([u8; 4]);

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.0[0], self.0[1], self.0[2], self.0[3])
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(ChunkType(value))
    }
}

// impl TryFrom<&str> for ChunkType {
//     type Error = &'static str;

//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         if value.len() != 4 {
//             return Err("ChunkType must be 4 bytes in length");
//         }
//         let mut ret : [u8;4] = [0; 4];
//         ret.copy_from_slice(&value.as_bytes()[..4]);
//         return Ok(ChunkType(ret));
//     }
// }

impl FromStr for ChunkType {
    type Err = &'static str; 

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err("ChunkType must be 4 bytes in length");
        }
        let mut ret : [u8;4] = [0; 4];
        ret.copy_from_slice(&s.as_bytes()[..4]);
        return Ok(ChunkType(ret));
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.0
    }
    // pub fn is_valid(&self) -> bool {}
    pub fn is_critical(&self) -> bool {}
    // pub fn is_pubic(&self) -> bool {}
    // pub fn is_reserved_bit_valid(&self) -> bool {}
    // pub fn is_safe_to_copy(&self) -> bool {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    // #[test]
    // pub fn test_chunk_type_is_not_critical() {
    //     let chunk = ChunkType::from_str("ruSt").unwrap();
    //     assert!(!chunk.is_critical());
    // }

    // #[test]
    // pub fn test_chunk_type_is_public() {
    //     let chunk = ChunkType::from_str("RUSt").unwrap();
    //     assert!(chunk.is_public());
    // }

    // #[test]
    // pub fn test_chunk_type_is_not_public() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert!(!chunk.is_public());
    // }

    // #[test]
    // pub fn test_chunk_type_is_reserved_bit_valid() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert!(chunk.is_reserved_bit_valid());
    // }

    // #[test]
    // pub fn test_chunk_type_is_reserved_bit_invalid() {
    //     let chunk = ChunkType::from_str("Rust").unwrap();
    //     assert!(!chunk.is_reserved_bit_valid());
    // }

    // #[test]
    // pub fn test_chunk_type_is_safe_to_copy() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert!(chunk.is_safe_to_copy());
    // }

    // #[test]
    // pub fn test_chunk_type_is_unsafe_to_copy() {
    //     let chunk = ChunkType::from_str("RuST").unwrap();
    //     assert!(!chunk.is_safe_to_copy());
    // }

    // #[test]
    // pub fn test_valid_chunk_is_valid() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert!(chunk.is_valid());
    // }

    // #[test]
    // pub fn test_invalid_chunk_is_valid() {
    //     let chunk = ChunkType::from_str("Rust").unwrap();
    //     assert!(!chunk.is_valid());

    //     let chunk = ChunkType::from_str("Ru1t");
    //     assert!(chunk.is_err());
    // }

    // #[test]
    // pub fn test_chunk_type_string() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert_eq!(&chunk.to_string(), "RuSt");
    // }

    // #[test]
    // pub fn test_chunk_type_trait_impls() {
    //     let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
    //     let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
    //     let _chunk_string = format!("{}", chunk_type_1);
    //     let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    // }
}
