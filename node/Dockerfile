FROM rust:1.81 as build
WORKDIR /app
COPY Cargo.toml .
RUN mkdir src && echo 'fn main(){}' > src/main.rs
RUN cargo build --release || true

COPY . .
RUN cargo build --release

FROM debian:stable-slim
WORKDIR /app
COPY --from=build /app/target/release/people-cloud-node /usr/local/bin/people-cloud-node
EXPOSE 8787
CMD ["people-cloud-node"]