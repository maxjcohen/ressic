use crate::models::Article;

pub mod local;
pub mod mock;

#[derive(Debug)]
pub enum StorageError {
    Io(std::io::Error),
    Json(serde_json::Error),
    FeedEmpty,
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

pub trait FeedStorage {
    fn get_all_articles(&self, feed: &str) -> Result<Vec<Article>, StorageError>;
    fn get_latest_article(&self, feed: &str) -> Result<Article, StorageError>;
    fn store_article(&mut self, feed: &str, article: Article) -> Result<(), StorageError>;
}

pub use local::LocalFile;
pub use mock::MockStorage;