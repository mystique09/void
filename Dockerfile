FROM rust:1.75.0-alpine AS builder

WORKDIR /usr/src/void
COPY . . 

# Create a .env file to avoid error.
RUN cp .sample.env .env

# Build the binary.
RUN cargo install --path .
# Install necesary dependencies.
RUN cargo install sqlx-cli --no-default-features --features postgres,rustls

# Run sqlx migrate command to migrate the database, this uses the sqlx-data.json file.
RUN cargo sqlx migrate run

# Second stage building, to avoid bloated binary.
FROM debian:buster-slim

WORKDIR /app
# Copy the built binary to the second stage.
COPY --from=builder /usr/src/void/target/release/void-bot .

ENTRYPOINT ["./void-bot"]
