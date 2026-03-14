use ressic::Client;
use ressic::ClientError;
use ressic::generator::PlainText;
use ressic::storage::{JsonLocalStorage, MockStorage, StorageError};

#[test]
fn load_client() {
    let storage = MockStorage::new();
    let generator = PlainText::new();
    let _client = Client::new(storage, generator);
}

#[test]
fn generate_feed_nonexistent_returns_feed_not_found() {
    let base = "./feeds-test/test_client_generate_nonexistent";
    let _ = std::fs::remove_dir_all(base);
    let storage = JsonLocalStorage::new(base).expect("failed to create storage");
    let generator = PlainText::new();
    let client = Client::new(storage, generator);

    let result = client.generate_feed("nonexistent_feed");
    let _ = std::fs::remove_dir_all(base);

    assert!(
        matches!(
            result,
            Err(ClientError::Storage(StorageError::FeedNotFound))
        ),
        "expected ClientError::Storage(FeedNotFound), got {:?}",
        result
    );
}
