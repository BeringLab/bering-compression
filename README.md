# Bering Compression Library

A simple and efficient data compression library. This library uses the Snappy compression algorithm to compress and decompress data.

## Features

- Snappy compression algorithm support
- Serialization/deserialization integration (using serde_json)
- Simple API
- Extensible design (ability to add other compression algorithms)
- Modular error handling

## Usage

### Add dependency to Cargo.toml

```toml
[dependencies]
compression = { git = "https://github.com/BeringLab/compression.git", branch = "main" }
```

### Basic Usage

```rust
use compression::TCompressible;
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

    // Compress data
    let compressed = data.compress().unwrap();
    
    // Decompress data
    let decompressed: MyData = TCompressible::decompress(&compressed).unwrap();
    
    assert_eq!(data, decompressed);
}
```

### Using Custom Compressor

```rust
use compression::{TCompressible, SnappyCompressor};
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

    // Compress data with specific compressor
    let compressed = data.compress_with::<SnappyCompressor>().unwrap();
    
    // Decompress data with specific compressor
    let decompressed: MyData = TCompressible::decompress_with::<SnappyCompressor>(&compressed).unwrap();
    
    assert_eq!(data, decompressed);
}
```

### Implementing Custom Compressor

```rust
use compression::{TCompressor, CompressorError};

struct MyCustomCompressor;

impl TCompressor for MyCustomCompressor {
    fn compress(value: &[u8]) -> Result<Vec<u8>, CompressorError> {
        // Implement compression logic
        // ...
    }

    fn decompress(compressed: &[u8]) -> Result<Vec<u8>, CompressorError> {
        // Implement decompression logic
        // ...
    }
}
```

### Error Handling

```rust
use compression::{TCompressible, CompressorError};
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