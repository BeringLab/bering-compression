use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompressorError {
    #[error("Compression error: {0}")]
    CompressionError(String),
    #[error("Decompression error: {0}")]
    DecompressionError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
}
