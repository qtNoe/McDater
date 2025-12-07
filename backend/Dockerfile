# Stage 1: Builder
FROM rust:latest as builder

WORKDIR /usr/src/app

# 1. Caching-Layer: Nur Dependencies bauen
# Wir kopieren nur die Manifest-Dateien
COPY Cargo.toml Cargo.lock ./

# Wir erstellen eine Dummy-Datei, damit wir die Dependencies kompilieren können
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Dieser Build lädt alle Crates und kompiliert sie (dauert lange, wird aber gecached)
RUN cargo build --release

# Entferne den Dummy-Code wieder, damit er nicht stört
RUN rm -rf src

# 2. Echten Code kopieren
COPY . .

# --- WICHTIGER FIX ---
# Wir aktualisieren den Zeitstempel der main.rs. 
# Ohne das denkt Cargo manchmal, die Datei sei alt und kompiliert nicht neu!
RUN touch src/main.rs
# ---------------------

# 3. Finaler Build des echten Codes
RUN cargo build --release


# Stage 2: Runtime (kleines Image)
FROM debian:bookworm-slim

WORKDIR /app

# Installiere notwendige Runtime-Bibliotheken
# libssl3 für Verschlüsselung, libmariadb3 für die Datenbankverbindung
RUN apt-get update && \
    apt-get install -y \
    libssl3 \
    libmariadb3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Kopiere das kompilierte Binary aus dem Builder-Stage
# HINWEIS: Prüfe in deiner Cargo.toml, ob [package] name = "backend" heißt.
# Falls dein Projekt anders heißt, musst du "backend" hier anpassen!
COPY --from=builder /usr/src/app/target/release/backend ./server

# Kopiere Seed-Dateien (falls dein Code sie zur Laufzeit liest)
COPY seeds ./seeds

# Environment für Produktion vorbereiten
ENV PORT=8080
ENV HOST=0.0.0.0
ENV DATABASE_URL=mysql://zit:zit@localhost:3306/app


EXPOSE 8080

# Starte den Server
CMD ["./server"]