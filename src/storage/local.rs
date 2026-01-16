use super::{FeedStorage, StorageError};
use crate::models::Article;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

/// A JSON-based local file storage implementation for RSS feed articles.
///
/// This storage backend persists articles to the local filesystem using JSON Lines (JSONL) format,
/// where each line in a file represents a single serialized article. Each feed is stored in a
/// separate file named `{feed_name}.jsonl` within the configured base directory.
///
/// # File Format
///
/// Articles are stored in JSON Lines format (one JSON object per line), which allows for:
/// - Efficient appending of new articles
/// - Easy line-by-line parsing without loading entire files into memory
/// - Simple recovery from partial writes
///
/// # Security
///
/// Feed names are validated to prevent path traversal attacks. Invalid feed names will be rejected
/// during operations. See [`validate_feed_name`](Self::validate_feed_name) for validation rules.
///
/// # Thread Safety
///
/// This implementation is not thread-safe. Concurrent access to the same feed from multiple
/// threads or processes may result in data corruption or race conditions.
///
/// # Example
///
/// ```no_run
/// use ressic::storage::local::JsonLocalStorage;
///
/// let storage = JsonLocalStorage::new("/path/to/feeds").unwrap();
/// ```
pub struct JsonLocalStorage {
    base_dir: PathBuf,
}

impl JsonLocalStorage {
    /// Creates a new local storage instance.
    ///
    /// The base directory will be created if it doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `base_dir` - The directory where feed files will be stored
    ///
    /// # Errors
    ///
    /// Returns `StorageError::Io` if the directory cannot be created.
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Result<Self, StorageError> {
        let p = base_dir.as_ref().to_path_buf();
        if !p.exists() {
            std::fs::create_dir_all(&p)?;
        }
        Ok(JsonLocalStorage { base_dir: p })
    }

    /// Validates that a feed name is safe for use in file paths.
    ///
    /// Prevents path traversal attacks by rejecting feed names containing:
    /// - Path separators (/, \)
    /// - Parent directory references (..)
    /// - Hidden files (.)
    /// - Control characters
    /// - Empty strings
    fn validate_feed_name(feed: &str) -> Result<(), StorageError> {
        if feed.is_empty() {
            return Err(StorageError::InvalidFeedName(
                "Feed name cannot be empty".to_string(),
            ));
        }

        // Reject if contains path separators or parent references
        if feed.contains('/') || feed.contains('\\') || feed.contains("..") {
            return Err(StorageError::InvalidFeedName(format!(
                "Feed name contains invalid path characters: {}",
                feed
            )));
        }

        // Reject if starts with dot (hidden files)
        if feed.starts_with('.') {
            return Err(StorageError::InvalidFeedName(format!(
                "Feed name cannot start with '.': {}",
                feed
            )));
        }

        // Reject control characters and non-printable ASCII
        if feed.chars().any(|c| c.is_control()) {
            return Err(StorageError::InvalidFeedName(
                "Feed name contains control characters".to_string(),
            ));
        }

        Ok(())
    }

    fn feed_path(&self, feed: &str) -> Result<PathBuf, StorageError> {
        Self::validate_feed_name(feed)?;
        let mut p = self.base_dir.clone();
        p.push(format!("{}.jsonl", feed));
        Ok(p)
    }

    fn read_all(&self, feed: &str) -> Result<Vec<Article>, StorageError> {
        let path = self.feed_path(feed)?;
        if !path.exists() {
            return Ok(vec![]);
        }

        let f = File::open(&path)?;
        let reader = BufReader::new(f);
        let mut out = Vec::new();
        for line in reader.lines() {
            let l = line?;
            if l.trim().is_empty() {
                continue;
            }
            let a: Article = serde_json::from_str(&l)?;
            out.push(a);
        }
        Ok(out)
    }

    fn write_all(&self, feed: &str, articles: &[Article]) -> Result<(), StorageError> {
        let path = self.feed_path(feed)?;
        let tmp_path = path.with_extension("jsonl.tmp");

        if let Some(parent) = tmp_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }

        let tmpf = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&tmp_path)?;
        let mut writer = BufWriter::new(tmpf);
        for a in articles {
            let line = serde_json::to_string(a)?;
            writer.write_all(line.as_bytes())?;
            writer.write_all(b"\n")?;
        }
        writer.flush()?;

        // Atomically replace
        std::fs::rename(tmp_path, path)?;
        Ok(())
    }
}

impl FeedStorage for JsonLocalStorage {
    fn get_all_articles(&self, feed: &str) -> Result<Vec<Article>, StorageError> {
        let articles = self.read_all(feed)?;
        // Return in insertion order
        Ok(articles)
    }

    fn get_latest_article(&self, feed: &str) -> Result<Article, StorageError> {
        let articles = self.read_all(feed)?;
        articles
            .into_iter()
            .max_by_key(|a| a.pub_date)
            .ok_or(StorageError::FeedEmpty)
    }

    fn store_article(&self, feed: &str, article: Article) -> Result<(), StorageError> {
        // Read existing, dedupe by url, update or append, then rewrite file
        let mut articles = self.read_all(feed)?;
        if let Some(a) = articles.iter_mut().find(|a| a.url == article.url) {
            *a = article;
        } else {
            articles.push(article);
        }
        self.write_all(feed, &articles)
    }
}