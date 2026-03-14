mod common;

use crate::common::with_localfile_storage;
use chrono::{TimeZone, Utc};
use ressic::{
    models::{Article, Feed},
    storage::{FeedStorage, StorageError},
};

fn make_article(title: &str, content: &str, id: &str, url: &str) -> Article {
    Article::new(
        title.into(),
        content.into(),
        id.into(),
        url.into(),
        "summary".into(),
        Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
    )
    .unwrap()
}

fn make_feed(name: &str, articles: Vec<Article>) -> Feed {
    Feed::new(
        name.into(),
        format!("{} title", name),
        "https://example.com".into(),
        format!("{} description", name),
        articles,
    )
    .unwrap()
}

// After put_feed, get_feed should return the stored feed with all articles and metadata.
fn assert_put_then_get<S: FeedStorage>(storage: S) {
    let article = make_article(
        "Real title",
        "Real content",
        "42",
        "https://example.com/article",
    );
    let feed = make_feed("test", vec![article.clone()]);
    storage.put_feed(&feed).expect("put_feed failed");

    let retrieved = storage.get_feed("test").expect("get_feed failed");
    assert_eq!(
        retrieved.articles.len(),
        1,
        "expected exactly one stored article"
    );
    assert_eq!(retrieved.articles[0], article);
    assert_eq!(retrieved.title, feed.title);
    assert_eq!(retrieved.link, feed.link);
    assert_eq!(retrieved.description, feed.description);
}

#[test]
fn localfile_put_then_get() {
    with_localfile_storage("put_then_get", |storage| {
        assert_put_then_get(storage);
    });
}

// Test that get_latest_article returns FeedNotFound error for non-existent feeds.
fn assert_empty_feed_error<S: FeedStorage>(storage: S) {
    let result = storage.get_feed("nonexistent");
    assert!(matches!(result, Err(StorageError::FeedNotFound)));
}

#[test]
fn localfile_empty_feed_error() {
    with_localfile_storage("empty_feed_error", |storage| {
        assert_empty_feed_error(storage);
    });
}

// put_feed should update feed metadata on a second call.
fn assert_put_updates_metadata<S: FeedStorage>(storage: S) {
    let article = make_article("Article 1", "Content 1", "1", "https://example.com/1");
    let feed_v1 = Feed::new(
        "test".into(),
        "Original Title".into(),
        "https://example.com".into(),
        "Original Description".into(),
        vec![article.clone()],
    )
    .unwrap();
    storage.put_feed(&feed_v1).expect("put_feed v1 failed");

    let feed_v2 = Feed::new(
        "test".into(),
        "Updated Title".into(),
        "https://example.com/v2".into(),
        "Updated Description".into(),
        vec![article],
    )
    .unwrap();
    storage.put_feed(&feed_v2).expect("put_feed v2 failed");

    let retrieved = storage.get_feed("test").expect("get_feed failed");
    assert_eq!(retrieved.title, "Updated Title");
    assert_eq!(retrieved.link, "https://example.com/v2");
    assert_eq!(retrieved.description, "Updated Description");
}

#[test]
fn localfile_put_updates_metadata() {
    with_localfile_storage("put_updates_metadata", |storage| {
        assert_put_updates_metadata(storage);
    });
}

// When storing in one feed, another feed should remain empty.
fn assert_isolated_between_feeds<S: FeedStorage>(storage: S) {
    let article = make_article("unique", "body", "100", "https://example.com/unique");
    let feed_one = make_feed("feed_one", vec![article]);
    storage.put_feed(&feed_one).expect("put_feed failed");

    // Read from a different feed; expect FeedNotFound error
    let result = storage.get_feed("feed_two");
    assert!(matches!(result, Err(StorageError::FeedNotFound)));
    // Also verify that feed_one has the article
    let retrieved_one = storage.get_feed("feed_one").expect("get_feed failed");
    assert_eq!(retrieved_one.articles.len(), 1);
    assert_eq!(retrieved_one.articles[0].title, "unique");
}

#[test]
fn localfile_isolated_between_feeds() {
    with_localfile_storage("isolated_between_feeds", |storage| {
        assert_isolated_between_feeds(storage);
    });
}

// put_feed deduplicates articles by URL across calls; incoming wins on conflict.
fn assert_put_deduplicates_by_url<S: FeedStorage>(storage: S) {
    let url = "https://example.com/article";
    let pub_date1 = Utc.with_ymd_and_hms(2024, 3, 10, 8, 0, 0).unwrap();
    let a1 = Article::new(
        "First title".into(),
        "First content".into(),
        "1".into(),
        url.into(),
        "First summary".into(),
        pub_date1,
    )
    .unwrap();
    storage
        .put_feed(&make_feed("test", vec![a1]))
        .expect("put_feed 1 failed");

    let pub_date2 = Utc.with_ymd_and_hms(2024, 3, 10, 9, 0, 0).unwrap();
    let a2 = Article::new(
        "Second title".into(),
        "Second content".into(),
        "2".into(),
        url.into(), // same URL
        "Second summary".into(),
        pub_date2,
    )
    .unwrap();
    storage
        .put_feed(&make_feed("test", vec![a2.clone()]))
        .expect("put_feed 2 failed");

    let feed = storage.get_feed("test").expect("get_feed failed");
    assert_eq!(
        feed.articles.len(),
        1,
        "expected exactly one article after deduplication by URL"
    );
    assert_eq!(
        &feed.articles[0], &a2,
        "incoming article should win on URL conflict"
    );
}

#[test]
fn localfile_put_deduplicates_by_url() {
    with_localfile_storage("put_deduplicates_by_url", |storage| {
        assert_put_deduplicates_by_url(storage);
    });
}

// put_feed with an invalid name in Feed.name should return InvalidFeedName.
fn assert_invalid_feed_names<S: FeedStorage>(storage: S) {
    let invalid_names = vec![
        "../etc/passwd",
        "../../sensitive",
        "./hidden",
        "path/with/slash",
        "path\\with\\backslash",
        "",
        ".hidden",
    ];

    for name in invalid_names {
        // Bypass Feed::new() to test storage-layer validation directly.
        let feed = Feed {
            name: name.to_string(),
            title: "test".to_string(),
            link: "https://example.com".to_string(),
            description: "test".to_string(),
            articles: vec![],
        };
        let result = storage.put_feed(&feed);
        assert!(
            matches!(result, Err(StorageError::InvalidFeedName(_))),
            "Expected InvalidFeedName error for feed name: {:?}",
            name
        );
    }
}

#[test]
fn localfile_rejects_invalid_feed_names() {
    with_localfile_storage("invalid_names", |storage| {
        assert_invalid_feed_names(storage);
    });
}

// list_feeds should return all feed names after multiple put_feed calls.
fn assert_list_feeds<S: FeedStorage>(storage: S) {
    // Initially, there should be no feeds
    let feeds = storage.list_feeds().expect("list_feeds failed");
    assert_eq!(feeds.len(), 0, "expected no feeds initially");

    for (name, url_suffix) in [("feed_a", "a"), ("feed_b", "b"), ("feed_c", "c")] {
        let article = make_article(
            &format!("Article {}", name),
            "content",
            name,
            &format!("https://example.com/{}", url_suffix),
        );
        storage
            .put_feed(&make_feed(name, vec![article]))
            .expect("put_feed failed");
    }

    let mut feeds = storage.list_feeds().expect("list_feeds failed");
    feeds.sort();
    assert_eq!(feeds.len(), 3, "expected exactly three feeds");
    assert_eq!(feeds, vec!["feed_a", "feed_b", "feed_c"]);
}

#[test]
fn localfile_list_feeds() {
    with_localfile_storage("list_feeds", |storage| {
        assert_list_feeds(storage);
    });
}
