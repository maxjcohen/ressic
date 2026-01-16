use crate::models::{Article, Feed};

pub mod local;
pub mod mock;

/// Errors that can occur during feed storage operations.
#[derive(Debug)]
pub enum StorageError {
    /// An I/O error occurred while reading or writing feed data.
    Io(std::io::Error),
    /// A JSON serialization/deserialization error occurred.
    Json(serde_json::Error),
    /// The requested feed contains no articles.
    FeedEmpty,
    /// The feed name contains invalid characters or is unsafe.
    InvalidFeedName(String),
}

impl From<std::io::Error> for StorageError {
    fn from(e: std::io::Error) -> Self {
        StorageError::Io(e)
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(e: serde_json::Error) -> Self {
        StorageError::Json(e)
    }
}

/// Trait for feed storage backends.
///
/// Implementations of this trait provide persistent storage for RSS feed articles.
/// Multiple storage backends can be implemented (e.g., local files, databases).
pub trait FeedStorage {
    /// Retrieves all articles from the specified feed.
    ///
    /// Returns a `FeedEmpty` error if the feed contains no articles.
    fn get_feed(&self, feed_name: &str) -> Result<Feed, StorageError>;

    /// Stores an article in the specified feed.
    ///
    /// If an article with the same URL already exists, it will be replaced.
    fn store_article(&self, feed_name: &str, article: Article) -> Result<(), StorageError>;

    /// Set feed metadata.
    fn set_feed_metadata(&self, feed_name: &str, feed: &Feed) -> Result<(), StorageError>;
}

pub use local::JsonLocalStorage;
pub use mock::MockStorage;
