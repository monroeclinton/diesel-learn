FROM rust:1.67.1

WORKDIR /diesel-learn

RUN cargo install diesel_cli --no-default-features --features postgres
