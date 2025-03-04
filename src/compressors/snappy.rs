use super::TCompressor;
use crate::error::CompressorError;
use std::io;

pub struct SnappyCompressor;

impl TCompressor for SnappyCompressor {
    fn compress(&self, mut value: &[u8]) -> Result<Vec<u8>, CompressorError> {
        let mut compressed = Vec::new();
        let mut encoder = snap::write::FrameEncoder::new(&mut compressed);

        io::copy(&mut value, &mut encoder)
            .map_err(|err| CompressorError::CompressionError(err.to_string()))?;

        drop(encoder);
        Ok(compressed)
    }

    fn decompress(&self, compressed: &[u8]) -> Result<Vec<u8>, CompressorError> {
        let mut reader = snap::read::FrameDecoder::new(compressed);
        let mut decompressed = Vec::new();

        io::copy(&mut reader, &mut decompressed)
            .map_err(|err| CompressorError::DecompressionError(err.to_string()))?;

        Ok(decompressed)
    }
}
