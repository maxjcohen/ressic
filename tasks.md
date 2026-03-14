# Tasks

## Project context (read this first after a context reset)

**What this project is:** Ressic — a minimal self-hosted Rust web service. Clients POST articles to
`/v1/feeds/:feed_name`; the service stores them and serves RSS 2.0 feeds at `/v1/rss/:feed_name`.

**Key source files:**
- `src/lib.rs` — `Client<S,G>` struct, `create_app()` router setup
- `src/api.rs` — Axum HTTP handlers (`post_feed`, `list_feeds`, `get_rss`), `ApiError`
- `src/models.rs` — `Article` and `Feed` structs with constructor-based validation, `ValidationError`
- `src/storage/mod.rs` — `FeedStorage` trait, `StorageError` enum
- `src/storage/local.rs` — `JsonLocalStorage` (JSONL-ish, actually writes full JSON per feed)
- `src/storage/mock.rs` — `MockStorage` (no-op, for tests)
- `src/generator/mod.rs` — `FeedGenerator` trait, `GeneratorError` enum
- `src/generator/rss20.rs` — `Rss20` generator (RSS 2.0 XML)
- `src/generator/plain_text.rs` — `PlainText` generator (testing helper)
- `src/generator/mock.rs` — `Mock` generator (no-op)
- `tests/` — integration and unit tests; `tests/common/mod.rs` has `with_localfile_storage` helper

**How to run tests:** `cargo test`
**How to build:** `cargo build --release`
**How to run:** `cargo run` (server on `0.0.0.0:3000`)

---

## Dependency map

Tasks must be completed in an order that respects these dependencies:

```
T1 (rename FeedEmpty)
  └─> T2 (Display/Error impls)
        └─> T3 (introduce ClientError)
              └─> T4 (fix generate_feed panic)
                    └─> T5 (handle mutex poisoning)

T6 (N+1 → add put_feed)
  └─> T7 (remove struct literal fallbacks)  [T8 may be dropped]
  └─> T8 (fix set_feed_metadata)            [superseded by T6, drop if T6 done]
  └─> T9 (stop bypassing Client in api.rs)
        └─> T10 (acquire lock after validation)
        └─> T11 (make Client fields private)

T12 (align feed name validation)   — independent
T13 (remove tower-http)            — independent
T14 (clean up API test dirs)       — independent
T15 (change &mut self → &self)     — independent
T16 (use field shorthand)          — independent
T17 (consolidate RSS tests)        — independent
```

## Development
Use this file as your only source of context: relevant information from a task MUST be added to this file in concise and precise wordings. 

---

## Tasks

### Naming / Semantics (do first — other tasks reference these names)

- [x] **T1 — Rename `FeedEmpty` to `FeedNotFound`**
  - Files: `src/storage/mod.rs`, `src/storage/local.rs`, `src/api.rs`, all test files that match on this variant
  - Problem: `StorageError::FeedEmpty` is returned when a feed *file does not exist*, not when the feed has zero articles. An empty feed (0 articles) is a valid state. The wrong name causes the API to respond `"Feed is empty"` when the real situation is "feed doesn't exist".
  - Fix: rename variant to `FeedNotFound` everywhere. In `api.rs` the HTTP mapping (`NotFound`) is already correct — only change the message string to `"Feed not found"`. Update any `matches!(result, Err(StorageError::FeedEmpty))` assertions in tests.
  - Done when: `cargo test` passes and `FeedEmpty` no longer appears in the codebase.

- [x] **T2 — Add `Display` and `Error` impls for `StorageError` and `GeneratorError`**
  - Files: `src/storage/mod.rs`, `src/generator/mod.rs`
  - Problem: only `ValidationError` has `Display` + `Error` impls. `StorageError` and `GeneratorError` can only be formatted with `{:?}`. This blocks building a proper `ClientError` wrapper (T3) and is inconsistent with Rust error conventions.
  - Fix: implement `std::fmt::Display` for both (human-readable messages, e.g. `"Feed not found"`, `"IO error: {}"`, `"Serialization error: {}"`). Then add `impl std::error::Error for StorageError` and `impl std::error::Error for GeneratorError`.
  - Done when: `cargo test` passes and both types can be used as `Box<dyn std::error::Error>`.

### Critical (panics)

- [x] **T3 — Introduce `ClientError` wrapping `StorageError` and `GeneratorError`**
  - File: `src/lib.rs`
  - Problem: `Client::generate_feed` returns `Result<String, GeneratorError>` but calls `self.storage.get_feed()` which returns `Result<Feed, StorageError>`. There is no way to propagate `StorageError` with the current signature, so `.unwrap()` is used.
  - Prerequisite: T2 (both error types must implement `std::error::Error`).
  - Fix: add a `ClientError` enum in `src/lib.rs` with `Storage(StorageError)` and `Generator(GeneratorError)` variants, plus `From` impls. Change `generate_feed` signature to `Result<String, ClientError>`. Add `impl From<ClientError> for ApiError` in `src/api.rs`. Add tests for the error path (e.g. calling `generate_feed` on a non-existent feed returns `ClientError::Storage(StorageError::FeedNotFound)`).
  - Done when: `cargo test` passes and `generate_feed` contains no `.unwrap()`.

- [ ] **T4 — Fix `.unwrap()` panic in `Client::generate_feed`**
  - File: `src/lib.rs`, method `Client::generate_feed`
  - Problem: `self.storage.get_feed(feed_name).unwrap()` panics if the feed does not exist.
  - Prerequisite: T3 (`ClientError` must exist).
  - Fix: replace `.unwrap()` with `?`. The `?` operator now propagates through `ClientError`.
  - Done when: `src/lib.rs` `generate_feed` contains no `.unwrap()` and `cargo test` passes.

- [ ] **T5 — Handle mutex poisoning in all handlers**
  - Files: `src/api.rs` — all three handlers (`post_feed`, `list_feeds`, `get_rss`)
  - Problem: every handler calls `client.lock().unwrap()`. If any handler panics while holding the lock, the mutex becomes poisoned; every subsequent request panics, effectively killing the server.
  - Prerequisite: T4 (the main panic source must be removed first, otherwise this is addressing a symptom).
  - Fix: replace `.unwrap()` with `.map_err(|_| ApiError::InternalError("Server state corrupted".to_string()))?` in each handler. This returns HTTP 500 instead of panicking.
  - Done when: `cargo test` passes and no handler calls `.lock().unwrap()`.

### Design / Architecture

- [ ] **T6 — Fix N+1 reads/writes on POST — add `put_feed` to `FeedStorage`**
  - Files: `src/storage/mod.rs` (trait), `src/storage/local.rs`, `src/storage/mock.rs`, `src/api.rs`
  - Problem: `post_feed` with N articles does N+1 reads and N+1 writes: one `set_feed_metadata` (read+write) plus one `store_article` (read+write) per article.
  - Fix: add `fn put_feed(&self, feed: &Feed) -> Result<(), StorageError>` to the `FeedStorage` trait. In `JsonLocalStorage`, implement it as a single atomic read-merge-write. In `MockStorage`, implement as no-op. Update `post_feed` in `api.rs` to validate all articles first, then call `put_feed` once. Add tests for `put_feed`.
  - Note: completing this task makes T7 (`set_feed_metadata` signature) and T8 (struct literal fallbacks) either fully superseded or significantly simpler — reassess T7 and T8 after T6 is done.
  - Done when: `cargo test` passes and `post_feed` calls storage exactly once.

- [ ] **T7 — Fix `set_feed_metadata` signature** *(reassess after T6 — may be dropped)*
  - Files: `src/storage/mod.rs`, `src/storage/local.rs`, `src/storage/mock.rs`, `src/api.rs`
  - Problem: `set_feed_metadata(&self, feed_name: &str, feed: &Feed)` takes a full `Feed` but only uses `title`, `link`, and `description`; articles in the argument are silently ignored. This is a leaky API.
  - Prerequisite: T6. If T6 replaces all usage of `set_feed_metadata` with `put_feed`, this trait method can simply be removed. If it still has uses, replace the `feed: &Feed` parameter with individual fields or a dedicated `FeedMetadata { title, link, description }` struct.
  - Done when: `set_feed_metadata` either no longer exists or its signature no longer accepts unused data.

- [ ] **T8 — Remove invalid `Feed` struct literal fallbacks in storage** *(reassess after T6)*
  - File: `src/storage/local.rs` — `store_article` and `set_feed_metadata`
  - Problem: both methods construct `Feed { link: "".to_string(), ... }` directly as a not-found fallback, bypassing `Feed::new()` validation and creating instances with empty required fields. The fallback in `store_article` is dangerous: the expected call order is `set_feed_metadata` first, so `store_article` silently creates a corrupt feed when called out of order.
  - Prerequisite: T6. If `put_feed` replaces both methods, these fallbacks disappear automatically. Otherwise, remove the fallback in `store_article` (return `StorageError::FeedNotFound` instead); the fallback in `set_feed_metadata` can use `Feed::new()` with defaults or also be removed.
  - Done when: no direct `Feed { ... }` struct literals remain in `local.rs`.

- [ ] **T9 — Stop bypassing `Client` in `api.rs`**
  - Files: `src/api.rs`, `src/lib.rs`
  - Problem: all three handlers access `client.storage.*` and `client.generator.*` directly. This makes `Client` a passive container rather than the intended abstraction layer. The methods `Client::list_feeds` and `Client::generate_feed` already exist but are not used by the handlers.
  - Prerequisite: T6 (handlers need a `put_feed`-based method on `Client`), T4 (handlers need to call `generate_feed` without it panicking).
  - Fix: add `pub fn post_feed(&self, feed_name: &str, feed: Feed) -> Result<(), ClientError>` (or equivalent) to `Client`. Rewrite handlers to call only `Client` methods. Validation can remain in the handler layer (before the lock) or move into the client method.
  - Done when: handlers in `api.rs` contain no `.storage.` or `.generator.` field accesses.

- [ ] **T10 — Acquire lock after validation in `post_feed`**
  - File: `src/api.rs`, `post_feed`
  - Problem: the `Mutex<Client>` guard is acquired before article/feed validation and held through all I/O, serializing all concurrent requests behind file operations.
  - Prerequisite: T9 (once bypassing is removed, the lock boundary becomes a clean choice).
  - Fix: perform all `Article::new` and `Feed::new` validation before acquiring the lock. Acquire the lock only for the call(s) to `Client` storage methods.
  - Done when: in `post_feed`, `client.lock()` is called after all validation, not before.

- [ ] **T11 — Make `Client` fields private**
  - File: `src/lib.rs` — `pub storage`, `pub generator`
  - Problem: both fields are `pub` solely because `api.rs` bypasses `Client` methods. Public fields prevent enforcing invariants.
  - Prerequisite: T9 (handlers must stop accessing fields directly).
  - Fix: remove `pub` from both field declarations.
  - Done when: `storage` and `generator` are private and `cargo test` passes.

- [ ] **T12 — Align feed name validation between model and storage layers**
  - Files: `src/models.rs` `Feed::new`, `src/storage/local.rs` `JsonLocalStorage::validate_feed_name`
  - Problem: `Feed::new` allows only alphanumeric, `-`, `_`. `JsonLocalStorage::validate_feed_name` allows any printable non-path character (so `test@feed` passes storage but would have been rejected at the model layer). The storage guard should be at least as strict as the model guard.
  - Fix: update `JsonLocalStorage::validate_feed_name` to apply the same whitelist: `c.is_alphanumeric() || c == '-' || c == '_'`.
  - Done when: feed names like `test@feed` are rejected by `validate_feed_name` and `cargo test` passes.

### Minor / Style

- [ ] **T13 — Remove unused `tower-http` dependency**
  - File: `Cargo.toml`
  - Problem: `tower-http = { version = "0.5", features = ["trace"] }` is listed in `[dependencies]` but never imported in any source file.
  - Fix: delete that line from `Cargo.toml`. Run `cargo build` to confirm.
  - Done when: `tower-http` does not appear in `Cargo.toml`.

- [ ] **T14 — Clean up API test temp directories**
  - File: `tests/api_tests.rs`, `spawn_test_server`
  - Problem: each test creates a `feeds-test/api_test_{timestamp}` directory that is never deleted. The `common::with_localfile_storage` helper used by storage tests does cleanup, but the API tests skip it.
  - Fix: return the temp directory path from `spawn_test_server` alongside the server URL. At the end of each `#[tokio::test]`, call `std::fs::remove_dir_all` on it. Alternatively, use a drop guard struct.
  - Done when: no `feeds-test/api_test_*` directories remain after `cargo test`.

- [ ] **T15 — Change `Client::store_article` signature from `&mut self` to `&self`**
  - File: `src/lib.rs`
  - Problem: `pub fn store_article(&mut self, ...)` uses `&mut self` but `FeedStorage::store_article` takes `&self` (interior mutability). The `&mut` is unnecessary and inconsistent.
  - Fix: change to `&self`.
  - Done when: `cargo test` passes and `store_article` on `Client` takes `&self`.

- [ ] **T16 — Use field shorthand in `Client::new`**
  - File: `src/lib.rs`, `Client::new`
  - Problem: `Client { storage: storage, generator: generator }` does not use Rust's field shorthand.
  - Fix: change to `Client { storage, generator }`.
  - Done when: the initialiser uses shorthand syntax.

- [ ] **T17 — Consolidate duplicate RSS generator tests**
  - Files: `tests/rss_generator_tests.rs`, `tests/generators/rss20_tests.rs`, `tests/generators/mod.rs`
  - Problem: `tests/rss_generator_tests.rs` already has extensive RSS 2.0 tests. `tests/generators/rss20_tests.rs` only contains a trivial `is_ok()` check that is a strict subset. Having two locations causes confusion about where new tests should go.
  - Fix: delete `tests/generators/rss20_tests.rs`. Remove the `mod rss20_tests;` line from `tests/generators/mod.rs`. Verify `tests/generators/plain_text_tests.rs` still compiles.
  - Done when: `tests/generators/rss20_tests.rs` no longer exists and `cargo test` passes.
