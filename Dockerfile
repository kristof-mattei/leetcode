FROM rust:1.68.0@sha256:e10f1e351cb58f164d0e89a3c2b623ae8e2757bee0311110052420709307d6ea as builder

ENV TARGET=x86_64-unknown-linux-musl
RUN rustup target add ${TARGET}

RUN rm -f /etc/apt/apt.conf.d/docker-clean; echo 'Binary::apt::APT::Keep-Downloaded-Packages "true";' > /etc/apt/apt.conf.d/keep-cache

# borrowed (Ba Dum Tss!) from
# https://github.com/pablodeymo/rust-musl-builder/blob/7a7ea3e909b1ef00c177d9eeac32d8c9d7d6a08c/Dockerfile#L48-L49
RUN --mount=type=cache,target=/var/cache/apt --mount=type=cache,target=/var/lib/apt \
    apt-get update && \
    apt-get --no-install-recommends install -y \
    build-essential \
    musl-dev \
    musl-tools

# The following block
# creates an empty app, and we copy in Cargo.toml and Cargo.lock as they represent our dependencies
# This allows us to copy in the source in a different layer which in turn allows us to leverage Docker's layer caching
# That means that if our dependencies don't change rebuilding is much faster
WORKDIR /build
RUN cargo new rust-end-to-end-application
WORKDIR /build/rust-end-to-end-application
COPY Cargo.toml Cargo.lock ./
RUN --mount=type=cache,id=cargo-only,target=/build/rust-end-to-end-application/target \
    cargo build --release --target ${TARGET}

# now we copy in the source which is more prone to changes and build it
COPY src ./src
# --release not needed, it is implied with install
RUN --mount=type=cache,id=full-build,target=/build/rust-end-to-end-application/target \
    cargo install --path . --target ${TARGET} --root /output

FROM alpine:3.17.2@sha256:ff6bdca1701f3a8a67e328815ff2346b0e4067d32ec36b7992c1fdc001dc8517

RUN addgroup -S appgroup && adduser -S appuser -G appgroup
USER appuser

WORKDIR /app
COPY --from=builder /output/bin/rust-end-to-end-application /app
ENTRYPOINT ["/app/rust-end-to-end-application"]
