FROM rust

COPY ./ ./

RUN cargo build --release

# Run the binary
CMD ["./target/release/helloworldapp"]