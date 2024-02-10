FROM rust:slim

WORKDIR /usr/src/app

COPY . .

RUN apt-get update && apt-get install -y libpq-dev

EXPOSE 3000

CMD ["cargo", "run"]