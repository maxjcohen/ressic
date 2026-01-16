/// Represents an article in an RSS feed.
///
/// An article contains the essential information needed for RSS feed generation:
/// title, content, unique identifier, URL, and publication date.
#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Article {
    /// The title of the article.
    pub title: String,
    /// The main content/body of the article.
    pub content: String,
    /// Unique identifier for the article within a feed.
    /// Supports arbitrary strings for UUIDs, URLs, or custom ID schemes.
    pub id: String,
    /// The URL linking to the original article source.
    pub url: String,
    /// The publication date and time of the article (UTC).
    pub pub_date: chrono::DateTime<chrono::Utc>,
}