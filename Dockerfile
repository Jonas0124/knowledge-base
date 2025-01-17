# Dockerfile
FROM rust:1.83.0

WORKDIR /usr/knowledge-base
COPY . /usr/knowledge-base

RUN apt-get update && apt-get install -y libmariadb-dev-compat

RUN apt-get update && apt-get install libssl-dev pkg-config

RUN  cargo install diesel_cli --no-default-features --features mysql



CMD ["./knowledge-base"]