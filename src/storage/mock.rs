use super::{FeedStorage, StorageError};
use crate::models::Article;

pub struct MockStorage {}

impl MockStorage {
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