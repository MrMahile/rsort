use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

mod deduplicator;
mod chunk_processor;
mod progress;

use deduplicator::Deduplicator;

#[derive(Parser, Debug)]
#[command(name = "rsort")]
#[command(about = "High-performance deduplication tool for large files")]
struct Args {
    /// Input file path
    input: PathBuf,
    
    /// Output file path
    output: PathBuf,
    
    /// Chunk size in MB (default: 50)
    #[arg(long, default_value = "50")]
    chunk_size: usize,
    
    /// Number of parallel threads (default: CPU count)
    #[arg(long)]
    threads: Option<usize>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Validate input file exists and is readable
    if !args.input.exists() {
        eprintln!("Error: Input file does not exist: {}", args.input.display());
        std::process::exit(1);
    }
    
    if !args.input.is_file() {
        eprintln!("Error: Input path is not a file: {}", args.input.display());
        std::process::exit(1);
    }
    
    // Check if we can read the input file
    match std::fs::File::open(&args.input) {
        Ok(_) => {}
        Err(e) => {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                eprintln!("Error: Permission denied reading input file: {}", args.input.display());
                eprintln!("Please check file permissions.");
            } else {
                eprintln!("Error: Failed to open input file: {}", args.input.display());
                eprintln!("Details: {}", e);
            }
            std::process::exit(1);
        }
    }
    
    // Check if output directory exists, create if not
    if let Some(parent) = args.output.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                eprintln!("Error: Permission denied creating output directory: {}", parent.display());
            } else {
                eprintln!("Error: Failed to create output directory: {}", parent.display());
                eprintln!("Details: {}", e);
            }
            std::process::exit(1);
        }
    }
    
    // Check if we can write to output location
    match std::fs::File::create(&args.output) {
        Ok(_) => {}
        Err(e) => {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                eprintln!("Error: Permission denied writing to output file: {}", args.output.display());
                eprintln!("Please check file permissions.");
            } else {
                eprintln!("Error: Failed to create output file: {}", args.output.display());
                eprintln!("Details: {}", e);
            }
            std::process::exit(1);
        }
    }
    
    let num_threads = args.threads.unwrap_or_else(|| num_cpus::get());
    
    if num_threads == 0 {
        eprintln!("Error: Number of threads must be greater than 0");
        std::process::exit(1);
    }
    
    if args.chunk_size == 0 {
        eprintln!("Error: Chunk size must be greater than 0");
        std::process::exit(1);
    }
    
    let mut deduplicator = Deduplicator::new(
        &args.input,
        &args.output,
        args.chunk_size * 1024 * 1024, // Convert MB to bytes
    )?;
    
    if let Err(e) = deduplicator.process() {
        eprintln!("Error during processing:");
        eprintln!("{:#}", e);
        std::process::exit(1);
    }
    
    Ok(())
}

