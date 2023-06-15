FROM rust:1.70.0 as builder

# Create a new directory
WORKDIR /usr/src/voter

# Weird trick to cache dependencies
# Basically it creates a dummy project, builds it, and then deletes the dummy project
# But this dummy project install the dependencies the real project needs, so the real project can use the cache
# as only the real project files will really change
# COPY Cargo.toml .
# RUN mkdir src && echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && cargo build --release
# RUN rm src/*.rs

# Copy the source code
COPY . .

RUN . ./bin/activate-hermit

# Set PROTOC env var
ENV PROTOC=/usr/src/voter/bin/protoc

# Build for release
RUN cargo build --release

# Run the binary
CMD ["./target/release/voter"]

# Stage 2: Migrations
FROM rust:1.70.0 as refinery

RUN cargo install refinery_cli

RUN mkdir /usr/migrations

COPY ./migrations /usr/migrations

RUN ls /usr/migrations

# Create the refinery.toml file with the connection string and [main] section

CMD ["/usr/local/cargo/bin/refinery", "migrate", "-c", "/usr/migrations/refinery.toml", "-p", "/usr/migrations"]
