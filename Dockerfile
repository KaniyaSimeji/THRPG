FROM rust:latest AS builder

# builder
WORKDIR /thrpg
COPY Cargo.toml Cargo.toml
COPY libs/ libs/
COPY bins/ bins/
RUN cargo build --release

# bin
FROM debian:bullseye
WORKDIR /opt/thrpg
COPY chara/ chara/
COPY i18n/ i18n/
COPY THRPG.toml THRPG.toml
COPY --from=builder /thrpg/target/release/bot bot
