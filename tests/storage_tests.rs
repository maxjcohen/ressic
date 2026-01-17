mod common;

use crate::common::with_localfile_storage;
use chrono::{TimeZone, Utc};
use ressic::{
    models::Article,
    storage::{FeedStorage, StorageError},
};

// After storing an article, storage.get_feed() should return it.
fn assert_store_then_get_all<S: FeedStorage>(storage: S) {
    let title = "Real title";
    let content = "Real content";
    let id = "42";
    let pub_date = Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap();
    storage
        .store_article(
            "test",
            Article::new(
                title.into(),
                content.into(),
                id.into(),
                "https://example.com/article".into(),
                "Test summary".into(),
                pub_date,
            )
            .unwrap(),
        )
        .expect("store_article failed");

    let feed = storage.get_feed("test").expect("get_feed failed");
    // Expect the stored article to be present.
    assert_eq!(
        feed.articles.len(),
        1,
        "expected exactly one stored article"
    );
    let expected = Article::new(
        title.into(),
        content.into(),
        id.into(),
        "https://example.com/article".into(),
        "Test summary".into(),
        pub_date,
    )
    .unwrap();
    assert_eq!(&feed.articles[0], &expected);
}

#[test]
fn localfile_store_then_get_all() {
    with_localfile_storage("store_then_get_all", |storage| {
        assert_store_then_get_all(storage);
    });
}

// Test that get_latest_article returns FeedEmpty error for empty feeds.
fn assert_empty_feed_error<S: FeedStorage>(storage: S) {
    let result = storage.get_feed("empty_feed");
    assert!(matches!(result, Err(StorageError::FeedEmpty)));
}

#[test]
fn localfile_empty_feed_error() {
    with_localfile_storage("empty_feed_error", |storage| {
        assert_empty_feed_error(storage);
    });
}

// When storing an article in one feed, another feed should remain empty.
// This is extracted into a generic helper to match the other tests' style.
fn assert_isolated_between_feeds<S: FeedStorage>(storage: S) {
    storage
        .store_article(
            "feed_one",
            Article::new(
                "unique".into(),
                "body".into(),
                "100".into(),
                "https://example.com/unique".into(),
                "unique summary".into(),
                Utc.with_ymd_and_hms(2024, 2, 1, 12, 0, 0).unwrap(),
            )
            .unwrap(),
        )
        .expect("store_article failed");

    // Read from a different feed; expect FeedEmpty error
    let result = storage.get_feed("feed_two");
    assert!(matches!(result, Err(StorageError::FeedEmpty)));
    // Also verify that feed_one has the article
    let feed_one = storage.get_feed("feed_one").expect("get_feed failed");
    assert_eq!(feed_one.articles.len(), 1);
    assert_eq!(feed_one.articles[0].title, "unique");
}

#[test]
fn localfile_isolated_between_feeds() {
    with_localfile_storage("isolated_between_feeds", |storage| {
        assert_isolated_between_feeds(storage);
    });
}

// Test deduplication when storing an article with the same URL.
// Articles are deduplicated by URL, not by ID. Two articles with the same URL
// but different IDs should result in only one article (the most recent).
fn assert_deduplication<S: FeedStorage>(storage: S) {
    let url = "https://example.com/article";
    let pub_date1 = Utc.with_ymd_and_hms(2024, 3, 10, 8, 0, 0).unwrap();

    // Store first article with id "1"
    storage
        .store_article(
            "test",
            Article::new(
                "First title".into(),
                "First content".into(),
                "1".into(),
                url.into(),
                "First summary".into(),
                pub_date1,
            )
            .unwrap(),
        )
        .expect("store_article failed");

    let pub_date2 = Utc.with_ymd_and_hms(2024, 3, 10, 9, 0, 0).unwrap();
    // Store second article with same URL but different id "2"
    storage
        .store_article(
            "test",
            Article::new(
                "Second title".into(),
                "Second content".into(),
                "2".into(),
                url.into(),
                "Second summary".into(),
                pub_date2,
            )
            .unwrap(),
        )
        .expect("store_article failed");

    let feed = storage.get_feed("test").expect("get_feed failed");

    // Expect only one article because they have the same URL
    assert_eq!(
        feed.articles.len(),
        1,
        "expected exactly one article after deduplication by URL"
    );

    // The second article should have replaced the first
    let expected = Article::new(
        "Second title".into(),
        "Second content".into(),
        "2".into(),
        url.into(),
        "Second summary".into(),
        pub_date2,
    )
    .unwrap();
    assert_eq!(&feed.articles[0], &expected);
}

#[test]
fn localfile_deduplication() {
    with_localfile_storage("deduplication", |storage| {
        assert_deduplication(storage);
    });
}

// Test that invalid feed names are rejected
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
        let result = storage.store_article(
            name,
            Article::new(
                "test".into(),
                "test".into(),
                "1".into(),
                "https://example.com/test".into(),
                "test summary".into(),
                Utc.with_ymd_and_hms(2024, 4, 1, 14, 0, 0).unwrap(),
            )
            .unwrap(),
        );
        assert!(
            matches!(result, Err(StorageError::InvalidFeedName(_))),
            "Expected InvalidFeedName error for feed name: {}",
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

// Test that list_feeds returns all feed names
fn assert_list_feeds<S: FeedStorage>(storage: S) {
    // Initially, there should be no feeds
    let feeds = storage.list_feeds().expect("list_feeds failed");
    assert_eq!(feeds.len(), 0, "expected no feeds initially");

    // Store articles in multiple feeds
    storage
        .store_article(
            "feed_a",
            Article::new(
                "Article A".into(),
                "Content A".into(),
                "a1".into(),
                "https://example.com/a".into(),
                "Summary A".into(),
                Utc.with_ymd_and_hms(2024, 5, 1, 10, 0, 0).unwrap(),
            )
            .unwrap(),
        )
        .expect("store_article failed");

    storage
        .store_article(
            "feed_b",
            Article::new(
                "Article B".into(),
                "Content B".into(),
                "b1".into(),
                "https://example.com/b".into(),
                "Summary B".into(),
                Utc.with_ymd_and_hms(2024, 5, 2, 11, 0, 0).unwrap(),
            )
            .unwrap(),
        )
        .expect("store_article failed");

    storage
        .store_article(
            "feed_c",
            Article::new(
                "Article C".into(),
                "Content C".into(),
                "c1".into(),
                "https://example.com/c".into(),
                "Summary C".into(),
                Utc.with_ymd_and_hms(2024, 5, 3, 12, 0, 0).unwrap(),
            )
            .unwrap(),
        )
        .expect("store_article failed");

    // List feeds and verify all three are present
    let mut feeds = storage.list_feeds().expect("list_feeds failed");
    feeds.sort(); // Sort for consistent comparison
    assert_eq!(feeds.len(), 3, "expected exactly three feeds");
    assert_eq!(feeds, vec!["feed_a", "feed_b", "feed_c"]);
}

#[test]
fn localfile_list_feeds() {
    with_localfile_storage("list_feeds", |storage| {
        assert_list_feeds(storage);
    });
}
