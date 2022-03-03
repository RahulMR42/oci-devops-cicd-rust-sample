# Rust as the base image
FROM rust

#Switch to a work directory
WORKDIR /helloworldapp

# 2. Copy cargo manifests
COPY ./Cargo.toml ./Cargo.toml

# 3. Build with dependecies
RUN cargo build --release
RUN rm src/*.rs

# 4. Copy source
COPY ./src ./src

# 5. Cargo release 
RUN rm ./target/release/deps/helloworldapp*
RUN cargo install --path .

CMD ["helloworldapp"]