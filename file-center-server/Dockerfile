# build stage
FROM rustlang/rust:nightly-slim as build

# install libpq
RUN apt-get update
RUN apt-get install -y libpq-dev
RUN rm -rf /var/lib/apt/lists/*

# create new empty binary project
RUN USER=root cargo new --bin app
WORKDIR /app

# copy manifests
COPY ./Cargo.toml ./Cargo.toml

# build this project to cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy project source and necessary files
COPY ./src ./src
COPY ./sql ./sql
COPY .env .env 

# rebuild app with project source
RUN rm ./target/release/deps/*
RUN cargo build --release

# deploy stage
FROM debian:stretch-slim

# create app directory
RUN mkdir app
WORKDIR /app

# install libpq
RUN apt-get update
RUN apt-get install -y libpq-dev
RUN rm -rf /var/lib/apt/lists/*

# copy binary and configuration files
COPY --from=build /app/target/release/file-center .
COPY --from=build /app/.env .

# expose port
EXPOSE 8000

# run the binary
ENTRYPOINT ["/app/file-center"]
