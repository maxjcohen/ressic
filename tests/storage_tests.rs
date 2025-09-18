// Storage tests for FeedStorage trait. Test storing articles and retrieving them.

use std::fs;

use ressic::{
    models::Article,
    storage::{FeedStorage, LocalFile, StorageError},
};

// After storing an article, storage.get_all_articles() should return it.
fn assert_store_then_get_all<S: FeedStorage>(mut storage: S) {
    let title = "Real title";
    let content = "Real content";
    let id = 42;
    storage
        .store_article(
            "test",
            Article {
                title: title.into(),
                content: content.into(),
                id,
            },
        )
        .expect("store_article failed");

    let articles = storage
        .get_all_articles("test")
        .expect("get_all_articles failed");
    // Expect the stored article to be present.
    assert_eq!(articles.len(), 1, "expected exactly one stored article");
    let expected = Article {
        title: title.into(),
        content: content.into(),
        id,
    };
    assert_eq!(&articles[0], &expected);
}

// After storing multiple articles, get_latest_article() should return the most recent one.
fn assert_latest_is_most_recent<S: FeedStorage>(mut storage: S) {
    storage
        .store_article(
            "test",
            Article {
                title: "first".into(),
                content: "c1".into(),
                id: 1,
            },
        )
        .expect("store_article failed");
    storage
        .store_article(
            "test",
            Article {
                title: "second".into(),
                content: "c2".into(),
                id: 2,
            },
        )
        .expect("store_article failed");

    let expected = Article {
        title: "second".into(),
        content: "c2".into(),
        id: 2,
    };
    let latest = storage
        .get_latest_article("test")
        .expect("get_latest_article failed");
    assert_eq!(latest, expected);
}

// Test that get_latest_article returns FeedEmpty error for empty feeds.
fn assert_empty_feed_error<S: FeedStorage>(storage: S) {
    let result = storage.get_latest_article("empty_feed");
    assert!(matches!(result, Err(StorageError::FeedEmpty)));
}

// When storing an article in one feed, another feed should remain empty.
// This is extracted into a generic helper to match the other tests' style.
fn assert_isolated_between_feeds<S: FeedStorage>(mut storage: S) {
    storage
        .store_article(
            "feed_one",
            Article {
                title: "unique".into(),
                content: "body".into(),
                id: 100,
            },
        )
        .expect("store_article failed");

    // Read from a different feed; expect no articles.
    let articles = storage
        .get_all_articles("feed_two")
        .expect("get_all_articles failed");
    assert_eq!(
        articles.len(),
        0,
        "expected no articles in a different feed"
    );
}

// Test deduplication when storing an article with the same id.
fn assert_deduplication<S: FeedStorage>(mut storage: S) {
    let title1 = "First title";
    let content1 = "First content";
    let id = 42;
    storage
        .store_article(
            "test",
            Article {
                title: title1.into(),
                content: content1.into(),
                id,
            },
        )
        .expect("store_article failed");

    let title2 = "Second title";
    let content2 = "Second content";
    storage
        .store_article(
            "test",
            Article {
                title: title2.into(),
                content: content2.into(),
                id,
            },
        )
        .expect("store_article failed");

    let articles = storage
        .get_all_articles("test")
        .expect("get_all_articles failed");
    // Expect only one article with the same id.
    assert_eq!(articles.len(), 1, "expected exactly one stored article");
    let expected = Article {
        title: title2.into(),
        content: content2.into(),
        id,
    };
    assert_eq!(&articles[0], &expected);
}

#[test]
fn localfile_store_then_get_all() {
    let base = "./feeds-test/store_then_get_all";
    let _ = fs::remove_dir_all(base);
    let storage = LocalFile::new(base).expect("failed to create LocalFile");
    assert_store_then_get_all(storage);
    let _ = fs::remove_dir_all(base);
}

#[test]
fn localfile_latest_most_recent() {
    let base = "./feeds-test/latest_most_recent";
    let _ = fs::remove_dir_all(base);
    let storage = LocalFile::new(base).expect("failed to create LocalFile");
    assert_latest_is_most_recent(storage);
    let _ = fs::remove_dir_all(base);
}

#[test]
fn localfile_empty_feed_error() {
    let base = "./feeds-test/empty_feed_error";
    let _ = fs::remove_dir_all(base);
    let storage = LocalFile::new(base).expect("failed to create LocalFile");
    assert_empty_feed_error(storage);
    let _ = fs::remove_dir_all(base);
}

#[test]
fn localfile_isolated_between_feeds() {
    let base = "./feeds-test/isolated_between_feeds";
    let _ = fs::remove_dir_all(base);
    let storage = LocalFile::new(base).expect("failed to create LocalFile");
    assert_isolated_between_feeds(storage);
    let _ = fs::remove_dir_all(base);
}

#[test]
fn localfile_deduplication() {
    let base = "./feeds-test/deduplication";
    let _ = fs::remove_dir_all(base);
    let storage = LocalFile::new(base).expect("failed to create LocalFile");
    assert_deduplication(storage);
    let _ = fs::remove_dir_all(base);
}

