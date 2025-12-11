use indicatif::{ProgressBar, ProgressStyle};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

pub struct ProgressTracker {
    lines_processed: Arc<AtomicU64>,
    duplicates_removed: Arc<AtomicU64>,
    start_time: Instant,
    progress_bar: Option<ProgressBar>,
}

impl ProgressTracker {
    pub fn new(show_progress: bool) -> Self {
        let progress_bar = if show_progress {
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner:.green} [{elapsed_precise}] {msg}")
                    .unwrap(),
            );
            Some(pb)
        } else {
            None
        };

        Self {
            lines_processed: Arc::new(AtomicU64::new(0)),
            duplicates_removed: Arc::new(AtomicU64::new(0)),
            start_time: Instant::now(),
            progress_bar,
        }
    }

    pub fn increment_lines(&self, count: u64) {
        let prev = self.lines_processed.fetch_add(count, Ordering::Relaxed);
        let new_total = prev + count;
        
        if new_total % 100_000 == 0 {
            if let Some(ref pb) = self.progress_bar {
                pb.set_message(format!("Processed {} lines", new_total));
            } else {
                eprintln!("Processed {} lines", new_total);
            }
        }
    }

    pub fn increment_duplicates(&self, count: u64) {
        self.duplicates_removed.fetch_add(count, Ordering::Relaxed);
    }

    pub fn finish(&self) -> Metrics {
        if let Some(ref pb) = self.progress_bar {
            pb.finish_with_message("Completed");
        }

        Metrics {
            lines_processed: self.lines_processed.load(Ordering::Relaxed),
            duplicates_removed: self.duplicates_removed.load(Ordering::Relaxed),
            processing_time: self.start_time.elapsed(),
        }
    }
}

pub struct Metrics {
    pub lines_processed: u64,
    pub duplicates_removed: u64,
    pub processing_time: std::time::Duration,
}

impl std::fmt::Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Deduplication Complete!")?;
        writeln!(f, "  Lines processed: {}", self.lines_processed)?;
        writeln!(f, "  Duplicates removed: {}", self.duplicates_removed)?;
        writeln!(f, "  Processing time: {:.2}s", self.processing_time.as_secs_f64())?;
        if self.lines_processed > 0 {
            writeln!(
                f,
                "  Throughput: {:.2} lines/sec",
                self.lines_processed as f64 / self.processing_time.as_secs_f64()
            )?;
        }
        Ok(())
    }
}

