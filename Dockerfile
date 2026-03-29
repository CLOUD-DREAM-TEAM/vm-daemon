############## Planning stage ##############
FROM rust:trixie AS planner
RUN cargo install cargo-chef

WORKDIR /app

COPY src /app/src
COPY Cargo.toml /app/Cargo.toml
COPY Rocket.toml /app/Rocket.toml

RUN cargo chef prepare --recipe-path recipe.json

############## Caching stage ##############
FROM rust:trixie AS cacher
RUN cargo install cargo-chef

WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json

# RUN cargo chef cook --recipe-path recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

############## Build stage ##############
FROM rust:trixie AS builder

ENV USER=web
ENV UID=1001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /app

COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
COPY --from=planner /app/src /app/src
COPY --from=planner /app/Cargo.toml /app/Cargo.toml
COPY --from=planner /app/Rocket.toml /app/Rocket.toml

# RUN cargo build
RUN cargo build --release

############## Production stage ##############
# FROM rust:slim-trixie

# FROM rust:alpine3.23
# RUN apk update && apk upgrade
# RUN apk add gcompat
# RUN apk add openssl-dev

FROM gcr.io/distroless/cc-debian13
# COPY --from=builder /usr/lib/x86_64-linux-gnu/libpq.so.5 /usr/lib/x86_64-linux-gnu
# COPY --from=builder /usr/lib/x86_64-linux-gnu/libgssapi_krb5.so.2 /usr/lib/x86_64-linux-gnu
# COPY --from=builder /usr/lib/x86_64-linux-gnu/libldap_r.so /usr/lib/x86_64-linux-gnu
# COPY --from=builder /usr/lib/x86_64-linux-gnu/libkrb5.so.3 /usr/lib/x86_64-linux-gnu
# COPY --from=builder /usr/lib/x86_64-linux-gnu/libk5crypto.so.3 /usr/lib/x86_64-linux-gnu
# COPY --from=builder /lib/x86_64-linux-gnu/libcom_err.so.2 /usr/lib/x86_64-linux-gnu
# COPY --from=builder /usr/lib/x86_64-linux-gnu/libkrb5support.so.0 /usr/lib/x86_64-linux-gnu
# COPY --from=builder /usr/lib/x86_64-linux-gnu/liblber.so.2 /usr/lib/x86_64-linux-gnu
# COPY --from=builder /usr/lib/x86_64-linux-gnu/libsasl2.so.2 /usr/lib/x86_64-linux-gnu
# COPY --from=builder /usr/lib/x86_64-linux-gnu/libgnutls.so.30 /usr/lib/x86_64-linux-gnu
# COPY --from=builder /lib/x86_64-linux-gnu/libkeyutils.so.1 /usr/lib/x86_64-linux-gnu
# COPY --from=builder /usr/lib/x86_64-linux-gnu/libp11-kit.so.0 /usr/lib/x86_64-linux-gnu
# COPY --from=builder /usr/lib/x86_64-linux-gnu/libidn2.so.0 /usr/lib/x86_64-linux-gnu
# COPY --from=builder /usr/lib/x86_64-linux-gnu/libunistring.so.5 /usr/lib/x86_64-linux-gnu
# COPY --from=builder /usr/lib/x86_64-linux-gnu/libtasn1.so.6 /usr/lib/x86_64-linux-gnu
# COPY --from=builder /usr/lib/x86_64-linux-gnu/libnettle.so.8 /usr/lib/x86_64-linux-gnu
# COPY --from=builder /usr/lib/x86_64-linux-gnu/libhogweed.so.6 /usr/lib/x86_64-linux-gnu
# COPY --from=builder /usr/lib/x86_64-linux-gnu/libgmp.so.10 /usr/lib/x86_64-linux-gnu
# COPY --from=builder /usr/lib/x86_64-linux-gnu/libffi.so /usr/lib/x86_64-linux-gnu
# COPY --from=builder /usr/lib/x86_64-linux-gnu/libffi.so.8 /usr/lib/x86_64-linux-gnu
# COPY --from=builder /usr/lib/x86_64-linux-gnu/libffi.so.8.1.4 /usr/lib/x86_64-linux-gnu

ENV PATH="/app:${PATH}"

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
USER web:web

WORKDIR /app
# COPY --from=builder /app/target/debug/vm_runner .
COPY --from=builder /app/target/release/vm_runner .
COPY --from=builder /app/Rocket.toml .

EXPOSE 80

CMD ["./vm_runner"]
