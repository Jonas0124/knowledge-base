# Dockerfile
FROM rust:1.83.0

WORKDIR /usr/knowledge-base
COPY . /usr/knowledge-base
COPY ./diesel /usr/local/bin/diesel
RUN apt-get update && apt-get install -y libmariadb-dev-compat && \
    apt-get install libssl-dev pkg-config
#    && \
#    cargo install diesel_cli --no-default-features --features mysql

CMD ["./knowledge-base"]