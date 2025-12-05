# Stage 1: Build
FROM rust:latest as builder

WORKDIR /usr/src/app

# 1. Kopiere nur die Manifest-Dateien, um Dependencies zu cachen
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm src/main.rs

# 2. Jetzt den echten Code kopieren
COPY . .

# 3. Finaler Build
RUN cargo build --release

# Stage 2: Runtime (kleines Image)
FROM debian:bookworm-slim

WORKDIR /app

# Installiere OpenSSL und MySQL Client Libs 
RUN apt-get update && apt-get install -y libssl3 default-libmysqlclient-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# WICHTIG: Ersetze "mcdater_backend" durch den Namen aus deiner Cargo.toml!
COPY --from=builder /usr/src/app/target/release/backend ./server

# Expose den Port, auf dem Rust läuft
EXPOSE 8080

# Starte das Rust Binary
CMD ["./server"]