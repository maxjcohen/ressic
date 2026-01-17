use ressic::{Client, create_app, generator::Rss20, storage::JsonLocalStorage};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    // Initialize storage and generator
    let storage = JsonLocalStorage::new("feeds").expect("Failed to initialize storage");
    let generator = Rss20;

    // Create the client
    let client = Client::new(storage, generator);
    let shared_client = Arc::new(Mutex::new(client));

    // Create the Axum router
    let app = create_app(shared_client);

    // Bind to localhost:3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to address");

    println!("Ressic server listening on http://127.0.0.1:3000");

    // Start the server
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
