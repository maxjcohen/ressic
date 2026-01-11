/// Represents an article in an RSS feed.
///
/// An article contains the essential information needed for RSS feed generation:
/// title, content, and a unique identifier.
#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Article {
    /// The title of the article.
    pub title: String,
    /// The main content/body of the article.
    pub content: String,
    /// Unique identifier for the article within a feed.
    /// Supports arbitrary strings for UUIDs, URLs, or custom ID schemes.
    pub id: String,
}