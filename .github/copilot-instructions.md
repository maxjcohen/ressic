## Project Purpose

Design a minimal, self-hosted web service allowing clients to POST article data to an HTTP endpoint, where each endpoint corresponds to an RSS feed name. The service appends incoming articles to the chosen feed and exposes the RSS feed for consumption by standard aggregators.

## Roadmap
- First MVP should include
    - `FeedStorage` and `FeedGenerator` interfaces
    - Local XML file storage for feed
    - Feed generation in RSS v2 format
- V1 should include
    - Database storage via SQLite
    - Feed generation in Atom format


## Tech Stack

- Programming Language: Rust
- Framework: Diesel, Rocket in future developments
- RSS Generation: Use built-in XML libraries or lightweight packages
- Data Storage: Flat files (XML per feed) ; lightweight database (SQLite) for future developments.


## Functional Requirements

- Accept POST requests at `/v1/:feed_name` containing article content (title, body, link, etc.)
- Add each new post to the corresponding `feed_name` RSS feed
- Make each feed available at `/rss/:feed_name.xml` for traditional aggregators
- Feed updates must be reflected immediately or within seconds of new POST
- Minimal external dependencies; prioritize portability and readability

## Coding Patterns

- Develop interfaces for modularity. Use a `FeedStorage` interface which will be
  first implemented for local files, then databases such as SQLite. Use a
  `FeedGenerator` interface which will be first implemented for XML, then Atom
  format.
- Develop a client with a clear interface, then an API server on top of that client
- Routes must be defined using RESTful conventions
- Error handling: Return HTTP 400 for malformed requests, HTTP 404 for unknown feeds, etc.
- Validate all input (title, link, publication date)
- Use clear and concise comment and in-code documentation
- Commit after each modification
- Use Conventional Commits: <type>[scope]: <description> \n [optional body]
  Example: feat(storage): store article title

## Test, Develop and Run

- Develop using test driven approach
- Always add or edit small features at a time, wait for test validation before moving to the next one.


## Key Project Conventions

- All article POSTs must contain at least: feed name, title, URL, summary, publication date
- Feeds must use valid RSS 2.0 structure; update the feed immediately after adding an article
- Logging: Console-only, log all successful and failed POSTs with feed and title


## Additional Recommendations

- Do not introduce user management/auth unless essential
- Document configuration settings in README
- Use clear, single-responsibility functions
- Trust these instructions; search or explore the codebase only if unclear
- Do not use emojis under any circumstance
