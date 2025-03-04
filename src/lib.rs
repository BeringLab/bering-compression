use serde::{de::DeserializeOwned, Serialize};

pub mod compressors;
pub mod error;

pub use compressors::{
    CompressionAlgorithm, CompressorFactory, DefaultCompressor, SnappyCompressor, TCompressor,
};
pub use error::CompressorError;

/// A trait for types that can be compressed and decompressed
pub trait TCompressible: Serialize + DeserializeOwned {
    fn compress(&self) -> Result<Vec<u8>, CompressorError> {
        let serialized = serde_json::to_vec(self)
            .map_err(|err| CompressorError::SerializationError(err.to_string()))?;

        let compressor = SnappyCompressor;
        compressor.compress(&serialized)
    }

    fn decompress(compressed: &[u8]) -> Result<Self, CompressorError> {
        let compressor = SnappyCompressor;
        let decompressed = compressor.decompress(compressed)?;

        serde_json::from_slice(&decompressed)
            .map_err(|err| CompressorError::DeserializationError(err.to_string()))
    }

    fn compress_with<C: TCompressor>(&self, compressor: &C) -> Result<Vec<u8>, CompressorError> {
        let serialized = serde_json::to_vec(self)
            .map_err(|err| CompressorError::SerializationError(err.to_string()))?;

        compressor.compress(&serialized)
    }

    fn decompress_with<C: TCompressor>(
        compressed: &[u8],
        compressor: &C,
    ) -> Result<Self, CompressorError> {
        let decompressed = compressor.decompress(compressed)?;

        serde_json::from_slice(&decompressed)
            .map_err(|err| CompressorError::DeserializationError(err.to_string()))
    }

    fn compress_with_algorithm(
        &self,
        algorithm: CompressionAlgorithm,
    ) -> Result<Vec<u8>, CompressorError> {
        let compressor = CompressorFactory::get_compressor(algorithm);
        let serialized = serde_json::to_vec(self)
            .map_err(|err| CompressorError::SerializationError(err.to_string()))?;

        compressor.compress(&serialized)
    }

    fn decompress_with_algorithm(
        compressed: &[u8],
        algorithm: CompressionAlgorithm,
    ) -> Result<Self, CompressorError> {
        let compressor = CompressorFactory::get_compressor(algorithm);
        let decompressed = compressor.decompress(compressed)?;

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

        let compressor = SnappyCompressor;
        let compressed = test_struct.compress_with(&compressor).unwrap();
        let decompressed: TestStruct =
            TCompressible::decompress_with(&compressed, &compressor).unwrap();

        assert_eq!(test_struct, decompressed);
    }

    #[test]
    fn test_compression_with_algorithm() {
        let test_struct = TestStruct {
            field1: "Algorithm".to_string(),
            field2: 100,
        };

        let compressed = test_struct
            .compress_with_algorithm(CompressionAlgorithm::Snappy)
            .unwrap();
        let decompressed: TestStruct =
            TCompressible::decompress_with_algorithm(&compressed, CompressionAlgorithm::Snappy)
                .unwrap();

        assert_eq!(test_struct, decompressed);
    }
}
