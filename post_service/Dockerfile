FROM rust:slim-buster

RUN apt-get update && \
  apt-get -y upgrade && \
  apt-get -y install libpq-dev pkg-config libssl-dev make curl build-essential zlib1g-dev valgrind python  openssl libssl-dev

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch

WORKDIR /post_service
COPY . /post_service/
COPY .env.docker /post_service/.env

EXPOSE 8084

ENTRYPOINT ["/bin/bash", "-c", "cargo run --release"]

