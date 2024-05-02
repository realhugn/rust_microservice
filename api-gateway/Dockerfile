FROM rust:slim-buster

RUN apt-get update && \
  apt-get -y upgrade && \
  apt-get -y install libpq-dev pkg-config libssl-dev

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch

WORKDIR /api-gateway
COPY . /api-gateway/
COPY .env.docker /api-gateway/.env

EXPOSE 8080
ENTRYPOINT ["/bin/bash", "-c", "./wait-for-it.sh db:5432 -q -- diesel setup && cargo run --release"]

