###############################################################################
## Builder
###############################################################################
FROM rust:alpine3.23 AS builder

LABEL maintainer="Lorenzo Carbonell <a.k.a. atareao> lorenzo.carbonell.cerezo@gmail.com"

RUN apk update && apk upgrade && \
    apk add --no-cache \
            build-base \
            autoconf \
            gdb

WORKDIR /app

COPY Cargo.toml ./
COPY src src

RUN cargo generate-lockfile && \
    cargo build --release && \
    cp /app/target/release/miniflux-client /app/miniflux-client

###############################################################################
## Final image
###############################################################################
FROM alpine:3.23

ENV USER=app \
    UID=10001

RUN apk add --update --no-cache \
            ca-certificates \
            curl \
            openssl \
            tzdata~=2026 && \
    rm -rf /var/cache/apk/*

# Create a non-privileged user for security
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

# Copy our build
COPY --from=builder /app/miniflux-client /app/

# Set the work dir
WORKDIR /app
RUN chown -R ${USER}:${USER} /app
USER app

CMD ["/app/miniflux-client"]
