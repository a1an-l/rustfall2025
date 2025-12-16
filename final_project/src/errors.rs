use std::fmt;
use std::io;

pub struct ProcessingError {
    pub message: String,
    pub path: String,
}

impl ProcessingError {
    pub fn new(err: io::Error, path: &str) -> Self {
        Self {
            message: err.to_string(),
            path: path.to_string(),
        }
    }
}

impl fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.path, self.message)
    }
}
