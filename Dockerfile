FROM ekidd/rust-musl-builder:nightly-2020-01-26 AS rust-build

# Add alpine target
RUN rustup target add x86_64-unknown-linux-musl

# Re-own home
# https://github.com/emk/rust-musl-builder#making-static-releases-with-travis-ci-and-github
RUN sudo chown -R rust:rust /home

# Install and cache dependencies layers
# Rather than copying everything every time, re-use cached dependency layers
# to install/build deps only when Cargo.* files change.
RUN USER=root cargo new /home/airtravel --bin

WORKDIR /home/airtravel
COPY Cargo.toml Cargo.lock ./
RUN cargo build --bins --release --target x86_64-unknown-linux-musl

# Load real sources
COPY src ./src

# Rebuild with real sources
RUN rm ./target/x86_64-unknown-linux-musl/release/deps/hkd_encyclopedia_front_service*
RUN cargo build --bins --release --target x86_64-unknown-linux-musl


FROM alpine:3.9.4

WORKDIR /home

# Copy Rust artifacts
COPY --from=rust-build /home/airtravel/target/x86_64-unknown-linux-musl/release/airtravel .

EXPOSE 8080
CMD ["./airtravel"]
