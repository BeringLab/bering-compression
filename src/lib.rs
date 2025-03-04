use serde::{de::DeserializeOwned, Serialize};
use std::io;

pub mod error;
pub use error::CompressorError;

/// A trait for types that can compress and decompress data.
pub trait TCompressor {
    fn compress(value: &[u8]) -> Result<Vec<u8>, CompressorError>;
    fn decompress(compressed: &[u8]) -> Result<Vec<u8>, CompressorError>;
}

/// Snappy compression implementation
pub struct SnappyCompressor;

impl TCompressor for SnappyCompressor {
    fn compress(mut value: &[u8]) -> Result<Vec<u8>, CompressorError> {
        let mut compressed = Vec::new();
        let mut encoder = snap::write::FrameEncoder::new(&mut compressed);

        io::copy(&mut value, &mut encoder)
            .map_err(|err| CompressorError::CompressionError(err.to_string()))?;

        drop(encoder);
        Ok(compressed)
    }

    fn decompress(compressed: &[u8]) -> Result<Vec<u8>, CompressorError> {
        let mut reader = snap::read::FrameDecoder::new(compressed);
        let mut decompressed = Vec::new();

        io::copy(&mut reader, &mut decompressed)
            .map_err(|err| CompressorError::DecompressionError(err.to_string()))?;

        Ok(decompressed)
    }
}

/// Default compressor type
pub type DefaultCompressor = SnappyCompressor;

/// A trait for types that can be compressed and decompressed
pub trait TCompressible: Serialize + DeserializeOwned {
    fn compress(&self) -> Result<Vec<u8>, CompressorError> {
        let serialized = serde_json::to_vec(self)
            .map_err(|err| CompressorError::SerializationError(err.to_string()))?;

        DefaultCompressor::compress(&serialized)
    }

    fn decompress(compressed: &[u8]) -> Result<Self, CompressorError> {
        let decompressed = DefaultCompressor::decompress(compressed)?;
        serde_json::from_slice(&decompressed)
            .map_err(|err| CompressorError::DeserializationError(err.to_string()))
    }

    fn compress_with<C: TCompressor>(&self) -> Result<Vec<u8>, CompressorError> {
        let serialized = serde_json::to_vec(self)
            .map_err(|err| CompressorError::SerializationError(err.to_string()))?;
        C::compress(&serialized)
    }

    fn decompress_with<C: TCompressor>(compressed: &[u8]) -> Result<Self, CompressorError> {
        let decompressed = C::decompress(compressed)?;
        serde_json::from_slice(&decompressed)
            .map_err(|err| CompressorError::DeserializationError(err.to_string()))
    }
}

impl<T: Serialize + DeserializeOwned> TCompressible for T {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestStruct {
        field1: String,
        field2: i32,
    }

    #[test]
    fn test_default_compression() {
        let test_struct = TestStruct {
            field1: "Hello".to_string(),
            field2: 42,
        };

        let compressed = test_struct.compress().unwrap();
        let decompressed: TestStruct = TCompressible::decompress(&compressed).unwrap();

        assert_eq!(test_struct, decompressed);
    }

    #[test]
    fn test_custom_compression() {
        let test_struct = TestStruct {
            field1: "World".to_string(),
            field2: 24,
        };

        let compressed = test_struct.compress_with::<SnappyCompressor>().unwrap();
        let decompressed: TestStruct =
            TCompressible::decompress_with::<SnappyCompressor>(&compressed).unwrap();

        assert_eq!(test_struct, decompressed);
    }
}
