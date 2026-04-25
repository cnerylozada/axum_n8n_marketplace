# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build
cargo build
cargo build --release

# Run (requires Postgres connection)
cargo run

# Check / lint
cargo check
cargo clippy

# Run tests
cargo test

# Run a single test
cargo test <test_name>

# Apply migrations (requires sqlx-cli)
sqlx migrate run

# Deploy to Fly.io
fly deploy
```

## Architecture

This is an Axum-based REST API in Rust backed by a PostgreSQL database (Supabase). The app is deployed to Fly.io.

**Module layout:**
- `src/main.rs` — sets up the DB connection pool, composes the router, and starts the server on `:8080`
- `src/inventory/` — feature module following a `routes → handlers → models` pattern
  - `routes.rs` — registers HTTP routes and wires them to handlers
  - `handlers.rs` — contains the async handler functions; extracts state/path/body, runs queries, returns results
  - `models.rs` — structs for DB rows (`FromRow`) and request payloads (`Deserialize + Validate`)

**Request flow:** `Router (routes.rs)` → `Handler (handlers.rs)` → raw `sqlx` queries → `Model (models.rs)`

**Database:**
- `products` table (UUID PK, name)
- `inventory_variants` table (UUID PK, foreign key to `products.id`, size constrained to `XS/SM/M/L`, unique constraint on `(product_id, size)`)
- `tokens` table (UUID PK, mint_address — Solana/SVM token mints)
- Migrations live in `migrations/` and are managed with `sqlx migrate`

**Validation:** Payload structs use the `validator` crate with `#[derive(Validate)]`. Handlers call `.validate()` manually after deserialization.

**Adding a new feature module:** Create a directory under `src/`, add `mod.rs` exporting `handlers`, `models`, and `routes` sub-modules, then `mod` it in `main.rs` and nest its routes under `/api/v1`.

## Deployment

The app runs on Fly.io (`fly.toml`). The Docker image uses `cargo-chef` for layer caching. Region: `gru` (São Paulo). Port: `8080`.
