use super::{FeedStorage, StorageError};
use crate::models::{Article, Feed};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

/// A JSON-based local file storage implementation for RSS feed articles.
///
/// This storage backend persists articles to the local filesystem using Json format.
/// Each feed is stored in a separate file named `{feed_name}.json` within
/// the configured base directory.
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
    fn validate_feed_name(feed_name: &str) -> Result<(), StorageError> {
        if feed_name.is_empty() {
            return Err(StorageError::InvalidFeedName(
                "Feed name cannot be empty".to_string(),
            ));
        }

        // Reject if contains path separators or parent references
        if feed_name.contains('/') || feed_name.contains('\\') || feed_name.contains("..") {
            return Err(StorageError::InvalidFeedName(format!(
                "Feed name contains invalid path characters: {}",
                feed_name
            )));
        }

        // Reject if starts with dot (hidden files)
        if feed_name.starts_with('.') {
            return Err(StorageError::InvalidFeedName(format!(
                "Feed name cannot start with '.': {}",
                feed_name
            )));
        }

        // Reject control characters and non-printable ASCII
        if feed_name.chars().any(|c| c.is_control()) {
            return Err(StorageError::InvalidFeedName(
                "Feed name contains control characters".to_string(),
            ));
        }

        Ok(())
    }

    fn feed_path(&self, feed_name: &str) -> Result<PathBuf, StorageError> {
        Self::validate_feed_name(feed_name)?;
        let mut p = self.base_dir.clone();
        p.push(format!("{}.json", feed_name));
        Ok(p)
    }

    fn list_stored_feeds(&self) -> Result<Vec<String>, StorageError> {
        let mut feeds = Vec::new();
        for entry in std::fs::read_dir(&self.base_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "json" {
                        if let Some(stem) = path.file_stem() {
                            if let Some(feed_name) = stem.to_str() {
                                feeds.push(feed_name.to_string());
                            }
                        }
                    }
                }
            }
        }
        Ok(feeds)
    }

    fn read_feed(&self, feed_name: &str) -> Result<Feed, StorageError> {
        // Validate feed name and construct path
        let feed_path = self.feed_path(feed_name)?;
        // Attempt to read feed, return FeedEmpty if not found
        let f = match File::open(&feed_path) {
            Ok(file) => file,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Err(StorageError::FeedEmpty);
            }
            Err(e) => return Err(StorageError::Io(e)),
        };
        let reader = BufReader::new(f);
        Ok(serde_json::from_reader(reader)?)
    }

    fn write_feed(&self, feed_name: &str, feed: &Feed) -> Result<(), StorageError> {
        let feed_path = self.feed_path(feed_name)?;
        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&feed_path)?;
        let writer = BufWriter::new(f);
        serde_json::to_writer_pretty(writer, feed)?;
        Ok(())
    }
}

impl FeedStorage for JsonLocalStorage {
    fn get_feed(&self, feed_name: &str) -> Result<Feed, StorageError> {
        // Attempt to read existing feed; if not found, return FeedEmpty error
        Ok(self.read_feed(feed_name)?)
    }

    fn list_feeds(&self) -> Result<Vec<String>, StorageError> {
        self.list_stored_feeds()
    }

    fn store_article(&self, feed_name: &str, article: Article) -> Result<(), StorageError> {
        // Read existing, dedupe by url, update or append, then rewrite file
        // Attempt to read existing feed; if not found, start new feed
        let mut feed = self.read_feed(feed_name).unwrap_or(Feed {
            name: feed_name.to_string(),
            title: feed_name.to_string(),
            link: "".to_string(),
            description: format!("Default description for {}", feed_name),
            articles: vec![],
        });
        if let Some(a) = feed.articles.iter_mut().find(|a| a.url == article.url) {
            *a = article;
        } else {
            feed.articles.push(article);
        }
        self.write_feed(feed_name, &feed)
    }

    fn set_feed_metadata(&self, feed_name: &str, feed: &Feed) -> Result<(), StorageError> {
        // Attempt to read existing feed; if not found, start new feed
        let mut existing_feed = self.read_feed(feed_name).unwrap_or(Feed {
            name: feed_name.to_string(),
            title: "".to_string(),
            link: "".to_string(),
            description: "".to_string(),
            articles: vec![],
        });
        existing_feed.title = feed.title.clone();
        existing_feed.link = feed.link.clone();
        existing_feed.description = feed.description.clone();
        self.write_feed(feed_name, &existing_feed)
    }
}
