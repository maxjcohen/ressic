pub mod models;
pub mod storage;

use crate::{models::Article, storage::FeedStorage};

pub struct Client<S: FeedStorage> {
    pub storage: S,
}

impl<S: FeedStorage> Client<S> {
    pub fn new(storage: S) -> Self {
        Client { storage: storage }
    }

    pub fn store_article(&self, article: Article) {
        self.storage.store_article(article);
    }

    pub fn get_articles(&self) -> Vec<Article> {
        vec![Article::new_empty()]
    }

    pub fn generate_feed(&self) -> String {
        String::from("")
    }
}
