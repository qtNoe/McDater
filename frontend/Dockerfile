# Stage 1: Build
FROM node:20-alpine as builder

WORKDIR /app

# Dependencies installieren
COPY package*.json ./
# HIER GEÄNDERT: npm install statt npm ci
# npm ci schlägt fehl, wenn package-lock.json nicht synchron ist. 
# npm install fixt das beim Build.
RUN npm install

# Source Code kopieren
COPY . .

# Nuxt App bauen
RUN npm run build

# Stage 2: Production Runner
FROM node:20-alpine

WORKDIR /app

# Nur nötige Files für den Output kopieren
COPY --from=builder /app/.output ./.output
# Optional: Public Ordner falls statische Assets nicht im Build sind
# COPY --from=builder /app/public ./public

# Environment Variablen
ENV NUXT_HOST=0.0.0.0
ENV NUXT_PORT=3000
ENV NODE_ENV=production

# Expose internen Port
EXPOSE 3000

# Startbefehl für Nuxt 3 (Nitro Server)
CMD ["node", ".output/server/index.mjs"]