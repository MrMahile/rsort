use anyhow::{Context, Result};
use hashbrown::HashSet;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::chunk_processor::{find_chunk_boundaries, process_chunk_stream};
use crate::progress::ProgressTracker;

pub struct Deduplicator {
    input_path: std::path::PathBuf,
    output_path: std::path::PathBuf,
    chunk_size_bytes: usize,
}

impl Deduplicator {
    pub fn new(
        input_path: &Path,
        output_path: &Path,
        chunk_size_bytes: usize,
    ) -> Result<Self> {
        Ok(Self {
            input_path: input_path.to_path_buf(),
            output_path: output_path.to_path_buf(),
            chunk_size_bytes,
        })
    }

    pub fn process(&mut self) -> Result<()> {
        let progress = ProgressTracker::new(true);
        
        eprintln!("Splitting file into chunks...");
        let chunks = find_chunk_boundaries(&self.input_path, self.chunk_size_bytes)
            .context("Failed to split file into chunks")?;
        
        eprintln!("Found {} chunks, processing sequentially with streaming...", chunks.len());
        
        // Validate chunks before processing
        let file_size = crate::chunk_processor::get_file_size(&self.input_path)
            .context("Failed to get input file size for validation")?;
        
        for (idx, chunk) in chunks.iter().enumerate() {
            if chunk.start_offset > file_size {
                return Err(anyhow::anyhow!(
                    "Invalid chunk {}: start offset {} exceeds file size {} (file: {})",
                    idx,
                    chunk.start_offset,
                    file_size,
                    self.input_path.display()
                ));
            }
            if chunk.end_offset > file_size {
                eprintln!(
                    "Warning: Chunk {} end offset {} exceeds file size {}, will be adjusted during processing",
                    idx,
                    chunk.end_offset,
                    file_size
                );
            }
        }
        
        // Open output file with larger buffer for better I/O performance (1MB buffer)
        let output_file = File::create(&self.output_path)
            .with_context(|| format!("Failed to create output file: {}", self.output_path.display()))?;
        let mut writer = BufWriter::with_capacity(1024 * 1024, output_file); // 1MB buffer for better I/O
        let mut global_seen = HashSet::<u64>::new();
        
        // Process chunks sequentially, streaming directly to output
        let total_chunks = chunks.len();
        for (idx, chunk) in chunks.iter().enumerate() {
            if (idx + 1) % 10 == 0 || idx == 0 {
                eprintln!("Processing chunk {}/{} (offset {}-{})...", 
                         idx + 1, total_chunks, chunk.start_offset, chunk.end_offset);
            }
            
            process_chunk_stream(
                &self.input_path,
                chunk,
                &mut writer,
                &mut global_seen,
                &progress,
            )
            .with_context(|| format!(
                "Failed to process chunk {} (offset {}-{}, file: {})",
                idx,
                chunk.start_offset,
                chunk.end_offset,
                self.input_path.display()
            ))?;
            
            // Flush periodically to ensure progress is written
            if (idx + 1) % 50 == 0 {
                writer.flush().context("Failed to flush output buffer")?;
            }
        }
        
        // Final flush
        writer.flush().context("Failed to flush final output")?;
        
        let metrics = progress.finish();
        eprintln!("{}", metrics);
        
        Ok(())
    }
}

