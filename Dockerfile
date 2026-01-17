# ---------- node deps for worker ----------
FROM node:20-bookworm-slim AS node_builder
WORKDIR /app/backend
RUN corepack enable

COPY backend/package.json backend/pnpm-lock.yaml backend/pnpm-workspace.yaml ./
RUN pnpm install --frozen-lockfile --prod || pnpm install --prod

COPY backend/scanner.js ./scanner.js
COPY backend/src ./src


# ---------- rust build ----------
FROM rust:1.86-bookworm AS rust_builder
WORKDIR /app

COPY backend/ ./backend/
RUN cd backend && cargo build --release


# ---------- runtime ----------
FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    nodejs \
    chromium \
    fonts-liberation \
    libasound2 \
    libatk-bridge2.0-0 \
    libatk1.0-0 \
    libcups2 \
    libdbus-1-3 \
    libdrm2 \
    libgbm1 \
    libglib2.0-0 \
    libnss3 \
    libx11-6 \
    libx11-xcb1 \
    libxcb1 \
    libxcomposite1 \
    libxdamage1 \
    libxext6 \
    libxfixes3 \
    libxkbcommon0 \
    libxrandr2 \
    libxrender1 \
    libxshmfence1 \
    libxss1 \
    libxtst6 \
  && rm -rf /var/lib/apt/lists/*


# ---- Rust binary ----
COPY --from=rust_builder /app/backend/target/release/backend /app/server

# ---- Static frontend ----
COPY frontend/ /app/dist/

# ---- GeoLite DB ----
COPY backend/GeoLite2-City.mmdb /app/GeoLite2-City.mmdb

# ---- Node worker runtime ----
COPY --from=node_builder /app/backend /app/backend
ENV NODE_PATH=/app/backend/node_modules

# Defaults (you can override in Railway variables)
ENV INDEX_HTML=/app/dist/index.html
ENV GEOLITE_DB=/app/GeoLite2-City.mmdb

ENV SCANNER_DIR=/app/backend
ENV SCANNER_JS=/app/backend/scanner.js

ENV PUPPETEER_SKIP_DOWNLOAD=true
ENV PUPPETEER_EXECUTABLE_PATH=/usr/bin/chromium

EXPOSE 3000
CMD ["/app/server"]

