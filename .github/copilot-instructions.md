Start by reading the README for information about the project, its goals and implementation design. Then, follow the instructions below to implement the project according to the specified requirements and development guidelines.

### Functional Requirements
- Accept POST requests at `/v1/:feed_name` containing article content
- Add each new post to the corresponding `feed_name` RSS feed
- Make each feed available at `/rss/:feed_name.xml` for traditional aggregators

## Development
### Test, Develop and Run
- Follow this development workflow for implementing new features:
  1. Select a minimal feature to implement
  2. Write extensive tests for the feature
  3. Check that these tests fail with `cargo test`
  4. Write a minimal implementation for the feature that makes the tests pass
  5. Refactor the implementation if necessary, ensuring tests still pass
  6. Update the documentation if needed (comments, README, tasks.md, copilot-instructions.md, etc.)
  7. Commit the changes, adding only relevant files to the commit (avoid `git add -A`)
- When running `cargo` or `git` commands, do not use `cd` before, execute the
  command in the current working directory.

## Additional Recommendations
- Do not introduce user management/auth unless essential
- Document configuration settings in README
- Use clear, single-responsibility functions
- Trust these instructions; search or explore the codebase only if unclear
- Do not use emojis under any circumstance
- Before anything, always ask for clarification if unsure about any aspect of the project. Never make assumptions.
