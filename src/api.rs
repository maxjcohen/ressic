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

/// POST /v1/feeds/:feed_name
///
/// Adds articles to a feed. The request body should contain a Feed object with
/// metadata and a list of articles. Feed metadata will be replaced if the feed
/// already exists.
pub async fn post_feed<S: FeedStorage, G: FeedGenerator>(
    Path(feed_name): Path<String>,
    State(client): State<SharedClient<S, G>>,
    Json(raw_feed): Json<Feed>,
) -> Result<StatusCode, ApiError> {
    // Validate articles - construct validated Article instances
    let mut validated_articles = Vec::new();
    for raw_article in raw_feed.articles {
        let article = crate::models::Article::new(
            raw_article.title,
            raw_article.content,
            raw_article.id,
            raw_article.url,
            raw_article.summary,
            raw_article.pub_date,
        )?;
        validated_articles.push(article);
    }

    // Validate feed metadata and construct validated Feed
    // Note: feed_name from path must match or we use the path parameter
    let validated_feed = crate::models::Feed::new(
        feed_name.clone(),
        raw_feed.title,
        raw_feed.link,
        raw_feed.description,
        validated_articles,
    )?;

    let client = client.lock().unwrap();

    // Set feed metadata
    client
        .storage
        .set_feed_metadata(&feed_name, &validated_feed)?;

    // Store each article
    for article in &validated_feed.articles {
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
    let client = client.lock().unwrap();

    // Get the feed - storage layer will validate feed name
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
