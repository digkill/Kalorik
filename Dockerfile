FROM rust:1.88

WORKDIR /app
COPY . .

RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo build --release

CMD ["cargo", "run"]
