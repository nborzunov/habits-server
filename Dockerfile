                # Use official rust as base image
                FROM rust:latest as builder

                # Set the working directory
                WORKDIR /app

                # Copy the Cargo.toml file
                COPY Cargo.toml .

                # Copy the rest of the server code
                COPY . .

                # Build the Rust app
                RUN cargo build --release

                # Use an official alpine as the base image
                FROM alpine:latest

                # Copy the built Rust app and its dependencies
                COPY --from=builder /app/target/release/habits /app

                # Expose the port for the Rust server
                EXPOSE 8080

                # Start the Rust server
                CMD ["/app"]
