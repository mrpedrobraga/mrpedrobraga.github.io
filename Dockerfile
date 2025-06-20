FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin pedrobraga-website

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/pedrobraga-website /usr/local/bin
COPY --from=planner /app/templates /app/templates
COPY --from=planner /app/public /app/public
COPY --from=planner /app/blog /app/blog
COPY --from=planner /app/pages /app/pages
ENTRYPOINT ["/usr/local/bin/pedrobraga-website"]
