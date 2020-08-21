FROM rust:1.45 AS build

RUN USER=root cargo new --lib puzzle_maker
WORKDIR /puzzle_maker
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo build --release

FROM rust:1.45
COPY --from=build /puzzle_maker/target/release/puzzle_maker /puzzle_maker

CMD ["/bin/sh", "-c", "/puzzle_maker 38"]
