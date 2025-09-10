use super::models::Article;

pub trait FeedStorage {
    fn get_all_articles(&self) -> Vec<Article>;
    fn get_latest_article(&self) -> Article;
    fn store_article(&self, article: Article);
}

pub struct MockStorage {}

impl FeedStorage for MockStorage {
    fn get_all_articles(&self) -> Vec<Article> {
        vec![Article::new_empty()]
    }

    fn get_latest_article(&self) -> Article {
        Article::new_empty()
    }

    fn store_article(&self, _article: Article) {}
}
