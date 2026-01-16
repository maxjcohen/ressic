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
            Article {
                title: title.into(),
                content: content.into(),
                id: id.into(),
                url: "https://example.com/article".into(),
                summary: "Test summary".into(),
                pub_date,
            },
        )
        .expect("store_article failed");

    let feed = storage.get_feed("test").expect("get_feed failed");
    // Expect the stored article to be present.
    assert_eq!(
        feed.articles.len(),
        1,
        "expected exactly one stored article"
    );
    let expected = Article {
        title: title.into(),
        content: content.into(),
        id: id.into(),
        url: "https://example.com/article".into(),
        summary: "Test summary".into(),
        pub_date,
    };
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
            Article {
                title: "unique".into(),
                content: "body".into(),
                id: "100".into(),
                url: "https://example.com/unique".into(),
                summary: "unique summary".into(),
                pub_date: Utc.with_ymd_and_hms(2024, 2, 1, 12, 0, 0).unwrap(),
            },
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
            Article {
                title: "First title".into(),
                content: "First content".into(),
                id: "1".into(),
                url: url.into(),
                summary: "First summary".into(),
                pub_date: pub_date1,
            },
        )
        .expect("store_article failed");

    let pub_date2 = Utc.with_ymd_and_hms(2024, 3, 10, 9, 0, 0).unwrap();
    // Store second article with same URL but different id "2"
    storage
        .store_article(
            "test",
            Article {
                title: "Second title".into(),
                content: "Second content".into(),
                id: "2".into(),
                url: url.into(),
                summary: "Second summary".into(),
                pub_date: pub_date2,
            },
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
    let expected = Article {
        title: "Second title".into(),
        content: "Second content".into(),
        id: "2".into(),
        url: url.into(),
        summary: "Second summary".into(),
        pub_date: pub_date2,
    };
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
            Article {
                title: "test".into(),
                content: "test".into(),
                id: "1".into(),
                url: "https://example.com/test".into(),
                summary: "test summary".into(),
                pub_date: Utc.with_ymd_and_hms(2024, 4, 1, 14, 0, 0).unwrap(),
            },
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