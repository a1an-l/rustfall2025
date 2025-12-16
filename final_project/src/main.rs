mod thread_pool;
mod file_analyzer;
mod progress;
mod cancellation;
mod errors;

use std::sync::{Arc, Mutex};
use std::time::Instant;

use thread_pool::ThreadPool;
use progress::Progress;
use cancellation::CancellationToken;
use file_analyzer::analyze_file;

fn main() {
    let directories = vec![
        "gutenberg_books".to_string(), // put your 100 books here
    ];

    let files = file_analyzer::collect_files(directories);
    let total_files = files.len();

    let progress = Arc::new(Mutex::new(Progress::new(total_files)));
    let cancel_token = CancellationToken::new();

    let pool = ThreadPool::new(4); // change thread count here

    let start = Instant::now();

    for file in files {
        let progress = Arc::clone(&progress);
        let cancel = cancel_token.clone();

        pool.execute(move || {
            if cancel.is_cancelled() {
                return;
            }

            let result = analyze_file(file);

            let mut p = progress.lock().unwrap();
            println!("{}", result);
            p.update(&result);

        });
    }

    pool.shutdown();

    let elapsed = start.elapsed();
    println!("Finished processing in {:?}", elapsed);
}
