FROM rust:1.45 AS build

RUN USER=root cargo new --lib puzzle_maker 
WORKDIR /puzzle_maker 
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

RUN rm -rf ./src
COPY src ./src
RUN cargo build --release  

FROM ubuntu:focal
RUN USER=root apt update && apt install -y libssl1.1
COPY --from=build /puzzle_maker/target/release/puzzle_maker /puzzle_maker

CMD ["/bin/sh", "-c", "/puzzle_maker 37"] 
