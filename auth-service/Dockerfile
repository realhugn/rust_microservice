FROM rust:slim-buster

RUN apt-get update && \
  apt-get -y upgrade && \
  apt-get -y install libpq-dev pkg-config libssl-dev

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch

WORKDIR /auth-service
COPY . /auth-service/
COPY .env.docker /auth-service/.env

EXPOSE 8082

ENTRYPOINT ["/bin/bash", "-c", "cargo run --release"]
