//! Ressic - A minimal RSS feed management library.
//!
//! This library provides a simple client interface for managing RSS feeds,
//! with pluggable storage backends and feed format generators.

pub mod api;
pub mod generator;
pub mod models;
pub mod storage;

use crate::{
    generator::{FeedGenerator, GeneratorError},
    models::Article,
    storage::{FeedStorage, StorageError},
};
use axum::{Router, routing::get, routing::post};
use std::sync::{Arc, Mutex};

/// Errors that can occur in client operations, wrapping storage and generator errors.
#[derive(Debug)]
pub enum ClientError {
    /// A storage-layer error occurred.
    Storage(StorageError),
    /// A feed generation error occurred.
    Generator(GeneratorError),
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::Storage(e) => write!(f, "Storage error: {}", e),
            ClientError::Generator(e) => write!(f, "Generator error: {}", e),
        }
    }
}

impl std::error::Error for ClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ClientError::Storage(e) => Some(e),
            ClientError::Generator(e) => Some(e),
        }
    }
}

impl From<StorageError> for ClientError {
    fn from(e: StorageError) -> Self {
        ClientError::Storage(e)
    }
}

impl From<GeneratorError> for ClientError {
    fn from(e: GeneratorError) -> Self {
        ClientError::Generator(e)
    }
}

/// Client for managing RSS feeds.
///
/// The client provides a high-level interface for storing and retrieving
/// articles, abstracting over the specific storage implementation.
pub struct Client<S: FeedStorage, G: FeedGenerator> {
    /// The storage backend used by this client.
    pub storage: S,
    /// The feed generator.
    pub generator: G,
}

impl<S: FeedStorage, G: FeedGenerator> Client<S, G> {
    /// Creates a new client with the specified storage backend.
    ///
    /// # Arguments
    ///
    /// * `storage` - The storage backend to use for persisting articles
    pub fn new(storage: S, generator: G) -> Self {
        Client {
            storage: storage,
            generator: generator,
        }
    }

    pub fn list_feeds(&self) -> Result<Vec<String>, StorageError> {
        self.storage.list_feeds()
    }

    /// Stores an article in the specified feed.
    ///
    /// If an article with the same URL already exists, it will be replaced.
    ///
    /// # Arguments
    ///
    /// * `feed_name` - The name of the feed to store the article in.
    /// * `article` - The article to store. Must be pre-validated using `Article::new()`.
    ///
    /// # Errors
    ///
    /// Returns a `StorageError` if the article cannot be stored.
    ///
    /// # Note
    ///
    /// The client expects pre-validated models. Articles should be created using
    /// `Article::new()` to ensure all validation rules are applied before storage.
    pub fn store_article(&mut self, feed_name: &str, article: Article) -> Result<(), StorageError> {
        self.storage.store_article(feed_name, article)
    }

    /// Generates an RSS feed for the specified feed name.
    ///
    /// # Errors
    ///
    /// Returns `ClientError::Storage` if the feed cannot be retrieved, or
    /// `ClientError::Generator` if feed generation fails.
    pub fn generate_feed(&self, feed_name: &str) -> Result<String, ClientError> {
        let feed = self.storage.get_feed(feed_name)?;
        let output = self.generator.generate(&feed)?;
        Ok(output)
    }
}

/// Creates the Axum router for the API server
///
/// # Arguments
///
/// * `client` - The shared client instance to use for all requests
pub fn create_app<S: FeedStorage + Send + 'static, G: FeedGenerator + Send + 'static>(
    client: Arc<Mutex<Client<S, G>>>,
) -> Router {
    Router::new()
        .route("/v1/feeds/:feed_name", post(api::post_feed::<S, G>))
        .route("/v1/feeds/", get(api::list_feeds::<S, G>))
        .route("/v1/rss/:feed_name", get(api::get_rss::<S, G>))
        .with_state(client)
}
