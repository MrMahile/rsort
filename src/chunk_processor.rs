use hashbrown::HashSet;
use std::io::{BufRead, BufReader, Read, Seek, Write};
use std::fs::File;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use anyhow::{Result, Context};

pub struct Chunk {
    pub start_offset: u64,
    pub end_offset: u64,
}

/// Get the size of a file in bytes
pub fn get_file_size(file_path: &std::path::Path) -> Result<u64> {
    let metadata = std::fs::metadata(file_path)
        .with_context(|| format!("Failed to get file metadata: {}", file_path.display()))?;
    Ok(metadata.len())
}

/// Compute a u64 hash of a string (case-insensitive)
fn hash_string(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.to_lowercase().hash(&mut hasher);
    hasher.finish()
}

pub fn process_chunk_stream<W: Write>(
    file_path: &std::path::Path,
    chunk: &Chunk,
    writer: &mut W,
    global_seen: &mut HashSet<u64>,
    progress: &crate::progress::ProgressTracker,
) -> Result<()> {
    // Validate chunk boundaries against file size
    let file_size = get_file_size(file_path)
        .with_context(|| format!("Failed to get file size: {}", file_path.display()))?;
    
    if chunk.start_offset > file_size {
        return Err(anyhow::anyhow!(
            "Chunk start offset {} exceeds file size {} (file: {})",
            chunk.start_offset,
            file_size,
            file_path.display()
        ));
    }
    
    if chunk.end_offset > file_size {
        eprintln!(
            "Warning: Chunk end offset {} exceeds file size {}, adjusting to file size (file: {})",
            chunk.end_offset,
            file_size,
            file_path.display()
        );
    }
    
    let file = File::open(file_path)
        .with_context(|| format!("Failed to open file: {}", file_path.display()))?;
    // Use larger buffer for better I/O performance (256KB)
    let mut reader = BufReader::with_capacity(256 * 1024, file);
    
    // Seek to start offset
    reader.seek(std::io::SeekFrom::Start(chunk.start_offset))
        .with_context(|| format!(
            "Failed to seek to offset {} (file size: {}, file: {})",
            chunk.start_offset,
            file_size,
            file_path.display()
        ))?;
    
    let mut current_offset = chunk.start_offset;
    
    loop {
        if current_offset >= chunk.end_offset {
            break;
        }
        
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line)
            .with_context(|| format!(
                "Failed to read line at offset {} (chunk: {}-{}, file size: {}, file: {})",
                current_offset,
                chunk.start_offset,
                chunk.end_offset,
                file_size,
                file_path.display()
            ))?;
        
        if bytes_read == 0 {
            break;
        }
        
        current_offset += bytes_read as u64;
        
        // Skip empty lines or handle encoding issues
        if line.is_empty() && bytes_read == 1 {
            // Just a newline, skip
            continue;
        }
        
        progress.increment_lines(1);
        
        // Hash the lowercase string for case-insensitive comparison
        let key_lower = line.to_lowercase();
        let hash = hash_string(&key_lower);
        
        // Check against global HashSet and write immediately if unique
        if global_seen.insert(hash) {
            writer.write_all(line.as_bytes())
                .context("Failed to write line to output")?;
        } else {
            progress.increment_duplicates(1);
        }
        
        // Periodic flush to reduce bottlenecks
        if global_seen.len() % 100_000 == 0 {
            writer.flush().context("Failed to flush output buffer")?;
        }
    }
    
    Ok(())
}

pub fn find_chunk_boundaries(
    file_path: &std::path::Path,
    chunk_size_bytes: usize,
) -> Result<Vec<Chunk>> {
    let file = File::open(file_path)
        .with_context(|| format!("Failed to open file: {}", file_path.display()))?;
    let mut reader = BufReader::with_capacity(64 * 1024, file);
    
    let mut chunks = Vec::new();
    let mut current_offset = 0u64;
    let mut chunk_start = 0u64;
    let mut chunk_size = 0usize;
    
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => break, // EOF
            Ok(bytes_read) => {
                chunk_size += bytes_read;
                current_offset += bytes_read as u64;
            }
            Err(e) => {
                // Handle encoding errors - log warning and try to recover
                if e.kind() == std::io::ErrorKind::InvalidData {
                    eprintln!("Warning: Invalid UTF-8 sequence at offset {}. Attempting to recover.", current_offset);
                    // Read raw bytes until we find a newline or EOF
                    let mut buffer = vec![0u8; 1];
                    let mut found_newline = false;
                    let mut bytes_skipped = 0;
                    
                    // Skip bytes until we find a newline (0x0A) or EOF
                    while let Ok(1) = reader.read(&mut buffer) {
                        bytes_skipped += 1;
                        current_offset += 1;
                        chunk_size += 1;
                        if buffer[0] == b'\n' {
                            found_newline = true;
                            break;
                        }
                        if bytes_skipped > 10000 {
                            // Too many bytes skipped, likely corrupted file
                            eprintln!("Error: Too many invalid bytes encountered. File may be corrupted.");
                            break;
                        }
                    }
                    
                    if found_newline {
                        // Check if we need to finalize chunk
                        if chunk_size >= chunk_size_bytes {
                            chunks.push(Chunk {
                                start_offset: chunk_start,
                                end_offset: current_offset,
                            });
                            chunk_start = current_offset;
                            chunk_size = 0;
                        }
                        // Continue to next iteration of loop
                        continue;
                    } else {
                        // EOF or unrecoverable - exit loop
                        break;
                    }
                } else {
                    // Other I/O error
                    return Err(e).with_context(|| format!("I/O error at offset {}", current_offset));
                }
            }
        }
        
        // If chunk size exceeded, finalize current chunk and start new one
        if chunk_size >= chunk_size_bytes {
            chunks.push(Chunk {
                start_offset: chunk_start,
                end_offset: current_offset,
            });
            chunk_start = current_offset;
            chunk_size = 0;
        }
    }
    
    // End of file - add final chunk if it has content
    if chunk_size > 0 {
        chunks.push(Chunk {
            start_offset: chunk_start,
            end_offset: current_offset,
        });
    }
    
    // If no chunks were created (empty file or very small), create one chunk covering entire file
    if chunks.is_empty() {
        chunks.push(Chunk {
            start_offset: 0,
            end_offset: current_offset,
        });
    }
    
    Ok(chunks)
}

