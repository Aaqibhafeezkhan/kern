use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

/// Creates a new and unique path to a test file
/// The file itself will not be created
pub fn temp_file_path() -> PathBuf {
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);

    std::env::temp_dir().join(format!("kern-test-{}-{id}.txt", std::process::id()))
}
