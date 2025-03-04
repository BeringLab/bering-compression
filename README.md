# Bering Compression Library

A simple and efficient data compression library with support for multiple compression algorithms.

## Features

- Multiple compression algorithm support:
  - Snappy compression (default)
  - Gzip compression (TO-BE)
- Compression algorithm factory pattern
- Serialization/deserialization integration (using serde_json)
- Extensible design (easily add new compression algorithms)
- Feature flags for conditional compilation (TO-BE)

## Usage

### Add dependency to Cargo.toml

```toml
[dependencies]
bering-compression = { git = "https://github.com/BeringLab/bering-compression.git", branch = "main" }

# (TO-BE)
# Optional: Enable specific compression algorithms
# bering-compression = { git = "https://github.com/BeringLab/bering-compression.git", branch = "main", features = ["myalgo"] }

# Optional: Enable all compression algorithms
# bering-compression = { git = "https://github.com/BeringLab/bering-compression.git", branch = "main", features = ["all"] }
```

### Basic Usage

```rust
use bering_compression::TCompressible;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct MyData {
    field1: String,
    field2: i32,
}

fn main() {
    let data = MyData {
        field1: "Hello Beringlab".to_string(),
        field2: 42,
    };

    // Compress data using default algorithm (Snappy)
    let compressed = data.compress().unwrap();
    
    // Decompress data
    let decompressed: MyData = TCompressible::decompress(&compressed).unwrap();
    
    assert_eq!(data, decompressed);
}
```

### Using Specific Compressor Instance

```rust
use bering_compression::{TCompressible, SnappyCompressor};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct MyData {
    field1: String,
    field2: i32,
}

fn main() {
    let data = MyData {
        field1: "Hello Beringlab".to_string(),
        field2: 42,
    };

    // Create a compressor instance
    let compressor = SnappyCompressor;
    
    // Compress data with specific compressor
    let compressed = data.compress_with(&compressor).unwrap();
    
    // Decompress data with specific compressor
    let decompressed: MyData = TCompressible::decompress_with(&compressed, &compressor).unwrap();
    
    assert_eq!(data, decompressed);
}
```

### Using Compression Algorithm Enum (which is Partially implemented)

```rust
use bering_compression::{TCompressible, CompressionAlgorithm};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct MyData {
    field1: String,
    field2: i32,
}

fn main() {
    let data = MyData {
        field1: "Hello Beringlab".to_string(),
        field2: 42,
    };

    // Compress data with specific algorithm
    let compressed = data.compress_with_algorithm(CompressionAlgorithm::Snappy).unwrap();
    
    // Decompress data with specific algorithm
    let decompressed: MyData = TCompressible::decompress_with_algorithm(&compressed, CompressionAlgorithm::Snappy).unwrap();
    
    assert_eq!(data, decompressed);
    
    // Below Code is not implemented yet (TO-BE)
    /*
    // When zstd feature is enabled
    #[cfg(feature = "bzip2")]
    {
        // 
    }
    */
}
```

### Implementing Custom Compressor

```rust
use bering_compression::{TCompressor, CompressorError};
use std::io;

// Define your custom compressor
struct MyCustomCompressor;

impl TCompressor for MyCustomCompressor {
    fn compress(&self, value: &[u8]) -> Result<Vec<u8>, CompressorError> {
        // Implement compression logic
        // ...
        Ok(value.to_vec()) // Example (no actual compression)
    }

    fn decompress(&self, compressed: &[u8]) -> Result<Vec<u8>, CompressorError> {
        // Implement decompression logic
        // ...
        Ok(compressed.to_vec()) // Example (no actual decompression)
    }
}

// To use your custom compressor:
// let compressor = MyCustomCompressor;
// let compressed = my_data.compress_with(&compressor).unwrap();
```

### Adding a New Compression Algorithm to the Library (TO-BE)

To add a new compression algorithm to the library:

1. Create a new module in `src/compressors/` (e.g., `my_algo.rs`)
2. Implement the `TCompressor` trait for your algorithm
3. Add your algorithm to the `CompressionAlgorithm` enum in `src/compressors/mod.rs`
4. Update the `CompressorFactory::get_compressor` method to handle your algorithm
5. Add appropriate feature flags in `Cargo.toml`

Example (TO-BE):

```rust
// src/compressors/my_algo.rs
use super::TCompressor;
use crate::error::CompressorError;
use std::io;

pub struct MyAlgoCompressor;

impl TCompressor for MyAlgoCompressor {
    fn compress(&self, value: &[u8]) -> Result<Vec<u8>, CompressorError> {
        // Implement compression using your algorithm
        // ...
    }

    fn decompress(&self, compressed: &[u8]) -> Result<Vec<u8>, CompressorError> {
        // Implement decompression using your algorithm
        // ...
    }
}
```

```rust
// Update src/compressors/mod.rs (TO-BE)
pub mod my_algo;
pub use my_algo::MyAlgoCompressor;

pub enum CompressionAlgorithm {
    Snappy,
    MyAlgo, // Add your algorithm
}

impl CompressorFactory {
    pub fn get_compressor(algorithm: CompressionAlgorithm) -> Box<dyn TCompressor> {
        match algorithm {
            CompressionAlgorithm::Snappy => Box::new(SnappyCompressor),
            CompressionAlgorithm::MyAlgo => Box::new(my_algo::MyAlgoCompressor), // Add algorithm
        }
    }
}
```

### Error Handling

```rust
use bering_compression::{TCompressible, CompressorError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct MyData {
    field1: String,
    field2: i32,
}

fn main() -> Result<(), CompressorError> {
    let data = MyData {
        field1: "Hello Beringlab".to_string(),
        field2: 42,
    };

    // Compress data
    let compressed = data.compress()?;
    
    // Decompress data
    let decompressed: MyData = TCompressible::decompress(&compressed)?;
    
    assert_eq!(data, decompressed);
    Ok(())
}
```

## License

MIT 