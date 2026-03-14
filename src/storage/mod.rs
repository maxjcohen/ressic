use crate::models::Feed;

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
    FeedNotFound,
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

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::Io(e) => write!(f, "IO error: {}", e),
            StorageError::Json(e) => write!(f, "Serialization error: {}", e),
            StorageError::FeedNotFound => write!(f, "Feed not found"),
            StorageError::InvalidFeedName(name) => write!(f, "Invalid feed name: {}", name),
        }
    }
}

impl std::error::Error for StorageError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            StorageError::Io(e) => Some(e),
            StorageError::Json(e) => Some(e),
            _ => None,
        }
    }
}

/// Trait for feed storage backends.
///
/// Implementations of this trait provide persistent storage for RSS feed articles.
/// Multiple storage backends can be implemented (e.g., local files, databases).
pub trait FeedStorage {
    /// Retrieves all articles from the specified feed.
    ///
    /// Returns a `FeedNotFound` error if the feed does not exist.
    fn get_feed(&self, feed_name: &str) -> Result<Feed, StorageError>;

    /// Retrieve the name of all stored feeds.
    fn list_feeds(&self) -> Result<Vec<String>, StorageError>;

    /// Stores or updates a feed atomically.
    ///
    /// Reads the existing feed (treating `FeedNotFound` as empty), merges incoming
    /// articles by URL (incoming wins on conflict), overwrites metadata, then writes.
    fn put_feed(&self, feed: &Feed) -> Result<(), StorageError>;
}

pub use local::JsonLocalStorage;
pub use mock::MockStorage;
