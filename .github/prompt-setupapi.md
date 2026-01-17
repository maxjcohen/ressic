Setup a Web API for Ressic

API allows adding articles to be stored, and consuming resulting RSS feed.

The API uses the ressic Client. It NEVER uses the Storage and Generator interfaces directly. It is developped using Axum. Server execution in src/main.rs

## Paths
### Adding content
- POST /v1/feed/<feedname>: Add an article to the feed <feedname>. Input data should be of type Feed (metadata + list of articles), even for a single article. Even if the feed already exists (metadata are replaced).
- GET /v1/feed/: List stored feeds

### Consuming feed
- GET /v1/rss/<feedname>: Return a valid RSS feed.