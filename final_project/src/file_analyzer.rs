use std::fmt;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};

use crate::errors::ProcessingError;

pub struct FileAnalysis {
    pub filename: String,
    pub stats: FileStats,
    pub errors: Vec<ProcessingError>,
    pub processing_time: Duration,
}

pub struct FileStats {
    pub word_count: usize,
    pub line_count: usize,
    pub char_frequencies: HashMap<char, usize>,
    pub size_bytes: u64,
}

pub fn analyze_file(path: String) -> FileAnalysis {
    let start = Instant::now();
    let mut errors = Vec::new();

    let mut stats = FileStats {
        word_count: 0,
        line_count: 0,
        char_frequencies: HashMap::new(),
        size_bytes: 0,
    };

    match File::open(&path) {
        Ok(file) => {
            if let Ok(meta) = file.metadata() {
                stats.size_bytes = meta.len();
            }

            let reader = BufReader::new(file);
            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        stats.line_count += 1;
                        stats.word_count += l.split_whitespace().count();
                        for c in l.chars() {
                            *stats.char_frequencies.entry(c).or_insert(0) += 1;
                        }
                    }
                    Err(e) => errors.push(ProcessingError::new(e, &path)),
                }
            }
        }
        Err(e) => errors.push(ProcessingError::new(e, &path)),
    }

    FileAnalysis {
        filename: path,
        stats,
        errors,
        processing_time: start.elapsed(),
    }
}

pub fn collect_files(dirs: Vec<String>) -> Vec<String> {
    let mut files = Vec::new();

    for dir in dirs {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_file() {
                    if let Some(s) = p.to_str() {
                        files.push(s.to_string());
                    }
                }
            }
        }
    }

    files
}

impl fmt::Display for FileStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FileStats {{ word_count: {}, line_count: {}, size_bytes: {} }}",
            self.word_count,
            self.line_count,
            self.size_bytes
        )
    }
}

impl fmt::Display for FileAnalysis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FileAnalysis {{\n  filename: {},\n  stats: {},\n  errors: {},\n  processing_time: {:?}\n}}",
            self.filename,
            self.stats,
            self.errors.len(),
            self.processing_time
        )
    }
}
