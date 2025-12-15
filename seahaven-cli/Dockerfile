## Rust builder
FROM rust:1.92.0-slim-bookworm AS rust-builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        build-essential \
        clang \
        cmake \
        pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /src

COPY ./ ./

RUN cargo build --package seahaven-cli --bin truman --release


## Final image
FROM debian:bookworm-slim

COPY --from=rust-builder /src/target/release/truman /usr/local/bin/truman

ENTRYPOINT [ "truman" ]
