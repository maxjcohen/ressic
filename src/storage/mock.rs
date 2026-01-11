use super::{FeedStorage, StorageError};
use crate::models::Article;

/// Mock storage implementation for testing.
///
/// This implementation doesn't persist any data and is useful
/// for unit tests that don't require actual storage.
pub struct MockStorage {}

impl MockStorage {
    /// Creates a new mock storage instance.
    pub fn new() -> Self {
        MockStorage {}
    }
}

impl FeedStorage for MockStorage {
    fn get_all_articles(&self, _feed: &str) -> Result<Vec<Article>, StorageError> {
        Ok(vec![])
    }

    fn get_latest_article(&self, _feed: &str) -> Result<Article, StorageError> {
        Err(StorageError::FeedEmpty)
    }

    fn store_article(&mut self, _feed: &str, _article: Article) -> Result<(), StorageError> {
        Ok(())
    }
}