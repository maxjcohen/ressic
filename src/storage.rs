use super::models::Article;

pub trait FeedStorage {
    fn get_all_articles(&self, feed: &str) -> Vec<Article>;
    fn get_latest_article(&self, feed: &str) -> Article;
    fn store_article(&mut self, feed: &str, article: Article);
}

pub struct MockStorage {}

impl MockStorage {
    pub fn new() -> Self {
        MockStorage {}
    }
}

impl FeedStorage for MockStorage {
    fn get_all_articles(&self, _feed: &str) -> Vec<Article> {
        vec![Article::new_empty()]
    }

    fn get_latest_article(&self, _feed: &str) -> Article {
        Article::new_empty()
    }

    fn store_article(&mut self, _feed: &str, _article: Article) {}
}
