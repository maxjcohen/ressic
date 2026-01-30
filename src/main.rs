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

    // Bind server
    let address = "0.0.0.0";
    let port = 3000;
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", address, port))
        .await
        .expect("Failed to bind to address");

    println!("Ressic server listening on http://{}:{}", address, port);

    // Start the server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Server failed to start");

    println!("Server shut down gracefully");
}

async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            println!("Received Ctrl+C signal");
        },
        _ = terminate => {
            println!("Received SIGTERM signal");
        },
    }
}
