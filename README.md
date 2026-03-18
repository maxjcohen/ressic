# Ressic

A minimal, self-hosted web service for publishing RSS feeds via HTTP endpoints.

## Overview

When a service produces articles meant for an RSS aggregator, it must remain
continuously available and serve a valid RSS feed on demand. Ressic decouples
that responsibility: your service simply POSTs articles to Ressic, and Ressic
handles persistence and feed serving.

## Design

1. **Simple API**: articles are submitted via HTTP POST to a named endpoint;
   feeds are retrieved via HTTP GET in RSS format.
2. **Minimal dependencies and portability**: few external dependencies to limit
   code maintenance workload.
3. **Intentionally limited feature set**: no authentication or user management is planned;
   article deduplication is the only article-level logic.

## Quick Start

### Container (Docker/Podman)

These instructions for Podman also work with Docker. Pull the image from GitHub Container Registry and run it:
```bash
podman run --rm -p 3000:3000 -v ./feeds:/app/feeds ghcr.io/maxjcohen/ressic:latest
```

Or build the image locally and run it:
```bash
podman build -t ressic .
podman run --rm -p 3000:3000 -v ./feeds:/app/feeds ressic
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

This starts the application on port `:3000` using the default feed storage location `./feeds`.

### Configuration
Most deployment options, such as port number and feed storage location, will be configurable soon^TM. 

## Example usage
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

We abstract storage backends through the `FeedStorage` trait.  Future backends
(e.g. SQLite) can be added by implementing this trait. The active backend is
selected at startup. Currently implemented backends:

#### Local JSON files (default)

Feed data is stored in the `feeds/` directory at the project root. Each feed is
stored as a separate JSON file:

```
feeds/
├── default.json
├── tech_news.json
└── blog_posts.json
```

### Feed generation
We abstract feed generation in a similar way through the `FeedGenerator` trait.
The two currently implemented generators are RSS 2.0 and plain text, mainly for
debugging purposes. Plan is to implement Atom later.

### Article

Articles contain the following fields:

- `title`: The article title (required)
- `content`: Full article content (required)
- `id`: Unique identifier within a feed (required)
- `url`: Original article URL (required, used for deduplication)
- `pub_date`: Publication date in UTC (ISO 8601, required)
- `summary`: Brief article summary (optional)

Articles are deduplicated by URL. If an article with the same URL is posted
again, it replaces the previous one.

## API
The documentation for the API is served on `/docs` based on
[redocs](https://redocly.github.io/redoc/)

## Development

### Project Structure

```
src/
├── main.rs             # Application entry point
├── lib.rs              # Client interface and Axum router setup
├── models.rs           # Data models (Article, Feed)
├── api.rs              # HTTP route handlers
├── storage/            # FeedStorage trait and implementations
│   ├── mod.rs          # Storage trait definition
│   ├── local.rs        # Local file storage (JSON)
│   └── mock.rs         # Mock storage for testing
└── generator/          # FeedGenerator trait and implementations
    ├── mod.rs          # Generator trait definition
    ├── rss20.rs        # RSS 2.0 feed generator
    ├── plain_text.rs   # Plain text feed generator
    └── mock.rs         # Mock generator for testing

tests/
├── mod.rs                      # Test module root
├── api_tests.rs                # API endpoint tests
├── model_validation_tests.rs   # Model validation tests
├── storage_tests.rs            # Storage backend tests
├── test_client.rs              # Client tests
├── common/
│   └── mod.rs                  # Test utilities
└── generators/
    ├── mod.rs                  # Generator test module
    ├── rss20_tests.rs          # RSS 2.0 generator tests
    └── plain_text_tests.rs     # Plain text generator tests
```

Additional directories:
- `feeds/`: feed data files (JSON format)
- `feeds-test/`: feed files used during testing

### Running Tests
Simply through:

```bash
cargo test
```

### Coding Patterns

- Interfaces are modular: `FeedStorage` and `FeedGenerator`.
- The client exposes a clear interface; a transparent HTTP API server is built
  on top of it. Routes follow RESTful conventions.
- All input is validated (title, content, id).
- Code is documented with clear, concise comments.
- Each change is committed using [Conventional
  Commits](https://www.conventionalcommits.org/en/v1.0.0/).

## License
See [LICENSE](LICENSE) for details.