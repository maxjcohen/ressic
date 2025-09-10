pub mod models;
pub mod storage;

use crate::{models::Article, storage::{FeedStorage, StorageError}};

pub struct Client<S: FeedStorage> {
    pub storage: S,
}

impl<S: FeedStorage> Client<S> {
    pub fn new(storage: S) -> Self {
        Client { storage: storage }
    }

    pub fn store_article(&mut self, feed: &str, article: Article) -> Result<(), StorageError> {
        self.storage.store_article(feed, article)
    }

    pub fn get_articles(&self, feed: &str) -> Result<Vec<Article>, StorageError> {
        self.storage.get_all_articles(feed)
    }

    pub fn generate_feed(&self) -> String {
        String::from("")
    }
}
