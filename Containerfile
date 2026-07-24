# syntax=docker/dockerfile:1.20-labs
FROM docker.io/rust:1.97-bookworm AS builder
WORKDIR /app
COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY .sqlx .sqlx
COPY migrations migrations
COPY build.rs build.rs

ARG APPNAME=starboard
RUN --mount=type=cache,id=starboard-cargo-registry,target=/usr/local/cargo/registry,sharing=shared \
    --mount=type=cache,id=starboard-cargo-git,target=/usr/local/cargo/git,sharing=shared \
    --mount=type=cache,id=starboard-target,target=/app/target,sharing=locked \
    cargo build --release --locked --package ${APPNAME} && \
    cp /app/target/release/${APPNAME} /usr/local/bin/app

FROM gcr.io/distroless/cc-debian13:nonroot AS runtime
USER nonroot:nonroot
COPY --from=builder /usr/local/bin/app /
ENTRYPOINT ["/app"]
