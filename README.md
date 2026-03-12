## Compress File (Rust)

This is a simple Rust project to compress files, created for learning purposes.

### Features
- Compresses files using Rust libraries
- Easy to use CLI

### Usage
1. Build the project:
   ```bash
   cargo build --release
   ```
2. Run the compressor:
   ```bash
   ./target/release/compress_file <input_file> <output_file>
   ```

### Proof It Works
Compressed a file from **14MB** to **13MB**.

> Example:
> Original size: 14MB
> Compressed size: 13MB

![Compression Proof](the_proof.png)

---
Made for learning Rust file operations and compression.