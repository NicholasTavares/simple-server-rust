FROM rust:1.67-slim-bullseye

WORKDIR /app
COPY Cargo.toml Cargo.toml
COPY ./src ./src
