//! Ressic - A minimal RSS feed management library.
//!
//! This library provides a simple client interface for managing RSS feeds,
//! with pluggable storage backends and feed format generators.

pub mod generator;
pub mod models;
pub mod storage;

use crate::{
    generator::{FeedGenerator, GeneratorError},
    models::Article,
    storage::{FeedStorage, StorageError},
};

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
    /// If an article with the same ID already exists, it will be replaced.
    ///
    /// # Arguments
    ///
    /// * `feed` - The name of the feed to store the article in
    /// * `article` - The article to store
    ///
    /// # Errors
    ///
    /// Returns a `StorageError` if the article cannot be stored.
    pub fn store_article(&mut self, feed_name: &str, article: Article) -> Result<(), StorageError> {
        self.storage.store_article(feed_name, article)
    }

    /// Generates an RSS feed (not yet implemented).
    pub fn generate_feed(&self, feed_name: &str) -> Result<String, GeneratorError> {
        self.generator
            .generate(&self.storage.get_feed(feed_name).unwrap())
    }
}
