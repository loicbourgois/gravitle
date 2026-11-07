FROM rust:1-bookworm
RUN apt-get update && apt-get install -y nodejs npm coreutils && rm -rf /var/lib/apt/lists/*
RUN rustup component add rust-std && rustup target add wasm32-unknown-unknown
RUN cargo install wasm-pack wasm-bindgen-cli
WORKDIR /root/github.com/loicbourgois/gravitle/front/moonlighter
CMD ["/root/github.com/loicbourgois/gravitle/front/moonlighter/scripts/watch.sh"]
