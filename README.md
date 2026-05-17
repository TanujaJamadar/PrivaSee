# PrivaSee

PrivaSee is a privacy-traffic visibility dashboard. Give it a website URL and it opens the site in a headless browser, watches the outgoing network requests, flags common privacy risks, and plots request destinations on a live world map.

It is built with:

- Rust + Axum for the backend web server
- Socket.IO for live scan updates
- Puppeteer for browser-based traffic capture
- MaxMind GeoLite2 for request geolocation
- A single-page HTML/CSS/JS dashboard using Leaflet maps

## What It Shows

- Total network requests made by the scanned page
- Third-party tracker detection
- HTTP request warnings
- Possible email/PII leaks in request URLs and POST bodies
- Destination countries for network calls
- A simple compliance score inspired by DPDP Act 2023 checks
- A downloadable Privacy Evidence Report with scan summary and request evidence

## Standout Feature: Privacy Evidence Report

PrivaSee now builds a live evidence report while the scan is running. It turns raw network traffic into a structured audit artifact that can be exported as JSON.

The report includes:

- Target URL and scan timestamps
- Final compliance grade and risk score
- Total requests and violations
- Unique tracker domains
- Countries contacted during the scan
- Critical findings count
- Top contacted domain
- Most common privacy issue
- Full request-level evidence trail



## Project Structure

```text
PrivaSee/
+-- backend/
|   +-- src/
|   |   +-- main.rs       # Axum server + Socket.IO setup
|   |   +-- worker.rs     # Launches the Node/Puppeteer scanner
|   |   +-- analyzer.rs   # Tracker, HTTP, and PII checks
|   |   +-- geo.rs        # GeoIP lookup
|   |   +-- types.rs      # Shared Rust data types
|   +-- scanner.js        # Puppeteer network scanner
|   +-- GeoLite2-City.mmdb
|   +-- Cargo.toml
|   +-- package.json
+-- frontend/
|   +-- index.html        # Dashboard UI
+-- Dockerfile
+-- README.md
```

## Prerequisites

For the easiest run, install:

- Docker Desktop

For local development without Docker, install:

- Rust, including `cargo`
- Node.js
- pnpm, or enable it through Corepack
- Chrome/Chromium, if Puppeteer cannot download/use its bundled browser

## Run With Docker

From the project root:

```powershell
cd C:\Users\Hp\OneDrive\Documents\PrivaSEE\PrivaSee
docker build -t privasee .
docker run --rm -p 3000:3000 privasee
```

Open:

```text
http://localhost:3000
```

## Run Locally On Windows

From the project root:

```powershell
cd C:\Users\Hp\OneDrive\Documents\PrivaSEE\PrivaSee
```

Install the Node worker dependency:

```powershell
cd backend
corepack enable
corepack prepare pnpm@latest --activate
pnpm install
```

If pnpm is not available, this project can also install the dependency with npm:

```powershell
npm install
```

Start the Rust backend from the `backend` folder:

```powershell
$env:INDEX_HTML="..\frontend\index.html"
$env:GEOLITE_DB=".\GeoLite2-City.mmdb"
$env:SCANNER_DIR="."
$env:SCANNER_JS=".\scanner.js"
$env:PORT="3000"
cargo run
```

Open:

```text
http://localhost:3000
```

## Run Locally On macOS/Linux

From the project root:

```bash
cd PrivaSee
cd backend
corepack enable
corepack prepare pnpm@latest --activate
pnpm install
```

Then start the backend:

```bash
INDEX_HTML="../frontend/index.html" \
GEOLITE_DB="./GeoLite2-City.mmdb" \
SCANNER_DIR="." \
SCANNER_JS="./scanner.js" \
PORT="3000" \
cargo run
```

Open:

```text
http://localhost:3000
```

## How To Use

1. Start the server.
2. Open `http://localhost:3000`.
3. Enter a full URL, for example `https://instagram.com`.
4. Click `SCAN`.
5. Watch the request count, violations, traffic feed, compliance grade, evidence report, and map update in real time.
6. Click `EXPORT` after the scan completes to download the Privacy Evidence Report.

Always include the protocol in the URL:

```text
https://example.com
```

## Environment Variables

| Variable | Purpose | Docker default |
| --- | --- | --- |
| `PORT` | Server port | `3000` |
| `INDEX_HTML` | Path to the frontend HTML file | `/app/dist/index.html` |
| `GEOLITE_DB` | Path to the GeoLite2 database | `/app/GeoLite2-City.mmdb` |
| `SCANNER_DIR` | Working directory for the Node scanner | `/app/backend` |
| `SCANNER_JS` | Path to `scanner.js` | `/app/backend/scanner.js` |
| `PUPPETEER_EXECUTABLE_PATH` | Chromium path used by Puppeteer | `/usr/bin/chromium` |

## Troubleshooting

### `cargo` is not recognized

Install Rust from:

```text
https://rustup.rs/
```

Close and reopen the terminal after installing.

### `pnpm` is not recognized

Run:

```powershell
corepack enable
corepack prepare pnpm@latest --activate
```

Or use:

```powershell
npm install
```

### Puppeteer or Chromium fails to launch

Docker is the easiest way to avoid local browser setup issues.

For local runs, make sure Puppeteer installed correctly:

```powershell
cd backend
npm install
```

If you already have Chrome/Chromium installed, you can point Puppeteer to it:

```powershell
$env:PUPPETEER_EXECUTABLE_PATH="C:\Path\To\chrome.exe"
cargo run
```

### Docker build fails with `node:sqlite` or pnpm requires Node 22

The Dockerfile pins pnpm to a Node 20 compatible version. If this error appears after editing the Dockerfile, make sure the Node builder stage contains:

```dockerfile
RUN corepack enable && corepack prepare pnpm@10.23.0 --activate
```

Then rebuild:

```powershell
docker build --no-cache -t privasee .
```

### `Failed to load GeoLite2-City.mmdb`

Make sure the GeoLite database exists at:

```text
backend/GeoLite2-City.mmdb
```

For local Windows runs, start the server from `backend` and set:

```powershell
$env:GEOLITE_DB=".\GeoLite2-City.mmdb"
```

### The page opens but no scan happens

Check that:

- The URL starts with `https://` or `http://`
- The backend is still running
- Node dependencies are installed in `backend`
- The target website is reachable from your machine

## Build Check

To compile the Rust backend:

```powershell
cd backend
cargo check
```

To build a release binary:

```powershell
cd backend
cargo build --release
```

## Notes

PrivaSee performs a browser-based scan from your own machine or container. Results can vary depending on network, location, cookie state, target-site behavior, anti-bot systems, and whether third-party scripts load successfully.
