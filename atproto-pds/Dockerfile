# syntax=docker/dockerfile:experimental
FROM rust:1-alpine3.16 as builder
RUN apk add --no-cache cargo
ENV HOME=/root
WORKDIR /app/
COPY . /app/
ARG GIT_HASH
RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/root/app/target GIT_HASH=${GIT_HASH} cargo build --bin atproto-pds --release --target=x86_64-unknown-linux-musl --color never
RUN ls /app/target/x86_64-unknown-linux-musl/release/

FROM alpine:3.16
LABEL org.opencontainers.image.source=https://github.com/ngerakines/atproto-rs
LABEL org.opencontainers.image.description="An atproto personal data server implementation in rust."
LABEL org.opencontainers.image.authors="Nick Gerakines <nick.gerakines@gmail.com>"
LABEL org.opencontainers.image.licenses="MIT"
RUN apk add --no-cache curl
ENV RUST_LOG="warning"
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/atproto-pds /usr/local/bin/atproto-pds
CMD ["sh", "-c", "/usr/local/bin/atproto-pds"]
