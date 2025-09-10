// Storage tests for FeedStorage trait. Test storing articles and retrieving them.

use ressic::{
    models::Article,
    storage::{FeedStorage, MockStorage},
};

// After storing an article, storage.get_all_articles() should return it.
fn assert_store_then_get_all<S: FeedStorage>(mut storage: S) {
    let title = "Real title";
    let content = "Real content";
    let id = 42;

    storage.store_article(
        "test",
        Article {
            title: title.into(),
            content: content.into(),
            id,
        },
    );

    let articles = storage.get_all_articles("test");
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
    storage.store_article(
        "test",
        Article {
            title: "first".into(),
            content: "c1".into(),
            id: 1,
        },
    );
    storage.store_article(
        "test",
        Article {
            title: "second".into(),
            content: "c2".into(),
            id: 2,
        },
    );

    let expected = Article {
        title: "second".into(),
        content: "c2".into(),
        id: 2,
    };
    let latest = storage.get_latest_article("test");
    assert_eq!(latest, expected);
}

// When storing an article in one feed, another feed should remain empty.
// This is extracted into a generic helper to match the other tests' style.
fn assert_isolated_between_feeds<S: FeedStorage>(mut storage: S) {
    storage.store_article(
        "feed_one",
        Article {
            title: "unique".into(),
            content: "body".into(),
            id: 100,
        },
    );

    // Read from a different feed; expect no articles.
    let articles = storage.get_all_articles("feed_two");
    assert_eq!(
        articles.len(),
        0,
        "expected no articles in a different feed"
    );
}

// Run the above expectations against the current MockStorage.
#[test]
fn mock_storage_store_then_get_all() {
    let storage = MockStorage {};
    assert_store_then_get_all(storage);
}

#[test]
fn mock_storage_latest_most_recent() {
    let storage = MockStorage {};
    assert_latest_is_most_recent(storage);
}

#[test]
fn mock_storage_isolated_between_feeds() {
    let storage = MockStorage {};
    assert_isolated_between_feeds(storage);
}
