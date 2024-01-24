use crate::Error;
use std::{fmt::Display, str::FromStr};

// ---------------------------------------
// ---------------  Errors ---------------
// ---------------------------------------
#[derive(Debug)]
pub enum ChunkTypeError {
    ByteLengthError(usize),
    InvalidCharacter,
}

impl std::error::Error for ChunkTypeError {}

impl Display for ChunkTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkTypeError::ByteLengthError(actual) => write!(
                f,
                "Expected 4 bytes but received {} when creating chunk type",
                actual
            ),
            ChunkTypeError::InvalidCharacter => {
                write!(f, "Input contains one or more invalid characters")
            }
        }
    }
}
// ---------------------------------------
// ---------------  ChunkType ------------
// ---------------------------------------
#[derive(PartialEq, Eq, Debug)]
pub struct ChunkType(pub [u8; 4]);

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = std::str::from_utf8(&self.0).map_err(|_| std::fmt::Error)?;
        write!(f, "{}", output)
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(ChunkType(value))
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(Box::new(ChunkTypeError::ByteLengthError(s.len())));
        }
        let mut ret: [u8; 4] = [0; 4];
        ret.copy_from_slice(&s.as_bytes()[..4]);

        let chars_valid = s.as_bytes().iter().all(|&byte| byte.is_ascii_alphabetic());
        if !chars_valid {
            return Err(Box::new(ChunkTypeError::InvalidCharacter));
        }
        return Ok(ChunkType(ret));
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.0
    }
    pub fn is_critical(&self) -> bool {
        let char = char::from(self.0[0]);
        if char.is_lowercase() {
            return false;
        } else if char.is_uppercase() {
            return true;
        } else {
            return false;
        }
    }

    pub fn is_public(&self) -> bool {
        let char = char::from(self.0[1]);
        if char.is_lowercase() {
            return false;
        } else if char.is_uppercase() {
            return true;
        } else {
            return false;
        }
    }
    pub fn is_reserved_bit_valid(&self) -> bool {
        let char = char::from(self.0[2]);
        if !char.is_alphabetic() {
            return false;
        }
        if char.is_lowercase() {
            return false;
        } else if char.is_uppercase() {
            return true;
        } else {
            return false;
        }
    }
    pub fn is_safe_to_copy(&self) -> bool {
        let char = char::from(self.0[3]);

        if char.is_lowercase() {
            true
        } else if char.is_uppercase() {
            false
        } else {
            false
        }
    }

    pub fn is_valid(&self) -> bool {
        let valid_chars = self
            .0
            .iter()
            .all(|&b| (b >= b'a' && b <= b'z' || (b >= b'A' && b <= b'Z')));
        valid_chars && self.is_reserved_bit_valid()
    }
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

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
