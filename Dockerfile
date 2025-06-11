# Rust toolchain setup
FROM --platform=${BUILDPLATFORM} rust:1.87.0@sha256:b571d7b2dc7b9154517eeda87c0c3c97865d432e95ec205e34a194fd2baaff1d AS rust-base

ARG APPLICATION_NAME

RUN rm -f /etc/apt/apt.conf.d/docker-clean \
    && echo 'Binary::apt::APT::Keep-Downloaded-Packages "true";' > /etc/apt/apt.conf.d/keep-cache

# borrowed (Ba Dum Tss!) from
# https://github.com/pablodeymo/rust-musl-builder/blob/7a7ea3e909b1ef00c177d9eeac32d8c9d7d6a08c/Dockerfile#L48-L49
RUN --mount=type=cache,id=apt-cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,id=apt-lib,target=/var/lib/apt,sharing=locked \
    apt-get update \
    && apt-get upgrade --yes \
    && apt-get install --no-install-recommends --yes \
        build-essential \
        musl-dev \
        musl-tools

FROM rust-base AS rust-linux-amd64
ARG TARGET=x86_64-unknown-linux-musl

FROM rust-base AS rust-linux-arm64
ARG TARGET=aarch64-unknown-linux-musl

FROM rust-${TARGETPLATFORM//\//-} AS rust-cargo-build

COPY ./build-scripts/setup-env.sh .
RUN --mount=type=cache,id=apt-cache,from=rust-base,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,id=apt-lib,from=rust-base,target=/var/lib/apt,sharing=locked \
    ./setup-env.sh

RUN rustup target add ${TARGET}

# The following block
# creates an empty app, and we copy in Cargo.toml and Cargo.lock as they represent our dependencies
# This allows us to copy in the source in a different layer which in turn allows us to leverage Docker's layer caching
# That means that if our dependencies don't change rebuilding is much faster
WORKDIR /build
RUN cargo new ${APPLICATION_NAME}

WORKDIR /build/${APPLICATION_NAME}

COPY ./build-scripts/build.sh .

COPY .cargo ./.cargo
COPY Cargo.toml Cargo.lock ./

RUN --mount=type=cache,target=/build/${APPLICATION_NAME}/target \
    --mount=type=cache,id=cargo-git,target=/usr/local/cargo/git/db,sharing=locked \
    --mount=type=cache,id=cargo-registery,target=/usr/local/cargo/registry/,sharing=locked \
    ./build.sh build --release --target ${TARGET}

# Rust full build
FROM rust-cargo-build AS rust-build

WORKDIR /build/${APPLICATION_NAME}

# now we copy in the source which is more prone to changes and build it
COPY src ./src

# ensure cargo picks up on the change
RUN touch ./src/main.rs

# --release not needed, it is implied with install
RUN --mount=type=cache,target=/build/${APPLICATION_NAME}/target \
    --mount=type=cache,id=cargo-git,target=/usr/local/cargo/git/db,sharing=locked \
    --mount=type=cache,id=cargo-registery,target=/usr/local/cargo/registry/,sharing=locked \
    ./build.sh install --path . --locked --target ${TARGET} --root /output

# Container user setup
FROM --platform=${BUILDPLATFORM} alpine:3.22.0@sha256:8a1f59ffb675680d47db6337b49d22281a139e9d709335b492be023728e11715 AS passwd-build

# setting `--system` prevents prompting for a password
RUN addgroup --gid 900 appgroup \
    && adduser --ingroup appgroup --uid 900 --system --shell /bin/false appuser

RUN cat /etc/group | grep appuser > /tmp/group_appuser
RUN cat /etc/passwd | grep appuser > /tmp/passwd_appuser

# Final stage, no `BUILDPLATFORM`, this one is run where it is deployed
FROM scratch

ARG APPLICATION_NAME

COPY --from=passwd-build /tmp/group_appuser /etc/group
COPY --from=passwd-build /tmp/passwd_appuser /etc/passwd

USER appuser

WORKDIR /app

COPY --from=rust-build /output/bin/${APPLICATION_NAME} /app/entrypoint

ENV RUST_BACKTRACE=full

ENTRYPOINT ["/app/entrypoint"]
