# Étape 1 : Builder le microservice account-service
FROM rustlang/rust:nightly AS builder

WORKDIR /portefeuille-service

# Copier les manifests du microservice
COPY . .

# Compiler en release
RUN cargo build --release

# Étape 2 : Image finale minimale
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libpq-dev libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /portefeuille-service

# Copier le binaire compilé
COPY --from=builder /portefeuille-service/target/release/portefeuille-service .

EXPOSE 8081

CMD ["./portefeuille-service"]