//! Test helper: a uniquely named directory under the system temp dir, removed on drop so a
//! failing test does not leave files behind.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct TempDir(PathBuf);

impl TempDir {
    /// Creates `<temp>/kinovea-<name>-<pid>-<nanos>`. Tests run in parallel, so the name
    /// carries the process id and the creation time.
    pub fn new(name: &str) -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock after the epoch")
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("kinovea-{name}-{}-{nanos}", std::process::id()));
        fs::create_dir_all(&path).expect("create temp dir");
        Self(path)
    }

    pub fn path(&self) -> &Path {
        &self.0
    }

    pub fn join(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
