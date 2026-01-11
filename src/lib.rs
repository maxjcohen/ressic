//! Ressic - A minimal RSS feed management library.
//!
//! This library provides a simple client interface for managing RSS feeds,
//! with pluggable storage backends.

pub mod models;
pub mod storage;

use crate::{models::Article, storage::{FeedStorage, StorageError}};

/// Client for managing RSS feeds.
///
/// The client provides a high-level interface for storing and retrieving
/// articles, abstracting over the specific storage implementation.
pub struct Client<S: FeedStorage> {
    /// The storage backend used by this client.
    pub storage: S,
}

impl<S: FeedStorage> Client<S> {
    /// Creates a new client with the specified storage backend.
    ///
    /// # Arguments
    ///
    /// * `storage` - The storage backend to use for persisting articles
    pub fn new(storage: S) -> Self {
        Client { storage: storage }
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
    pub fn store_article(&mut self, feed: &str, article: Article) -> Result<(), StorageError> {
        self.storage.store_article(feed, article)
    }

    /// Retrieves all articles from the specified feed.
    ///
    /// # Arguments
    ///
    /// * `feed` - The name of the feed to retrieve articles from
    ///
    /// # Errors
    ///
    /// Returns a `StorageError` if articles cannot be retrieved.
    pub fn get_articles(&self, feed: &str) -> Result<Vec<Article>, StorageError> {
        self.storage.get_all_articles(feed)
    }

    /// Generates an RSS feed (not yet implemented).
    pub fn generate_feed(&self) -> String {
        String::from("")
    }
}
