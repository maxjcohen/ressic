use chrono::Utc;
use ressic::{
    Client,
    generator::Rss20,
    models::{Article, Feed},
    storage::JsonLocalStorage,
};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

/// Drop guard that removes its directory when it goes out of scope.
struct TempDir {
    path: String,
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

/// Helper function to create a test server instance.
/// Returns the server URL and a `TempDir` guard; the directory is deleted when the guard drops.
async fn spawn_test_server() -> (String, TempDir) {
    // Use a unique test directory for each test
    let test_dir = format!(
        "feeds-test/api_test_{}",
        Utc::now().timestamp_nanos_opt().unwrap()
    );
    let storage = JsonLocalStorage::new(&test_dir).unwrap();
    let generator = Rss20;
    let client = Client::new(storage, generator);
    let shared_client = Arc::new(Mutex::new(client));

    // Create the router using the app function from main
    let app = ressic::create_app(shared_client);

    // Bind to a random available port
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    // Spawn the server in a background task
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // Give the server a moment to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    (format!("http://{}", addr), TempDir { path: test_dir })
}

#[tokio::test]
async fn test_post_and_get_rss() {
    let (server_url, _temp_dir) = spawn_test_server().await;

    // Create a feed with an article
    let article = Article::new(
        "Test Article".to_string(),
        "This is test content".to_string(),
        "test-1".to_string(),
        "http://example.com/test-1".to_string(),
        "Test summary".to_string(),
        Utc::now(),
    )
    .unwrap();

    let feed = Feed::new(
        "testfeed".to_string(),
        "Test Feed".to_string(),
        "http://example.com".to_string(),
        "A test feed".to_string(),
        vec![article.clone()],
    )
    .unwrap();

    // POST the feed
    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/v1/feeds/testfeed", server_url))
        .json(&feed)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    // GET the RSS feed
    let response = client
        .get(&format!("{}/v1/rss/testfeed", server_url))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let rss_content = response.text().await.unwrap();

    // Verify RSS contains the article
    assert!(rss_content.contains("Test Article"));
    assert!(rss_content.contains("Test summary")); // RSS uses summary, not full content
    assert!(rss_content.contains("http://example.com/test-1"));
}

#[tokio::test]
async fn test_list_feeds() {
    let (server_url, _temp_dir) = spawn_test_server().await;

    // Create two feeds
    let feed1 = Feed::new(
        "feed1".to_string(),
        "Feed One".to_string(),
        "http://example.com".to_string(),
        "First feed".to_string(),
        vec![],
    )
    .unwrap();

    let feed2 = Feed::new(
        "feed2".to_string(),
        "Feed Two".to_string(),
        "http://example.com".to_string(),
        "Second feed".to_string(),
        vec![],
    )
    .unwrap();

    let client = reqwest::Client::new();

    // POST both feeds
    client
        .post(&format!("{}/v1/feeds/feed1", server_url))
        .json(&feed1)
        .send()
        .await
        .unwrap();

    client
        .post(&format!("{}/v1/feeds/feed2", server_url))
        .json(&feed2)
        .send()
        .await
        .unwrap();

    // GET the list of feeds
    let response = client
        .get(&format!("{}/v1/feeds/", server_url))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let feeds: Vec<String> = response.json().await.unwrap();

    assert_eq!(feeds.len(), 2);
    assert!(feeds.contains(&"feed1".to_string()));
    assert!(feeds.contains(&"feed2".to_string()));
}

#[tokio::test]
async fn test_invalid_feed_name() {
    let (server_url, _temp_dir) = spawn_test_server().await;

    // Note: Using struct literal here to bypass validation - we're testing API validation
    let feed = Feed {
        name: "invalid@feed".to_string(),
        title: "Invalid Feed".to_string(),
        link: "http://example.com".to_string(),
        description: "Should fail".to_string(),
        articles: vec![],
    };

    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/v1/feeds/invalid@feed", server_url))
        .json(&feed)
        .send()
        .await
        .unwrap();

    // Should return 400 Bad Request for invalid feed name
    assert_eq!(response.status(), 400);
}

#[tokio::test]
async fn test_get_nonexistent_feed() {
    let (server_url, _temp_dir) = spawn_test_server().await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/v1/rss/nonexistent", server_url))
        .send()
        .await
        .unwrap();

    // Should return 404 Not Found
    assert_eq!(response.status(), 404);
}
