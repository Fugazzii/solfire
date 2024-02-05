FROM rust:slim

WORKDIR /app

COPY Cargo.toml .
RUN cargo build --release

COPY . .

EXPOSE 3000

CMD ["cargo", "run"]