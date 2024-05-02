FROM rust:slim-buster

RUN apt-get update && \
  apt-get -y upgrade && \
  apt-get -y install libpq-dev pkg-config libssl-dev cmake make curl zstd build-essential

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch

WORKDIR /notification_service
COPY . /notification_service/
COPY .env.docker /notification_service/.env

EXPOSE 8083

ENTRYPOINT ["/bin/bash", "-c", "cargo run --release"]

