use crate::{Client, generator::FeedGenerator, models::Feed, storage::FeedStorage};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::sync::{Arc, Mutex};

/// Shared state type for the API server
pub type SharedClient<S, G> = Arc<Mutex<Client<S, G>>>;

/// Validates a feed name to prevent path traversal attacks and ensure safe filenames
///
/// Valid feed names must:
/// - Be non-empty
/// - Only contain alphanumeric characters, hyphens, and underscores
/// - Not contain path separators or special characters
fn validate_feed_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Feed name cannot be empty".to_string());
    }

    if name.contains("..") || name.contains('/') || name.contains('\\') {
        return Err("Feed name contains invalid path characters".to_string());
    }

    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Err(
            "Feed name must contain only alphanumeric characters, hyphens, and underscores"
                .to_string(),
        );
    }

    Ok(())
}

/// POST /v1/feeds/:feed_name
///
/// Adds articles to a feed. The request body should contain a Feed object with
/// metadata and a list of articles. Feed metadata will be replaced if the feed
/// already exists.
pub async fn post_feed<S: FeedStorage, G: FeedGenerator>(
    Path(feed_name): Path<String>,
    State(client): State<SharedClient<S, G>>,
    Json(feed): Json<Feed>,
) -> Result<StatusCode, ApiError> {
    // Validate feed name
    validate_feed_name(&feed_name)?;

    // Validate feed metadata
    if feed.title.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "Feed title cannot be empty".to_string(),
        ));
    }

    if feed.link.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "Feed link cannot be empty".to_string(),
        ));
    }

    if feed.description.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "Feed description cannot be empty".to_string(),
        ));
    }

    // Validate articles
    for article in &feed.articles {
        if article.title.trim().is_empty() {
            return Err(ApiError::BadRequest(
                "Article title cannot be empty".to_string(),
            ));
        }
        if article.content.trim().is_empty() {
            return Err(ApiError::BadRequest(
                "Article content cannot be empty".to_string(),
            ));
        }
        if article.id.trim().is_empty() {
            return Err(ApiError::BadRequest(
                "Article id cannot be empty".to_string(),
            ));
        }
        if article.url.trim().is_empty() {
            return Err(ApiError::BadRequest(
                "Article url cannot be empty".to_string(),
            ));
        }
    }

    let client = client.lock().unwrap();

    // Set feed metadata
    client.storage.set_feed_metadata(&feed_name, &feed)?;

    // Store each article
    for article in &feed.articles {
        client.storage.store_article(&feed_name, article.clone())?;
        println!("Added article '{}' to feed '{}'", article.title, feed_name);
    }

    println!("Successfully updated feed '{}'", feed_name);
    Ok(StatusCode::OK)
}

/// GET /v1/feeds/
///
/// Returns a JSON array of all stored feed names
pub async fn list_feeds<S: FeedStorage, G: FeedGenerator>(
    State(client): State<SharedClient<S, G>>,
) -> Result<Json<Vec<String>>, ApiError> {
    let client = client.lock().unwrap();
    let feeds = client.storage.list_feeds()?;
    Ok(Json(feeds))
}

/// GET /v1/rss/:feed_name
///
/// Returns the RSS feed for the specified feed name
pub async fn get_rss<S: FeedStorage, G: FeedGenerator>(
    Path(feed_name): Path<String>,
    State(client): State<SharedClient<S, G>>,
) -> Result<Response, ApiError> {
    // Validate feed name
    validate_feed_name(&feed_name)?;

    let client = client.lock().unwrap();

    // Get the feed
    let feed = client.storage.get_feed(&feed_name)?;

    // Generate RSS
    let rss_content = client.generator.generate(&feed)?;
    let mime_type = client.generator.mime_type();

    Ok(([(axum::http::header::CONTENT_TYPE, mime_type)], rss_content).into_response())
}

/// Custom error type for API handlers
#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    NotFound(String),
    InternalError(String),
}

impl From<String> for ApiError {
    fn from(s: String) -> Self {
        ApiError::BadRequest(s)
    }
}

impl From<crate::models::ValidationError> for ApiError {
    fn from(err: crate::models::ValidationError) -> Self {
        ApiError::BadRequest(err.to_string())
    }
}

impl From<crate::storage::StorageError> for ApiError {
    fn from(err: crate::storage::StorageError) -> Self {
        use crate::storage::StorageError;
        match err {
            StorageError::FeedEmpty => ApiError::NotFound("Feed is empty".to_string()),
            StorageError::Io(e) => {
                // Check if it's a "file not found" error
                if e.kind() == std::io::ErrorKind::NotFound {
                    ApiError::NotFound(format!("Feed not found: {}", e))
                } else {
                    ApiError::InternalError(format!("IO error: {}", e))
                }
            }
            StorageError::Json(e) => ApiError::BadRequest(format!("JSON error: {}", e)),
            StorageError::InvalidFeedName(name) => {
                ApiError::BadRequest(format!("Invalid feed name: {}", name))
            }
        }
    }
}

impl From<crate::generator::GeneratorError> for ApiError {
    fn from(err: crate::generator::GeneratorError) -> Self {
        match err {
            crate::generator::GeneratorError::Serialization(msg) => {
                ApiError::InternalError(format!("Generator error: {}", msg))
            }
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        (status, message).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_feed_name() {
        // Valid names
        assert!(validate_feed_name("myFeed").is_ok());
        assert!(validate_feed_name("my-feed").is_ok());
        assert!(validate_feed_name("my_feed").is_ok());
        assert!(validate_feed_name("feed123").is_ok());

        // Invalid names
        assert!(validate_feed_name("").is_err());
        assert!(validate_feed_name("my/feed").is_err());
        assert!(validate_feed_name("my\\feed").is_err());
        assert!(validate_feed_name("../feed").is_err());
        assert!(validate_feed_name("my feed").is_err());
        assert!(validate_feed_name("my@feed").is_err());
    }
}
