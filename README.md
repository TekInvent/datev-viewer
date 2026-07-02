# Teki DATEV Viewer

A lightweight, read-only desktop application for inspecting and validating DATEV EXTF files before importing them into accounting software.

---

## Overview

Teki DATEV Viewer is built for **entrepreneurs** and **accounting professionals** who need to verify the structure and contents of a DATEV export before handing it off to their accountant or importing it into DATEV-compatible software.

It never modifies, repairs, or writes back to your files. What you see is exactly what is in the file.

---

## Features

- **Drag & drop** — open a file by dropping it anywhere on the window
- **File summary** — encoding, line endings, file size, and record count at a glance
- **Header metadata** — all parsed DATEV EXTF header fields displayed as key/value pairs
- **Booking table** — virtualised, scrollable table with:
  - sortable and resizable columns
  - instant full-text search and filtering
  - row and cell selection with clipboard copy (Ctrl+C / ⌘C)
- **Validation** — structural validation of header and every booking record, with severity indicators (Valid / Warning / Invalid)
- **Totals** — booking count, total amount, debit total, credit total, warning count, error count
- **Error screen** — detailed diagnostics for files that cannot be parsed, including the offending line
- **Completely offline** — no network access, no telemetry, no account required

---

## Supported Input

| Property | Value |
|---|---|
| Format | DATEV EXTF CSV |
| Extensions | `.csv`, `.txt` |
| Encodings | UTF-8, UTF-8 with BOM, Windows-1252 |
| Line endings | CRLF, LF |

Format compatibility is determined from file contents, not from the file extension.

---

## Platforms

| Platform | Status |
|---|---|
| macOS (Apple Silicon) | ✅ Supported |
| Windows x64 | ✅ Supported |
| Linux | Not supported in v1 |

---

## Technology Stack

| Layer | Technology |
|---|---|
| Desktop shell | [Tauri 2](https://tauri.app) |
| UI | React 18 + TypeScript |
| Business logic | Rust |
| Bundler | Vite |

All parsing, validation, encoding detection, and totals calculation are implemented in Rust (`datev_core`). The React frontend is responsible for presentation only.

---

## Architecture

```
┌─────────────────────────────────────┐
│          React UI (TypeScript)       │
│  Presentation · State · Tables       │
└──────────────┬──────────────────────┘
               │  Tauri IPC
┌──────────────▼──────────────────────┐
│        Rust Backend (Tauri)          │
│  File I/O · Encoding detection       │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│           datev_core (crate)         │
│  Parser · Validator · Calculator     │
│  Domain model · Totals               │
└─────────────────────────────────────┘
```

`datev_core` is a standalone Rust library crate. It has no dependency on Tauri or any desktop framework and can be reused or tested independently.

---

## Development

### Prerequisites

Development takes place inside an OrbStack (or Docker) container. The container includes all Rust and Node toolchains.

Build the container image once (or after `Dockerfile` changes):

```sh
docker compose build
```

Install Node dependencies:

```sh
docker compose run --rm dev npm install
```

### Common Commands

| Task | Command |
|---|---|
| TypeScript type-check | `docker compose run --rm dev npx tsc --noEmit` |
| Rust compilation check | `docker compose run --rm dev cargo check --workspace` |
| Run Rust tests | `docker compose run --rm dev cargo test --workspace` |
| Rust linter | `docker compose run --rm dev cargo clippy --workspace -- -D warnings` |
| Rust formatter check | `docker compose run --rm dev cargo fmt --all -- --check` |

> **Note:** All of the above commands run inside the container. Do not run them directly on the macOS host.

### Running the Desktop App (macOS host only)

```sh
npm run tauri dev
```

This command runs on the macOS host and requires explicit approval before execution.

---

## Project Structure

```
teki-datev-viewer/
├── datev_core/          # Standalone Rust library: parser, validator, calculator
│   └── src/
│       ├── lib.rs       # Public API: process_decoded_file()
│       ├── models.rs    # Domain model (DatevFile, BookingRecord, …)
│       ├── parser.rs    # DATEV EXTF CSV parser
│       ├── reader.rs    # File reading, encoding & line-ending detection
│       ├── validator.rs # Structural validation engine
│       └── calculator.rs# Totals calculation
│
├── src-tauri/           # Tauri application shell (Rust)
│
├── src/                 # React frontend (TypeScript)
│   ├── App.tsx          # Application root, state management
│   ├── types.ts         # TypeScript view models
│   └── components/
│       ├── BookingTable.tsx      # Virtualised booking table
│       └── ValidationResults.tsx # Validation status panel
│
├── Dockerfile
├── docker-compose.yml
└── SPECS.md             # Product Requirements Specification
```

---

## Validation Rules

### File level
- Non-empty, readable file
- Supported and consistent encoding
- Consistent line endings
- Valid CSV structure
- Valid DATEV EXTF identifier in the first field
- Consistent column count across rows

### Header
- Mandatory fields present
- Supported DATEV version (510, 700)
- Consultant and client number format
- Valid date values and accounting period range

### Booking records
- Mandatory fields present (amount, date, account)
- Amount format (numeric, comma as decimal separator)
- Date format (DDMM / DMM / YYYYMMDD)
- Account and contra-account numeric format and length
- Tax key format and length

---

## Safety Guarantees

- The application **never** writes to or modifies an opened file
- The application **never** auto-corrects or silently repairs malformed input
- The application **never** makes network requests
- Given identical input, output is always identical (fully deterministic)

---

## License

Licensed under the [Apache License, Version 2.0](./LICENSE).
