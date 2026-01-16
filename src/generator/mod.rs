use crate::models::Feed;

pub mod mock;
pub mod plain_text;

/// Errors that can occur during feed generation operations.
#[derive(Debug)]
pub enum GeneratorError {
    /// An error occurred while serializing feed data.
    Serialization(String),
}

/// Trait for feed format generators.
///
/// Implementations of this trait convert article collections into
/// various feed formats (RSS 2.0, Atom, etc.). Each generator is
/// responsible for formatting articles according to its specific
/// feed format specification.
pub trait FeedGenerator {
    /// Generates a feed document from a stored semantic feed.
    ///
    /// # Arguments
    ///
    /// * `feed` - Feed object containing metadata and articles
    ///
    /// # Returns
    ///
    /// A string containing the complete feed document in the
    /// generator's format.
    ///
    /// # Errors
    ///
    /// Returns `GeneratorError::Serialization` if feed generation fails.
    fn generate(&self, feed: &Feed) -> Result<String, GeneratorError>;

    /// Returns the MIME type for this feed format.
    ///
    /// Examples:
    /// - RSS 2.0: "application/rss+xml"
    /// - Atom: "application/atom+xml"
    /// - JSON Feed: "application/feed+json"
    fn mime_type(&self) -> &'static str;

    /// Returns the format name/identifier.
    ///
    /// Examples: "rss-2.0", "atom-1.0", "json-feed"
    fn format_name(&self) -> &'static str;
}

pub use mock::Mock;
pub use plain_text::PlainText;
