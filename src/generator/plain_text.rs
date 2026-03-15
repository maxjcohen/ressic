use super::{FeedGenerator, GeneratorError};
use crate::models::Feed;

/// Plain text generator for testing purposes.
///
/// Returns a simple text representation of the feed and articles,
/// useful for verifying generator integration without dealing with
/// complex XML/Atom formatting.
pub struct PlainText;

impl PlainText {
    pub fn new() -> Self {
        PlainText
    }
}

impl Default for PlainText {
    fn default() -> Self {
        Self::new()
    }
}

impl FeedGenerator for PlainText {
    fn generate(&self, feed: &Feed) -> Result<String, GeneratorError> {
        let mut output = String::new();
        output.push_str(&format!("FEED: {}\n", feed.title));
        output.push_str(&format!("DESCRIPTION: {}\n", feed.description));
        output.push_str(&format!("LINK: {}\n", feed.link));
        output.push_str(&format!("ARTICLES: {}\n", feed.articles.len()));

        for (i, article) in feed.articles.iter().enumerate() {
            output.push_str(&format!("\n[Article {}]\n", i + 1));
            output.push_str(&format!("Title: {}\n", article.title));
            output.push_str(&format!("URL: {}\n", article.url));
            output.push_str(&format!("Summary: {}\n", article.summary));
            output.push_str(&format!("Date: {}\n", article.pub_date));
        }

        Ok(output)
    }

    fn mime_type(&self) -> &'static str {
        "text/plain"
    }
}
