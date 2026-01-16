# Tasks

- Implement FeedGenerator trait and a minimal RSS 2.0 generator (MVP).
  - Implement `Client::generate_feed` to return RSS XML for a given feed.
  - Add tests that generated RSS validates as RSS 2.0 (basic structure).

- API server & integration using Rocket
  - Implement a minimal HTTP server exposing:
    - POST /v1/:feed_name to accept articles (JSON) and append to storage.
    - GET /rss/:feed_name.xml to return generated RSS feed.
  - Ensure POSTs are logged to console with feed and title for both success and failures.
  - Add integration tests that start the server, post an article, then fetch the RSS and assert it contains the article.
  - Add integration tests for end-to-end behaviour (store -> generate -> fetch).