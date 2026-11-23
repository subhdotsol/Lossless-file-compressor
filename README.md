# Huffman Lossless File Compressor

A command-line file compressor written in Rust using the Huffman Coding algorithm.

I'm building this weekend project from scratch to understand how lossless compression works and how Huffman Coding can reduce file sizes without losing any data. No external compression libraries are used.

## Features

- File compression using Huffman Coding
- File decompression
- Frequency analysis
- Huffman tree construction
- Binary encoding and decoding
- Support for different file types
- Preserve original file contents

### Compress a file

huffman compress input.txt output.huff

### Decompress a file

    huffman decompress output.huff restored.txt

### How It Works

Huffman Coding is a lossless compression algorithm that assigns shorter binary codes to frequently occurring bytes and longer codes to less frequently occurring bytes. The compressor reads the input file, counts how often each byte appears, and uses those frequencies to build a Huffman tree. The tree is then used to generate binary codes for each byte, which are used to encode the original data. During decompression, the encoded data is decoded using the Huffman tree to recover the original file exactly as it was. The goal is to make the file smaller without changing or losing any of its original data.
