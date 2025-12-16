use crate::file_analyzer::FileAnalysis;

pub struct Progress {
    pub processed: usize,
    pub total: usize,
    pub errors: usize,
}

impl Progress {
    pub fn new(total: usize) -> Self {
        Self {
            processed: 0,
            total,
            errors: 0,
        }
    }

    pub fn update(&mut self, result: &FileAnalysis) {
        self.processed += 1;
        self.errors += result.errors.len();

        println!(
            "Processed {}/{} | {} errors | {} | words: {}",
            self.processed,
            self.total,
            self.errors,
            result.filename,
            result.stats.word_count
        );
    }
}
