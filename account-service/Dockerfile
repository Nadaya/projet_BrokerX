# Étape 1 : Builder le microservice account-service
FROM rustlang/rust:nightly AS builder

WORKDIR /account-service

# Copier les manifests du microservice
COPY . .

# Compiler en release
RUN cargo build --release

# Étape 2 : Image finale minimale
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libpq-dev libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /account-service

# Copier le binaire compilé
COPY --from=builder /account-service/target/release/account-service /app/app

EXPOSE 8080

CMD ["/app/app"]