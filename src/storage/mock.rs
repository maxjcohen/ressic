use super::{FeedStorage, StorageError};
use crate::models::Feed;

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

    fn list_feeds(&self) -> Result<Vec<String>, StorageError> {
        Ok(vec![])
    }

    fn put_feed(&self, _feed: &Feed) -> Result<(), StorageError> {
        Ok(())
    }
}
