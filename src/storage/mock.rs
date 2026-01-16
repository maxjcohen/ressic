use super::{FeedStorage, StorageError};
use crate::models::{Article, Feed};

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
    fn get_feed(&self, feed_name: &str) -> Result<Feed, StorageError> {
        Ok(Feed {
            name: feed_name.into(),
            title: "".into(),
            link: "".into(),
            description: "".into(),
            articles: vec![],
        })
    }

    fn store_article(&self, _feed_name: &str, _article: Article) -> Result<(), StorageError> {
        Ok(())
    }

    fn set_feed_metadata(&self, _feed_name: &str, _feed: &Feed) -> Result<(), StorageError> {
        Ok(())
    }
}
