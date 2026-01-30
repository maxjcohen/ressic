FROM docker.io/rust:1.92-trixie as BUILDER

WORKDIR /build

# 1. Copy only Cargo manifests
COPY Cargo.toml Cargo.lock ./

# 2. Create dummy src and build once to compile deps
RUN mkdir src \
 && echo 'fn main() { println!("dummy"); }' > src/main.rs \
 && cargo build --release

# 3. Now copy real source; deps layer remains cached
COPY src src

RUN cargo build --release

FROM docker.io/debian:bookworm-slim AS RUNTIME
WORKDIR /app

COPY --from=BUILDER /build/target/release/ressic /usr/local/bin/ressic

CMD ["ressic"]