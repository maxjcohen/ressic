# Tasks

## Stabilize
  - Add unit tests that assert proper error is returned when call get_latest_article for empty feeds.

- Data model improvements
  - Add `url: String` to `Article` and migrate tests/examples.
  - Update deduplication to use `url` (and keep id optional/auxiliary).

- Data model improvements
  - Add `pub_date: datetime` to `Article` using the `chrono` crate.
  - Update `get_latests_article` to retrieve last based on `pub_date`.


- Data model improvements
  - Add `summary: String` to `Article`.


- Validation and input handling
  - Add input validation for required fields (title, url, pub_date) and clear error messages.

- Tests, CI, and docs
  - Add unit tests for storage: read/write, dedupe-by-url, latest-by-pub_date.
  - Add integration tests for end-to-end behaviour (store -> generate -> fetch).
  - Add `README.md` with quick start and how to run tests; document feed file location (`feeds/`).
  - Ensure `cargo build` and `cargo test` pass in CI.

## MVP
- Implement FeedGenerator trait and a minimal RSS 2.0 generator (MVP).
  - Implement `Client::generate_feed` to return RSS XML for a given feed.
  - Add tests that generated RSS validates as RSS 2.0 (basic structure).

- API server & integration using Rocket
  - Implement a minimal HTTP server exposing:
    - POST /v1/:feed_name to accept articles (JSON) and append to storage.
    - GET /rss/:feed_name.xml to return generated RSS feed.
  - Ensure POSTs are logged to console with feed and title for both success and failures.
  - Add integration tests that start the server, post an article, then fetch the RSS and assert it contains the article.

## V1
- Add SQL backend using Diesel