# Rust as the base image
FROM rust

#Create and switch to a work directory
RUN USER=root cargo new --bin helloworldapp
WORKDIR /helloworldapp

# Copy cargo manifests
COPY ./Cargo.toml ./Cargo.toml

# Build with dependecies
RUN cargo build --release
RUN rm src/*.rs

# Copy source
COPY ./src ./src

# Cargo release 
RUN rm ./target/release/deps/helloworldapp*
RUN cargo install --path .

CMD ["helloworldapp"]