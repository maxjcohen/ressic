# Ressic

A minimal, self-hosted web service for publishing RSS feeds via HTTP endpoints.

## Overview
When a service produces articles meant for an RSS aggregator, it must remain
continuously available and serve a valid RSS feed on demand. Ressic decouples
that responsibility: your service simply POSTs articles to Ressic, and Ressic
handles persistence and feed serving.

Ressic allows clients to POST article data to HTTP endpoints, where each
endpoint corresponds to an RSS feed name. The service appends incoming articles
to the chosen feed and exposes the RSS feed for consumption by standard
aggregators.

## Features
- Simple REST API for posting articles to feeds
- Automatic RSS feed generation
- File-based storage
- Article deduplication by URL
- Minimal dependencies and portable design

## Quick Start
### Container (Docker/Podman)
These instructions for Podman also work with Docker.
```bash
$ podman build -t ressic .
$ podman run --rm -p 3000:3000 ressic
```

### Building
Prerequisites:
- Rust 1.85+ (edition 2024)
- Cargo

Clone the repository and build the project:
```bash
cargo build --release
```

Run the application with:
```bash
cargo run
```

This will start the application using the default feed storage location.

### Example usage
1. Add a new feed
```bash
curl -X POST http://localhost:3000/v1/feeds/myfeed \
     -H "Content-Type: application/json" \
     -d '{
           "title": "My Feed",
           "link": "http://example.com",
           "description": "This is my feed",
           "articles": [
                {
                     "id": "1",
                     "title": "First Article",
                     "url": "http://example.com/1",
                     "summary": "This is the first article",
                     "content": "Full content of the first article",
                     "pub_date": "2024-01-01T00:00:00Z"
                }
              ]
            }'
```

2. List currently stored feeds
```bash
curl -X GET http://localhost:3000/v1/feeds/
```

3. Get an RSS feed for the article we just pushed
```bash
curl -X GET http://localhost:3000/v1/rss/myfeed
```


## Data Model

### Storage
#### Local JSON files
Feed data is stored in the `feeds/` directory at the project root. Each feed is stored as a separate JSON file:

```
feeds/
├── default.json
├── tech_news.json
└── blog_posts.json
```

### Article
Articles contain the following fields:

- `title`: The article title (required)
- `content`: Full article content (required)
- `id`: Unique identifier within a feed (required)
- `url`: Original article URL (required, used for deduplication)
- `summary`: Brief article summary
- `pub_date`: Publication date in UTC (required)

### Feed Behavior
Articles are deduplicated by URL. If an article with the same URL is posted again, it replaces the previous one.

## Development
### Project Structure
```
src/
├── lib.rs          # Client interface and Axum router setup
├── main.rs         # Application entry point
├── models.rs       # Data models (Article, Feed)
├── api.rs          # HTTP route handlers
├── storage/        # Storage implementations
│   ├── mod.rs      # Storage trait definition
│   ├── local.rs    # Local file storage (JSON)
│   └── mock.rs     # Mock storage for testing
└── generator/      # Feed format generators
    ├── mod.rs      # Generator trait definition
    ├── rss20.rs    # RSS 2.0 feed generator
    ├── plain_text.rs # Plain text feed generator
    └── mock.rs     # Mock generator for testing

tests/
├── mod.rs                  # Test module root
├── api_tests.rs            # API endpoint tests
├── model_validation_tests.rs # Model validation tests
├── rss_generator_tests.rs  # RSS generator tests
├── storage_tests.rs        # Storage backend tests
├── test_client.rs          # Client tests
├── common/
│   └── mod.rs              # Test utilities
└── generators/
    ├── mod.rs              # Generator test module
    ├── rss20_tests.rs      # RSS 2.0 generator tests
    └── plain_text_tests.rs # Plain text generator tests
```
