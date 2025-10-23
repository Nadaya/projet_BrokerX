# Étape 1 : Builder le microservice account-service
FROM rustlang/rust:nightly AS builder

WORKDIR /usr/src/portefeuille-service

# Copier les manifests du microservice
COPY Cargo.toml ./
COPY src ./src

# Compiler en release
RUN cargo build --release

# Étape 2 : Image finale minimale
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libpq-dev libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copier le binaire compilé
COPY --from=builder /usr/src/account-service/target/release/portefeuille-service /app/app

EXPOSE 8081

CMD ["/app/app"]
