FROM rust:1.68.0

WORKDIR /usr/src/metric_gitlab_deploys

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

WORKDIR /usr/src/metric_gitlab_deploys/target/release

ENTRYPOINT ["./metric_gitlab_deploys"]