use super::{FeedGenerator, GeneratorError};
use crate::models::Feed;

/// RSS 2.0 feed generator.
///
/// Generates valid RSS 2.0 XML documents conforming to the RSS 2.0 specification.
/// Properly escapes XML entities and formats dates in RFC 2822 format.
pub struct Rss20;

impl Rss20 {
    pub fn new() -> Self {
        Rss20
    }
}

impl Default for Rss20 {
    fn default() -> Self {
        Self::new()
    }
}

impl FeedGenerator for Rss20 {
    fn generate(&self, feed: &Feed) -> Result<String, GeneratorError> {
        let mut output = String::new();

        // XML declaration
        output.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");

        // RSS root element
        output.push_str("<rss version=\"2.0\">\n");
        output.push_str("  <channel>\n");

        // Channel metadata
        output.push_str(&format!("    <title>{}</title>\n", escape_xml(&feed.title)));
        output.push_str(&format!("    <link>{}</link>\n", escape_xml(&feed.link)));
        output.push_str(&format!(
            "    <description>{}</description>\n",
            escape_xml(&feed.description)
        ));

        // Items (articles)
        for article in &feed.articles {
            output.push_str("    <item>\n");
            output.push_str(&format!(
                "      <title>{}</title>\n",
                escape_xml(&article.title)
            ));
            output.push_str(&format!(
                "      <link>{}</link>\n",
                escape_xml(&article.url)
            ));
            output.push_str(&format!(
                "      <description>{}</description>\n",
                escape_xml(&article.summary)
            ));
            output.push_str(&format!("      <guid>{}</guid>\n", escape_xml(&article.id)));

            // Format date in RFC 2822 format
            let pub_date = article.pub_date.format("%a, %d %b %Y %H:%M:%S %z");
            output.push_str(&format!("      <pubDate>{}</pubDate>\n", pub_date));

            output.push_str("    </item>\n");
        }

        // Close tags
        output.push_str("  </channel>\n");
        output.push_str("</rss>\n");

        Ok(output)
    }

    fn mime_type(&self) -> &'static str {
        "application/rss+xml"
    }
}

/// Escapes special XML characters in text content.
fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml_escaping() {
        assert_eq!(escape_xml("Hello World"), "Hello World");
        assert_eq!(escape_xml("A & B"), "A &amp; B");
        assert_eq!(escape_xml("<tag>"), "&lt;tag&gt;");
        assert_eq!(escape_xml("\"quoted\""), "&quot;quoted&quot;");
        assert_eq!(escape_xml("'apostrophe'"), "&apos;apostrophe&apos;");
        assert_eq!(
            escape_xml("<tag attr=\"value\">text & more</tag>"),
            "&lt;tag attr=&quot;value&quot;&gt;text &amp; more&lt;/tag&gt;"
        );
    }
}
