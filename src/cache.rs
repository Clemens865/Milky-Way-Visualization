use anyhow::Result;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

const CACHE_DIR: &str = "data/cache";

/// Simple file-based HTTP response cache.
/// Avoids hammering APIs during exploration.
pub struct Cache {
    dir: PathBuf,
    ttl: Duration,
}

impl Cache {
    pub fn new(ttl_hours: u64) -> Self {
        let dir = PathBuf::from(CACHE_DIR);
        std::fs::create_dir_all(&dir).ok();
        Self {
            dir,
            ttl: Duration::from_secs(ttl_hours * 3600),
        }
    }

    fn key_path(&self, url: &str) -> PathBuf {
        let mut hasher = Sha256::new();
        hasher.update(url.as_bytes());
        let hash = hex::encode(hasher.finalize());
        self.dir.join(format!("{hash}.json"))
    }

    pub fn get(&self, url: &str) -> Option<String> {
        let path = self.key_path(url);
        if !path.exists() {
            return None;
        }

        // Check TTL
        if let Ok(meta) = std::fs::metadata(&path) {
            if let Ok(modified) = meta.modified() {
                if SystemTime::now()
                    .duration_since(modified)
                    .unwrap_or_default()
                    > self.ttl
                {
                    return None;
                }
            }
        }

        std::fs::read_to_string(&path).ok()
    }

    pub fn set(&self, url: &str, data: &str) -> Result<()> {
        let path = self.key_path(url);
        std::fs::write(&path, data)?;
        Ok(())
    }

    pub fn stats(&self) -> Result<CacheStats> {
        let mut count = 0u64;
        let mut total_bytes = 0u64;

        if let Ok(entries) = std::fs::read_dir(&self.dir) {
            for entry in entries.flatten() {
                if entry.path().extension().is_some_and(|e| e == "json") {
                    count += 1;
                    total_bytes += entry.metadata().map(|m| m.len()).unwrap_or(0);
                }
            }
        }

        Ok(CacheStats { count, total_bytes })
    }

    pub fn clear(&self) -> Result<u64> {
        let mut removed = 0u64;
        if let Ok(entries) = std::fs::read_dir(&self.dir) {
            for entry in entries.flatten() {
                if entry.path().extension().is_some_and(|e| e == "json") {
                    std::fs::remove_file(entry.path())?;
                    removed += 1;
                }
            }
        }
        Ok(removed)
    }
}

pub struct CacheStats {
    pub count: u64,
    pub total_bytes: u64,
}

impl std::fmt::Display for CacheStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mb = self.total_bytes as f64 / (1024.0 * 1024.0);
        write!(f, "{} cached responses ({:.2} MB)", self.count, mb)
    }
}
