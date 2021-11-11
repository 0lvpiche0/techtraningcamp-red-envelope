FROM rust
WORKDIR /usr/src/myapp
COPY . .

RUN apt-get update && apt-get install -y cmake && rm -rf /var/lib/apt/lists/*

RUN cargo build --release
CMD ["./techtraningcamp-red-envelope"]