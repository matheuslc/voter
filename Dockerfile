FROM rust:1.70.0

# Create a new directory
WORKDIR /usr/src/voter

# Copy the source code
COPY . .

# Build for release
RUN cargo build --release

# Run the binary
CMD ["./target/release/voter"]
