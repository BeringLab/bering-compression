use crate::error::CompressorError;

pub mod snappy;

pub use snappy::SnappyCompressor;

pub trait TCompressor {
    fn compress(&self, value: &[u8]) -> Result<Vec<u8>, CompressorError>;
    fn decompress(&self, compressed: &[u8]) -> Result<Vec<u8>, CompressorError>;
}

/// Compression algorithm types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionAlgorithm {
    Snappy,
    // Gzip or something
}

pub struct CompressorFactory;

impl CompressorFactory {
    pub fn get_compressor(algorithm: CompressionAlgorithm) -> Box<dyn TCompressor> {
        match algorithm {
            CompressionAlgorithm::Snappy => Box::new(SnappyCompressor),
        }
    }
}

pub type DefaultCompressor = SnappyCompressor;
