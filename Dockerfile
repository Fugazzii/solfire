FROM rust:slim

WORKDIR /usr/src/app

COPY . .
RUN cargo build --release

CMD ["/usr/src/app/target/debug/solfire"]