# Ressic

A minimal, self-hosted web service for managing RSS feeds via HTTP endpoints.

## Overview
Ressic allows clients to POST article data to HTTP endpoints, where each endpoint corresponds to an RSS feed name. The service appends incoming articles to the chosen feed and exposes the RSS feed for consumption by standard aggregators.

## Features
- Simple HTTP API for posting articles to feeds
- Automatic RSS feed generation
- File-based storage (JSONL format)
- Article deduplication by URL
- Feed isolation - each feed is independent
- Minimal dependencies and portable design

## Quick Start
### Prerequisites
- Rust 1.80+ (edition 2024)
- Cargo

### Installation
Clone the repository and build the project:

```bash
cargo build --release
```

### Running the Application
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
           "name": "myfeed",
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
#### Local Json files
Feed data is stored in the `feeds/` directory at the project root. Each feed is stored as a separate Json file:

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
- `summary`: Brief article summary (required)
- `pub_date`: Publication date in UTC (required)

### Feed Behavior
- **Deduplication**: Articles are deduplicated by URL. If an article with the same URL is posted again, it replaces the previous one.
- **Ordering**: When retrieving articles, the most recent article (by `pub_date`) is prioritized.
- **Isolation**: Each feed is independent. Articles posted to one feed do not appear in others.

## Development
### Project Structure
```
src/
├── lib.rs          # Client interface for feed operations
├── main.rs         # Application entry point
├── models.rs       # Data models (Article, Feed)
└── storage/        # Storage implementations
    ├── mod.rs      # Storage trait definition
    ├── local.rs    # Local file storage (JSONL)
    └── mock.rs     # Mock storage for testing

tests/
├── storage_tests.rs    # Storage backend tests
├── test_client.rs      # Client tests
└── common/
    └── mod.rs          # Test utilities
```