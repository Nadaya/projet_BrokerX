# Étape 1 : Builder l'application Rust
FROM rustlang/rust:nightly AS builder

WORKDIR /usr/src/app

# Copier uniquement les manifests pour profiter du cache Docker
COPY Cargo.toml Cargo.lock ./

# Copier le code source
COPY src ./src

# Compiler en release
RUN cargo build --release

# Étape 2 : Image finale
FROM debian:bookworm-slim

# Installer dépendances nécessaires (certificats, SSL, etc.)
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copier le binaire compilé
COPY --from=builder /usr/src/app/target/release/BrokerX /app/app

# Exposer le port de l'app
EXPOSE 8080

# Lancer l'application
CMD ["./app"]
