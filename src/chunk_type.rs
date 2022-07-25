use std::fmt;
use std::str::{FromStr, from_utf8};
use std::convert::TryFrom;



#[derive(Debug)]
pub struct ChunkType{chunks : [u8; 4]}

impl TryFrom<[u8; 4]> for ChunkType {

    type Error = &'static str;

    fn try_from(values: [u8; 4]) -> Result<Self, Self::Error> {
        if values[2] == 49 {
            return Err("Third byte cannot be 1")
        }
        Ok(ChunkType {chunks: values})
    }
}

impl FromStr for ChunkType {

    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s0 = s.as_bytes()[0];
        let s1 = s.as_bytes()[1];
        let s2 = s.as_bytes()[2];
        let s3 = s.as_bytes()[3];
        
        ChunkType::try_from([s0, s1, s2, s3])
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", from_utf8(&self.chunks).unwrap())
    }
}

impl PartialEq for ChunkType {
    fn eq(&self, other: &Self) -> bool {
        self.chunks[0] == other.chunks[0] ||
        self.chunks[1] == other.chunks[1] ||
        self.chunks[2] == other.chunks[2] ||
        self.chunks[3] == other.chunks[3]
    }
}

impl Eq for ChunkType {}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.chunks
    }
    fn is_valid(&self) -> bool {
        self.chunks[2].is_ascii_uppercase()
    }
    fn is_critical(&self) -> bool {
        self.chunks[0].is_ascii_uppercase()
    }
    fn is_public(&self) -> bool {
        self.chunks[1].is_ascii_uppercase()
    }
    fn is_reserved_bit_valid(&self) -> bool {
        self.chunks[2].is_ascii_uppercase()
    }
    fn is_safe_to_copy(&self) -> bool {
        self.chunks[3].is_ascii_lowercase()
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

