FROM rust:1.92.0-trixie as frontend-builder
WORKDIR /usr/src/client

# Multi-stage Dockerfile for building a Leptos (Rust) application
RUN apt-get update && apt-get install -y --no-install-recommends \
	build-essential \
	pkg-config \
	libssl-dev \
	ca-certificates \
	&& rm -rf /var/lib/apt/lists/*

RUN cargo install --locked trunk
RUN rustup target add wasm32-unknown-unknown

COPY client .
COPY tmf-leptos .

RUN trunk build --release
