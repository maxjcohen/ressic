use super::{FeedGenerator, GeneratorError};
use crate::models::Feed;

/// Mock feed generator.
pub struct Mock;

impl Mock {
    pub fn new() -> Self {
        Mock
    }
}

impl Default for Mock {
    fn default() -> Self {
        Self::new()
    }
}

impl FeedGenerator for Mock {
    fn generate(&self, _feed: &Feed) -> Result<String, GeneratorError> {
        Ok(String::from("This is a mock feed"))
    }

    fn mime_type(&self) -> &'static str {
        "text/plain"
    }
}
