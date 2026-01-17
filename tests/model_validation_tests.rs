use chrono::Utc;
use ressic::models::{Article, Feed, ValidationError};

// ============================================================================
// Article Validation Tests
// ============================================================================

#[test]
fn test_article_new_success() {
    let result = Article::new(
        "Test Article".to_string(),
        "This is the content".to_string(),
        "article-123".to_string(),
        "http://example.com/article".to_string(),
        "This is the summary".to_string(),
        Utc::now(),
    );

    assert!(result.is_ok());
    let article = result.unwrap();
    assert_eq!(article.title, "Test Article");
    assert_eq!(article.content, "This is the content");
    assert_eq!(article.id, "article-123");
    assert_eq!(article.url, "http://example.com/article");
    assert_eq!(article.summary, "This is the summary");
}

#[test]
fn test_article_new_with_empty_summary() {
    // Summary can be empty
    let result = Article::new(
        "Test Article".to_string(),
        "Content".to_string(),
        "id-1".to_string(),
        "http://example.com".to_string(),
        "".to_string(),
        Utc::now(),
    );

    assert!(result.is_ok());
}

#[test]
fn test_article_new_with_whitespace_trimmed() {
    let result = Article::new(
        "  Test Article  ".to_string(),
        "  Content  ".to_string(),
        "  id-1  ".to_string(),
        "  http://example.com  ".to_string(),
        "  Summary  ".to_string(),
        Utc::now(),
    );

    assert!(result.is_ok());
    let article = result.unwrap();
    assert_eq!(article.title, "Test Article");
    assert_eq!(article.content, "Content");
    assert_eq!(article.id, "id-1");
    assert_eq!(article.url, "http://example.com");
    assert_eq!(article.summary, "Summary");
}

#[test]
fn test_article_new_empty_title() {
    let result = Article::new(
        "".to_string(),
        "Content".to_string(),
        "id-1".to_string(),
        "http://example.com".to_string(),
        "Summary".to_string(),
        Utc::now(),
    );

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(
        err,
        ValidationError::EmptyField {
            field: "title".to_string()
        }
    );
}

#[test]
fn test_article_new_whitespace_only_title() {
    let result = Article::new(
        "   ".to_string(),
        "Content".to_string(),
        "id-1".to_string(),
        "http://example.com".to_string(),
        "Summary".to_string(),
        Utc::now(),
    );

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        ValidationError::EmptyField {
            field: "title".to_string()
        }
    );
}

#[test]
fn test_article_new_empty_content() {
    let result = Article::new(
        "Title".to_string(),
        "".to_string(),
        "id-1".to_string(),
        "http://example.com".to_string(),
        "Summary".to_string(),
        Utc::now(),
    );

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        ValidationError::EmptyField {
            field: "content".to_string()
        }
    );
}

#[test]
fn test_article_new_empty_id() {
    let result = Article::new(
        "Title".to_string(),
        "Content".to_string(),
        "".to_string(),
        "http://example.com".to_string(),
        "Summary".to_string(),
        Utc::now(),
    );

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        ValidationError::EmptyField {
            field: "id".to_string()
        }
    );
}

#[test]
fn test_article_new_empty_url() {
    let result = Article::new(
        "Title".to_string(),
        "Content".to_string(),
        "id-1".to_string(),
        "".to_string(),
        "Summary".to_string(),
        Utc::now(),
    );

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        ValidationError::EmptyField {
            field: "url".to_string()
        }
    );
}

// ============================================================================
// Feed Validation Tests
// ============================================================================

#[test]
fn test_feed_new_success() {
    let result = Feed::new(
        "my-feed".to_string(),
        "My Feed".to_string(),
        "http://example.com".to_string(),
        "A test feed".to_string(),
        vec![],
    );

    assert!(result.is_ok());
    let feed = result.unwrap();
    assert_eq!(feed.name, "my-feed");
    assert_eq!(feed.title, "My Feed");
    assert_eq!(feed.link, "http://example.com");
    assert_eq!(feed.description, "A test feed");
    assert_eq!(feed.articles.len(), 0);
}

#[test]
fn test_feed_new_with_articles() {
    let article = Article::new(
        "Article Title".to_string(),
        "Article Content".to_string(),
        "id-1".to_string(),
        "http://example.com/article".to_string(),
        "Summary".to_string(),
        Utc::now(),
    )
    .unwrap();

    let result = Feed::new(
        "my-feed".to_string(),
        "My Feed".to_string(),
        "http://example.com".to_string(),
        "A test feed".to_string(),
        vec![article],
    );

    assert!(result.is_ok());
    let feed = result.unwrap();
    assert_eq!(feed.articles.len(), 1);
}

#[test]
fn test_feed_new_with_whitespace_trimmed() {
    let result = Feed::new(
        "  my-feed  ".to_string(),
        "  My Feed  ".to_string(),
        "  http://example.com  ".to_string(),
        "  A test feed  ".to_string(),
        vec![],
    );

    assert!(result.is_ok());
    let feed = result.unwrap();
    assert_eq!(feed.name, "my-feed");
    assert_eq!(feed.title, "My Feed");
    assert_eq!(feed.link, "http://example.com");
    assert_eq!(feed.description, "A test feed");
}

#[test]
fn test_feed_new_empty_name() {
    let result = Feed::new(
        "".to_string(),
        "My Feed".to_string(),
        "http://example.com".to_string(),
        "A test feed".to_string(),
        vec![],
    );

    assert!(result.is_err());
    let err = result.unwrap_err();
    match err {
        ValidationError::InvalidFeedName { name, reason } => {
            assert_eq!(name, "");
            assert!(reason.contains("empty"));
        }
        _ => panic!("Expected InvalidFeedName error"),
    }
}

#[test]
fn test_feed_new_empty_title() {
    let result = Feed::new(
        "my-feed".to_string(),
        "".to_string(),
        "http://example.com".to_string(),
        "A test feed".to_string(),
        vec![],
    );

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        ValidationError::EmptyField {
            field: "title".to_string()
        }
    );
}

#[test]
fn test_feed_new_empty_link() {
    let result = Feed::new(
        "my-feed".to_string(),
        "My Feed".to_string(),
        "".to_string(),
        "A test feed".to_string(),
        vec![],
    );

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        ValidationError::EmptyField {
            field: "link".to_string()
        }
    );
}

#[test]
fn test_feed_new_empty_description() {
    let result = Feed::new(
        "my-feed".to_string(),
        "My Feed".to_string(),
        "http://example.com".to_string(),
        "".to_string(),
        vec![],
    );

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        ValidationError::EmptyField {
            field: "description".to_string()
        }
    );
}

// ============================================================================
// Feed Name Character Validation Tests
// ============================================================================

#[test]
fn test_feed_name_valid_alphanumeric() {
    let result = Feed::new(
        "feed123".to_string(),
        "My Feed".to_string(),
        "http://example.com".to_string(),
        "A test feed".to_string(),
        vec![],
    );

    assert!(result.is_ok());
}

#[test]
fn test_feed_name_valid_with_hyphens() {
    let result = Feed::new(
        "my-feed-name".to_string(),
        "My Feed".to_string(),
        "http://example.com".to_string(),
        "A test feed".to_string(),
        vec![],
    );

    assert!(result.is_ok());
}

#[test]
fn test_feed_name_valid_with_underscores() {
    let result = Feed::new(
        "my_feed_name".to_string(),
        "My Feed".to_string(),
        "http://example.com".to_string(),
        "A test feed".to_string(),
        vec![],
    );

    assert!(result.is_ok());
}

#[test]
fn test_feed_name_invalid_with_slash() {
    let result = Feed::new(
        "my/feed".to_string(),
        "My Feed".to_string(),
        "http://example.com".to_string(),
        "A test feed".to_string(),
        vec![],
    );

    assert!(result.is_err());
    match result.unwrap_err() {
        ValidationError::InvalidFeedName { name, reason } => {
            assert_eq!(name, "my/feed");
            assert!(reason.contains("alphanumeric") || reason.contains("character"));
        }
        _ => panic!("Expected InvalidFeedName error"),
    }
}

#[test]
fn test_feed_name_invalid_with_backslash() {
    let result = Feed::new(
        "my\\feed".to_string(),
        "My Feed".to_string(),
        "http://example.com".to_string(),
        "A test feed".to_string(),
        vec![],
    );

    assert!(result.is_err());
    match result.unwrap_err() {
        ValidationError::InvalidFeedName { .. } => {}
        _ => panic!("Expected InvalidFeedName error"),
    }
}

#[test]
fn test_feed_name_invalid_with_dot_dot() {
    let result = Feed::new(
        "my..feed".to_string(),
        "My Feed".to_string(),
        "http://example.com".to_string(),
        "A test feed".to_string(),
        vec![],
    );

    assert!(result.is_err());
    match result.unwrap_err() {
        ValidationError::InvalidFeedName { .. } => {}
        _ => panic!("Expected InvalidFeedName error"),
    }
}

#[test]
fn test_feed_name_invalid_with_space() {
    let result = Feed::new(
        "my feed".to_string(),
        "My Feed".to_string(),
        "http://example.com".to_string(),
        "A test feed".to_string(),
        vec![],
    );

    assert!(result.is_err());
    match result.unwrap_err() {
        ValidationError::InvalidFeedName { .. } => {}
        _ => panic!("Expected InvalidFeedName error"),
    }
}

#[test]
fn test_feed_name_invalid_with_special_chars() {
    let result = Feed::new(
        "my@feed".to_string(),
        "My Feed".to_string(),
        "http://example.com".to_string(),
        "A test feed".to_string(),
        vec![],
    );

    assert!(result.is_err());
    match result.unwrap_err() {
        ValidationError::InvalidFeedName { .. } => {}
        _ => panic!("Expected InvalidFeedName error"),
    }
}
