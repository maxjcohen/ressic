//! Core data models for Ressic RSS feed management.
//!
//! This module provides the fundamental data structures for representing RSS feeds
//! and articles, with validation built into constructors to ensure data integrity.
//!
//! # Validation Approach
//!
//! All models in this module use constructor-based validation. Struct literals
//! should not be used directly in production code - instead, use the `new()`
//! constructors which enforce validation rules:
//!
//! - **Article::new()**: Validates that title, content, id, and url are non-empty
//! - **Feed::new()**: Validates feed name format and that title, link, and description
//!   are non-empty
//!
//! # Example
//!
//! ```
//! use ressic::models::{Article, Feed};
//! use chrono::Utc;
//!
//! // Create a validated article
//! let article = Article::new(
//!     "Article Title".to_string(),
//!     "Article content".to_string(),
//!     "unique-id".to_string(),
//!     "https://example.com/article".to_string(),
//!     "Brief summary".to_string(),
//!     Utc::now(),
//! ).expect("valid article");
//!
//! // Create a validated feed
//! let feed = Feed::new(
//!     "my-feed".to_string(),
//!     "My Feed".to_string(),
//!     "https://example.com".to_string(),
//!     "Feed description".to_string(),
//!     vec![article],
//! ).expect("valid feed");
//! ```
//!
//! # Internal Storage
//!
//! The storage layer uses serde deserialization which bypasses validation. This is
//! acceptable because:
//! - Storage is an internal layer that only reads data it previously wrote
//! - All data entering storage has already been validated at the API boundary
//! - This keeps the storage layer simple and performant
//!
//! # Error Handling
//!
//! Validation errors are returned as `Result<T, ValidationError>` from constructors.
//! The API layer automatically converts `ValidationError` to HTTP 400 Bad Request responses.

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
#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
///
/// # Validation
///
/// Feeds cannot be constructed directly. Use `Feed::new()` which validates:
/// - `name` must not be empty and contain only alphanumeric characters, hyphens, and underscores
/// - `name` must not contain path separators (`/`, `\`) or `..`
/// - `title` must not be empty (after trimming whitespace)
/// - `link` must not be empty (after trimming whitespace)
/// - `description` must not be empty (after trimming whitespace)
/// - `articles` can be any Vec<Article> (including empty)
#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct Feed {
    /// The name identifier for the feed (used internally).
    /// When deserializing from a POST request body, this field is ignored in favour
    /// of the feed name present in the URL path.
    #[serde(default)]
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

impl Feed {
    /// Creates a new Feed with validation.
    ///
    /// # Arguments
    ///
    /// * `name` - Internal feed identifier (alphanumeric, hyphens, underscores only)
    /// * `title` - Feed title (must not be empty after trimming)
    /// * `link` - Feed website URL (must not be empty after trimming)
    /// * `description` - Feed description (must not be empty after trimming)
    /// * `articles` - List of articles (can be empty)
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::InvalidFeedName` if the name contains invalid characters
    /// or patterns. Returns `ValidationError::EmptyField` if any required field is empty
    /// or contains only whitespace.
    ///
    /// # Example
    ///
    /// ```
    /// use ressic::models::Feed;
    ///
    /// let feed = Feed::new(
    ///     "my-feed".to_string(),
    ///     "My Feed Title".to_string(),
    ///     "https://example.com".to_string(),
    ///     "A description of my feed".to_string(),
    ///     vec![],
    /// ).expect("Valid feed");
    /// ```
    pub fn new(
        name: String,
        title: String,
        link: String,
        description: String,
        articles: Vec<Article>,
    ) -> Result<Self, ValidationError> {
        // Validate and trim name
        let name = name.trim().to_string();

        // Check if name is empty
        if name.is_empty() {
            return Err(ValidationError::InvalidFeedName {
                name: name.clone(),
                reason: "Feed name cannot be empty".to_string(),
            });
        }

        // Check for path traversal patterns
        if name.contains("..") || name.contains('/') || name.contains('\\') {
            return Err(ValidationError::InvalidFeedName {
                name: name.clone(),
                reason: "Feed name contains invalid path characters".to_string(),
            });
        }

        // Check for valid characters (alphanumeric, hyphens, underscores only)
        if !name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return Err(ValidationError::InvalidFeedName {
                name: name.clone(),
                reason:
                    "Feed name must contain only alphanumeric characters, hyphens, and underscores"
                        .to_string(),
            });
        }

        // Validate title
        let title = title.trim().to_string();
        if title.is_empty() {
            return Err(ValidationError::EmptyField {
                field: "title".to_string(),
            });
        }

        // Validate link
        let link = link.trim().to_string();
        if link.is_empty() {
            return Err(ValidationError::EmptyField {
                field: "link".to_string(),
            });
        }

        // Validate description
        let description = description.trim().to_string();
        if description.is_empty() {
            return Err(ValidationError::EmptyField {
                field: "description".to_string(),
            });
        }

        Ok(Feed {
            name,
            title,
            link,
            description,
            articles,
        })
    }
}
