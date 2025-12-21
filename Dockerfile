###############################################################################
## Builder
###############################################################################
FROM rust:alpine3.23 AS builder

LABEL maintainer="Lorenzo Carbonell <a.k.a. atareao> lorenzo.carbonell.cerezo@gmail.com"

RUN apk add --update --no-cache \
            autoconf \
            gcc \
            gdb \
            make \
            musl-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src src

RUN cargo build --release && \
    cp /app/target/release/miniflux-client /app/miniflux-client

###############################################################################
## Final image
###############################################################################
FROM alpine:3.23

ENV USER=app
ENV UID=10001

RUN apk add --update --no-cache \
            ca-certificates \
            curl \
            openssl \
            tzdata~=2025 && \
    rm -rf /var/cache/apk && \
    rm -rf /var/lib/app/lists*

# Copy our build
COPY --from=builder /app/miniflux-client /app/

# Set the work dir
WORKDIR /app
#USER app

CMD ["/app/miniflux-client"]
