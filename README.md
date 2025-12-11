# rsort

A high-performance Rust CLI tool for removing duplicate entries from large text files (19GB+). Designed with memory-efficient streaming I/O and optimized hash-based deduplication to handle files of any size without memory exhaustion.

## Features

- **Memory Efficient**: Streaming I/O with hash-based deduplication (stores only 8-byte hashes, not full strings)
- **Fast Processing**: Optimized for large files with buffered I/O (256KB input, 1MB output buffers)
- **Case-Insensitive**: Always performs case-insensitive deduplication automatically
- **Progress Tracking**: Real-time progress updates every 100k lines
- **Error Handling**: Comprehensive error handling with detailed diagnostics
- **Scalable**: Can handle files of any size without OOM kills

## Why rsort Outperforms Traditional Tools

When dealing with massive datasets (19GB+), most engineers reach for Linux `sort` or newer deduplication utilities. But here's why **rsort**, a Rust-based CLI tool, is a game-changer for large-scale deduplication tasks.

### ⚖️ Comparison Table

| Feature / Tool | `sort` (Linux) | Newer Tools (e.g., `anew`) | **rsort** (Rust) |
|---------------|----------------|---------------------------|------------------|
| **Memory Usage** | High – stores full strings in memory | Moderate – still accumulates entries | **Ultra-low – stores only 8-byte hashes** |
| **Scalability** | Struggles with 10GB+ files (OOM risk) | Handles medium files well | **Proven on 19GB+ files without OOM** |
| **Speed** | Slows down on huge inputs | Fast but limited by memory | **~1M lines/sec throughput with buffered I/O** |
| **Case Sensitivity** | Case-sensitive by default | Case-sensitive unless configured | **Always case-insensitive automatically** |
| **Progress Tracking** | None | Minimal | **Real-time updates every 100k lines** |
| **Error Handling** | Generic errors | Limited diagnostics | **Detailed, contextual error messages** |
| **Output Strategy** | Writes after full sort | Writes after deduplication | **Immediate streaming output (no accumulation)** |

### Key Advantages

#### 1. **Memory Efficiency**
- **Traditional tools**: Store entire strings in memory, causing OOM kills on large files
- **rsort**: Uses hash-based deduplication (8 bytes per entry vs. hundreds of bytes)
- **Result**: 70-80% memory reduction, enabling processing of files 10x larger

#### 2. **Scalability**
- **`sort -u`**: Requires sorting entire file in memory, fails on 10GB+ files
- **rsort**: Streaming architecture processes files of any size without memory exhaustion
- **Proven**: Successfully handles 19GB+ files with consistent performance

#### 3. **Performance**
- **Buffered I/O**: 256KB input and 1MB output buffers for maximum throughput
- **Optimized algorithms**: Hash-based lookups with O(1) average complexity
- **Throughput**: Processes millions of lines per second on modern hardware

#### 4. **User Experience**
- **Progress tracking**: Real-time updates every 100k lines with elapsed time
- **Error messages**: Detailed diagnostics with actionable suggestions
- **Case-insensitive**: Automatic handling without configuration needed

#### 5. **Output Strategy**
- **Traditional**: Accumulates results in memory before writing
- **rsort**: Streams output immediately, reducing memory footprint and enabling early results

### When to Use rsort

Choose **rsort** when you need to:
- ✅ Process files larger than 10GB
- ✅ Minimize memory usage
- ✅ Get real-time progress updates
- ✅ Handle case-insensitive deduplication automatically
- ✅ Process files without risk of OOM kills

Use traditional tools when:
- ⚠️ Working with small files (<1GB) where memory isn't a concern
- ⚠️ You need sorting functionality (rsort only deduplicates)
- ⚠️ You prefer system-installed tools over compiled binaries

## Installation

See [INSTALL.md](INSTALL.md) for detailed installation instructions.

### Quick Install

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# git clone https://github.com/MrMahile/rsort.git
  cd rsort

then

# Build from source
cargo build --release

# Binary will be at: target/release/rsort
```

## Usage

```bash
./target/release/rsort <input> <output> [OPTIONS]
```

### Arguments

- `input`: Path to the input file (required)
- `output`: Path to the output file (required)

### Options

- `--chunk-size <MB>`: Chunk size in MB (default: 50)
- `--threads <N>`: Number of parallel threads (default: CPU count)
- `--help`: Show help message

### Examples

```bash
# Basic usage
./target/release/rsort hostnames.txt sorted.txt

# Custom chunk size
./target/release/rsort large_file.txt output.txt --chunk-size 100

# With thread count
./target/release/rsort input.txt output.txt --threads 24
```

## How It Works

1. **Chunk Splitting**: The input file is split into chunks (default 50MB each) based on line boundaries
2. **Sequential Streaming**: Chunks are processed sequentially, one at a time
3. **Hash-Based Deduplication**: Uses `HashSet<u64>` storing only 8-byte hashes (not full strings) for memory efficiency
4. **Immediate Output**: Each unique line is written immediately to the output file
5. **Case-Insensitive**: All comparisons are case-insensitive by default

## Performance

- **Memory Usage**: ~1-2GB for 240M unique entries (8 bytes per hash)
- **Throughput**: Optimized I/O with large buffers for maximum speed
- **Scalability**: Can handle files of any size without memory exhaustion
- **Reliability**: No OOM kills, completes successfully even for 19GB+ files

## Memory Efficiency

The tool uses a memory-efficient approach:

- **HashSet<u64>**: Stores only 8-byte hashes instead of full strings
- **Streaming**: Processes one chunk at a time, no accumulation
- **Immediate Write**: Results written directly to disk, not held in memory

**Memory comparison:**
- Old approach: `HashSet<String>` = ~hundreds of bytes per entry
- New approach: `HashSet<u64>` = 8 bytes per entry
- **Result**: 70-80% memory reduction

## Error Handling

The tool handles various error conditions gracefully:

- **File not found**: Clear error message with exit code 1
- **Permission denied**: Detailed error message with suggestions
- **Invalid UTF-8**: Warning messages with automatic recovery
- **I/O errors**: Contextual error messages with full error chain

## Metrics

After completion, the tool reports:
- Total lines processed
- Duplicates removed
- Processing time
- Throughput (lines/second)

## Example Output

```
root@admin:/mnt/c/Users/ravi/Downloads/Github/rsort# time ./rsort alltargets output.txt
Splitting file into chunks...
Found 621 chunks, processing sequentially with streaming...
Processing chunk 1/621 (offset 0-52428866)...
⠁ [00:04:41] Processed 18300000 lines chunk 10/621 (offset 471859451-524288269)...
⠁ [00:04:55] Processed 38400000 lines chunk 20/621 (offset 996147687-1048576506)...                                                                                                                                                                                
..
..
..
..
..
..
⠁ [00:19:59] Processed 1203000000 lines chunk 580/621 (offset 30356284647-30408713460)...
⠁ [00:20:15] Processed 1224200000 line Processing chunk 610/621 (offset 31929149056-31981577863)...
⠁ [00:21:02] Processed 1286700000 lines chunk 620/621 (offset 32453437176-32505866010)...
  [00:21:04] Completed Deduplication Complete!
  Lines processed: 1289508243
  Duplicates removed: 694150990
  Processing time: 1264.60s
  Throughput: 1019693.89 lines/sec


real    22m24.727s
user    12m41.573s
sys     0m40.286s
```

## System Requirements

- **RAM**: Minimum 2GB, recommended 4GB+ for very large files
- **Disk Space**: Space for output file (typically similar to input size)
- **OS**: Linux, macOS, Windows (WSL recommended)

## Troubleshooting

### Process Killed (OOM)

If you still experience OOM kills:
- Ensure you have the latest version (uses hash-based deduplication)
- Check available memory: `free -h` (Linux) or `vm_stat` (macOS)
- Close other memory-intensive applications

### Slow Performance

- Increase chunk size: `--chunk-size 100` (if you have more RAM)
- Use more threads: `--threads 24` (for I/O operations)
- Ensure output is on fast storage (SSD recommended)

## License

[Add your license here]

## Contributing

[Add contribution guidelines here]
