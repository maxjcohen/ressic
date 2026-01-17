/// Validation errors that can occur when constructing models.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// A required field is empty or contains only whitespace
    EmptyField { field: String },
    /// Feed name contains invalid characters or patterns
    InvalidFeedName { name: String, reason: String },
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::EmptyField { field } => {
                write!(f, "Field '{}' cannot be empty", field)
            }
            ValidationError::InvalidFeedName { name, reason } => {
                write!(f, "Invalid feed name '{}': {}", name, reason)
            }
        }
    }
}

impl std::error::Error for ValidationError {}

/// Represents an article in an RSS feed.
///
/// An article contains the essential information needed for RSS feed generation:
/// title, content, unique identifier, URL, and publication date.
///
/// # Validation
///
/// Articles cannot be constructed directly. Use `Article::new()` which validates:
/// - `title` must not be empty (after trimming whitespace)
/// - `content` must not be empty (after trimming whitespace)
/// - `id` must not be empty (after trimming whitespace)
/// - `url` must not be empty (after trimming whitespace)
/// - `summary` can be empty (optional description)
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

impl Article {
    /// Creates a new Article with validation.
    ///
    /// # Arguments
    ///
    /// * `title` - The article title (must not be empty after trimming)
    /// * `content` - The article content (must not be empty after trimming)
    /// * `id` - Unique identifier (must not be empty after trimming)
    /// * `url` - The article URL (must not be empty after trimming)
    /// * `summary` - Brief summary (can be empty)
    /// * `pub_date` - Publication date and time
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::EmptyField` if any required field is empty or contains only whitespace.
    ///
    /// # Example
    ///
    /// ```
    /// use ressic::models::Article;
    /// use chrono::Utc;
    ///
    /// let article = Article::new(
    ///     "Article Title".to_string(),
    ///     "Article content goes here".to_string(),
    ///     "unique-id-123".to_string(),
    ///     "https://example.com/article".to_string(),
    ///     "Brief summary".to_string(),
    ///     Utc::now(),
    /// ).expect("Valid article");
    /// ```
    pub fn new(
        title: String,
        content: String,
        id: String,
        url: String,
        summary: String,
        pub_date: chrono::DateTime<chrono::Utc>,
    ) -> Result<Self, ValidationError> {
        // Validate title
        let title = title.trim().to_string();
        if title.is_empty() {
            return Err(ValidationError::EmptyField {
                field: "title".to_string(),
            });
        }

        // Validate content
        let content = content.trim().to_string();
        if content.is_empty() {
            return Err(ValidationError::EmptyField {
                field: "content".to_string(),
            });
        }

        // Validate id
        let id = id.trim().to_string();
        if id.is_empty() {
            return Err(ValidationError::EmptyField {
                field: "id".to_string(),
            });
        }

        // Validate url
        let url = url.trim().to_string();
        if url.is_empty() {
            return Err(ValidationError::EmptyField {
                field: "url".to_string(),
            });
        }

        // Summary can be empty, but trim it
        let summary = summary.trim().to_string();

        Ok(Article {
            title,
            content,
            id,
            url,
            summary,
            pub_date,
        })
    }
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
