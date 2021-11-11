FROM rust
WORKDIR /usr/src/myapp
COPY . .

RUN sed -i "s@http://deb.debian.org@http://mirrors.aliyun.com@g" /etc/apt/sources.list \
    && apt-get update && apt-get install -y cmake && rm -rf /var/lib/apt/lists/*

RUN cargo build --release
CMD ["./target/release/techtraningcamp-red-envelope"]