## Project Purpose

Ressic is a minimal, self-hosted web service allowing clients to POST article data to an HTTP endpoint, where each endpoint corresponds to an RSS feed name. The service appends incoming articles to the chosen feed and exposes the RSS feed for consumption by standard aggregators.

## Roadmap
- First MVP should include
    - `FeedStorage` and `FeedGenerator` interfaces
    - Local JSONL file storage for feed
    - Feed generation in RSS v2 format
- V1 should include
    - Database storage via SQLite
    - Feed generation in Atom format

### Functional Requirements
- Accept POST requests at `/v1/:feed_name` containing article content
- Add each new post to the corresponding `feed_name` RSS feed
- Make each feed available at `/rss/:feed_name.xml` for traditional aggregators
- Feed updates must be reflected immediately or within seconds of new POST
- Minimal external dependencies; prioritize portability and readability

## Development
### Tech Stack
- Programming Language: Rust
- Data Storage: Flat files (JSONL per feed).

### Coding Patterns
- Develop interfaces for modularity. Use a `FeedStorage` interface which will be
  first implemented for local files, then databases such as SQLite. Use a
  `FeedGenerator` interface which will be first implemented for XML, then Atom
  format.
- Develop a client with a clear interface, then an API server on top of that client
- Routes must be defined using RESTful conventions
- Validate all input (title, content, id)
- Use clear and concise comment and in-code documentation
- Commit after each modification
- Use Conventional Commits: <type>[scope]: <description> \n [optional body]
  Example: feat(storage): store article title`

### Project Structure
- `src/`
  - `lib.rs`: Client interface for feed operations
  - `models.rs`: Data models for Article and Feed
  - `storage/`: Module containing `FeedStorage` interface and implementations
    - `mod.rs`: Storage interface definition
    - `local.rs`: Local file storage implementation using JSONL files
    - `mock.rs`: Mock storage for testing purposes
- `tests/`: Integration and unit tests

#### Additional Directories
- `feeds/`: Directory to store feed files (JSONL format)
- `feeds_test/`: Directory to store test feed files (JSONL format)

### Test, Develop and Run
- Follow this development workflow for implementing new features:
  1. Select a minimal feature to implement
  2. Write extensive tests for the feature
  3. Check that these tests fail with `cargo test`
  4. Write a minimal implementation for the feature that makes the tests pass
  5. Refactor the implementation if necessary, ensuring tests still pass
  6. Update the documentation if needed (comments, README, copilot-instructions.md, etc.)
  7. Commit the changes
- When running `cargo` or `git` commands, do not use `cd` before, execute the
  command in the current working directory.


## Key Project Conventions
- All article POSTs must contain at least: feed name, title, content, id
- Feeds must use valid RSS 2.0 structure; update the feed immediately after adding an article
- Logging: Console-only, log all successful and failed POSTs with feed and title


## Additional Recommendations
- Do not introduce user management/auth unless essential
- Document configuration settings in README
- Use clear, single-responsibility functions
- Trust these instructions; search or explore the codebase only if unclear
- Do not use emojis under any circumstance
- Before anything, always ask for clarification if unsure about any aspect of the project. Never make assumptions.
