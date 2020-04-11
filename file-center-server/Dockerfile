# build stage
FROM rust as build
WORKDIR /app

# copy manifests
COPY ./Cargo.toml ./Cargo.toml

# copy project source and necessary files
COPY ./src ./src
COPY ./sql ./sql
COPY ./file-center-db .
COPY .env .
RUN cargo build --release

# deploy stage
FROM debian

# create app directory
WORKDIR /app

# copy binary and configuration files
COPY --from=build /app/target/release/file-center .
COPY --from=build /app/sql ./sql
COPY --from=build /app/.env .
COPY --from=build /app/file-center-db .

# expose port
EXPOSE 8080

# run the binary
ENTRYPOINT ["/app/file-center"]
