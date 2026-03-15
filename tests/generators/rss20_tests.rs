use chrono::{TimeZone, Utc};
use ressic::generator::FeedGenerator;
use ressic::generator::Rss20;
use ressic::models::{Article, Feed};

#[test]
fn test_generate() {
    super::test_generate(&Rss20::new());
}

#[test]
fn test_rss_basic_structure() {
    let generator = Rss20::new();
    let feed = Feed::new(
        "test".into(),
        "Test Feed".into(),
        "https://example.com".into(),
        "A test feed".into(),
        vec![],
    )
    .unwrap();

    let output = generator.generate(&feed).expect("Failed to generate RSS");

    // Verify XML declaration
    assert!(output.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));

    // Verify RSS 2.0 element
    assert!(output.contains("<rss version=\"2.0\">"));
    assert!(output.contains("</rss>"));

    // Verify channel element
    assert!(output.contains("<channel>"));
    assert!(output.contains("</channel>"));
}

#[test]
fn test_rss_channel_metadata() {
    let generator = Rss20::new();
    let feed = Feed::new(
        "news".into(),
        "News Feed".into(),
        "https://news.example.com".into(),
        "Latest news and updates".into(),
        vec![],
    )
    .unwrap();

    let output = generator.generate(&feed).expect("Failed to generate RSS");

    // Verify required channel elements
    assert!(output.contains("<title>News Feed</title>"));
    assert!(output.contains("<link>https://news.example.com</link>"));
    assert!(output.contains("<description>Latest news and updates</description>"));
}

#[test]
fn test_rss_single_article() {
    let generator = Rss20::new();
    let article = Article::new(
        "First Article".into(),
        "This is the content of the first article.".into(),
        "article-1".into(),
        "https://example.com/article-1".into(),
        "A brief summary".into(),
        Utc.with_ymd_and_hms(2026, 1, 15, 12, 30, 0).unwrap(),
    )
    .unwrap();

    let feed = Feed::new(
        "test".into(),
        "Test Feed".into(),
        "https://example.com".into(),
        "Test feed description".into(),
        vec![article],
    )
    .unwrap();

    let output = generator.generate(&feed).expect("Failed to generate RSS");

    // Verify item element
    assert!(output.contains("<item>"));
    assert!(output.contains("</item>"));

    // Verify article title and link
    assert!(output.contains("<title>First Article</title>"));
    assert!(output.contains("<link>https://example.com/article-1</link>"));

    // Verify description (summary)
    assert!(output.contains("<description>A brief summary</description>"));

    // Verify guid
    assert!(output.contains("<guid>article-1</guid>"));

    // Verify pubDate in RFC 2822 format
    assert!(output.contains("<pubDate>"));
    assert!(output.contains("</pubDate>"));
}

#[test]
fn test_rss_multiple_articles() {
    let generator = Rss20::new();
    let articles = vec![
        Article::new(
            "Article One".into(),
            "Content one".into(),
            "1".into(),
            "https://example.com/1".into(),
            "Summary one".into(),
            Utc.with_ymd_and_hms(2026, 1, 15, 10, 0, 0).unwrap(),
        )
        .unwrap(),
        Article::new(
            "Article Two".into(),
            "Content two".into(),
            "2".into(),
            "https://example.com/2".into(),
            "Summary two".into(),
            Utc.with_ymd_and_hms(2026, 1, 15, 11, 0, 0).unwrap(),
        )
        .unwrap(),
        Article::new(
            "Article Three".into(),
            "Content three".into(),
            "3".into(),
            "https://example.com/3".into(),
            "Summary three".into(),
            Utc.with_ymd_and_hms(2026, 1, 15, 12, 0, 0).unwrap(),
        )
        .unwrap(),
    ];

    let feed = Feed::new(
        "multi".into(),
        "Multi Article Feed".into(),
        "https://example.com".into(),
        "Feed with multiple articles".into(),
        articles,
    )
    .unwrap();

    let output = generator.generate(&feed).expect("Failed to generate RSS");

    // Count item elements
    let item_count = output.matches("<item>").count();
    assert_eq!(item_count, 3, "Should have 3 items");

    // Verify all article titles are present
    assert!(output.contains("<title>Article One</title>"));
    assert!(output.contains("<title>Article Two</title>"));
    assert!(output.contains("<title>Article Three</title>"));
}

#[test]
fn test_rss_xml_escaping() {
    let generator = Rss20::new();
    let article = Article::new(
        "Article with <special> & \"chars\"".into(),
        "Content with <tags> & entities".into(),
        "special-1".into(),
        "https://example.com/special?param=value&other=123".into(),
        "Summary with <html> & \"quotes\"".into(),
        Utc.with_ymd_and_hms(2026, 1, 15, 12, 0, 0).unwrap(),
    )
    .unwrap();

    let feed = Feed::new(
        "test".into(),
        "Feed with <special> & \"characters\"".into(),
        "https://example.com".into(),
        "Description with & and < characters".into(),
        vec![article],
    )
    .unwrap();

    let output = generator.generate(&feed).expect("Failed to generate RSS");

    // Verify XML entities are escaped
    assert!(output.contains("&lt;") || output.contains("&amp;"));

    // Should not contain unescaped special characters in text content
    // (excluding XML structure tags)
    let lines_with_text: Vec<&str> = output
        .lines()
        .filter(|line| {
            line.contains("<title>") || line.contains("<description>") || line.contains("<link>")
        })
        .collect();

    for line in lines_with_text {
        if line.contains("<title>") && line.contains("</title>") {
            let content = line
                .split("<title>")
                .nth(1)
                .unwrap()
                .split("</title>")
                .next()
                .unwrap();
            if content.contains("special") {
                assert!(content.contains("&lt;") || content.contains("&amp;"));
            }
        }
    }
}

#[test]
fn test_rss_date_format() {
    let generator = Rss20::new();
    let article = Article::new(
        "Test Article".into(),
        "Content".into(),
        "1".into(),
        "https://example.com/1".into(),
        "Summary".into(),
        Utc.with_ymd_and_hms(2026, 1, 15, 14, 30, 45).unwrap(),
    )
    .unwrap();

    let feed = Feed::new(
        "test".into(),
        "Test Feed".into(),
        "https://example.com".into(),
        "Test".into(),
        vec![article],
    )
    .unwrap();

    let output = generator.generate(&feed).expect("Failed to generate RSS");

    // RFC 2822 date format should be like: Wed, 15 Jan 2026 14:30:45 +0000
    assert!(output.contains("<pubDate>"));

    // Extract the pubDate content
    let pub_date_start = output.find("<pubDate>").unwrap() + 9;
    let pub_date_end = output[pub_date_start..].find("</pubDate>").unwrap();
    let pub_date_str = &output[pub_date_start..pub_date_start + pub_date_end];

    // Verify it contains expected elements of RFC 2822 format
    assert!(pub_date_str.contains("Jan"));
    assert!(pub_date_str.contains("2026"));
}

#[test]
fn test_rss_mime_type() {
    let generator = Rss20::new();
    assert_eq!(generator.mime_type(), "application/rss+xml");
}

#[test]
fn test_rss_format_name() {
    let generator = Rss20::new();
    assert_eq!(generator.format_name(), "rss-2.0");
}

#[test]
fn test_rss_empty_feed() {
    let generator = Rss20::new();
    let feed = Feed::new(
        "empty".into(),
        "Empty Feed".into(),
        "https://example.com".into(),
        "A feed with no articles".into(),
        vec![],
    )
    .unwrap();

    let output = generator.generate(&feed).expect("Failed to generate RSS");

    // Should still be valid RSS with channel metadata
    assert!(output.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
    assert!(output.contains("<rss version=\"2.0\">"));
    assert!(output.contains("<channel>"));
    assert!(output.contains("<title>Empty Feed</title>"));

    // Should have no items
    assert!(!output.contains("<item>"));
}
