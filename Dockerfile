FROM rust:1.44-slim AS build

ARG arch=aarch64-unknown-linux-musl

RUN rustup target add $arch

RUN USER=root cargo new puzzle_maker 
WORKDIR /puzzle_maker 
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
# the additional flags are needed as a workaround for a build break. They are needed only for linux and will break the build on macos, etc.
RUN cargo rustc --target $arch --bin gmd --release -- -C target-feature=+crt-static -C link-arg=-lgcc
# Ugly hack here because it appears we can only use $arch in a RUN statement ...
RUN cp ./target/$arch/release/gmd ./

FROM busybox
COPY --from=build /puzzle_maker /puzzle_maker /

CMD ["/bin/sh", "-c", "/puzzle_maker 38"]
