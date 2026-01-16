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
    /// A brief summary or description of the article.
    pub summary: String,
    /// The publication date and time of the article (UTC).
    pub pub_date: chrono::DateTime<chrono::Utc>,
}

/// Represents metadata for an RSS feed.
///
/// Contains channel-level information required for RSS 2.0 feed generation,
/// including both required fields (title, link, description) and optional fields
/// for enhanced feed metadata.
#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Feed {
    /// The name identifier for the feed (used internally).
    pub name: String,
    /// The title of the feed (RSS channel title).
    pub title: String,
    /// The URL to the website associated with the feed.
    pub link: String,
    /// A description or tagline for the feed.
    pub description: String,
    /// A list of articles contained in the feed.
    pub articles: Vec<Article>,
}