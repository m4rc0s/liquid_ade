# Liquid ADE — Deterministic Workspace Automation

default:
    @just --list

# Validate all specifications and code standards
check:
    python3 .agents/skills/scpe/scripts/scpe.py validate --strict
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets -- -D warnings
    cd apps/ade/ui && bun run lint
    cd apps/ade/ui && bunx tsc -b --noEmit

# Run linters on backend and frontend
lint:
    cargo clippy --workspace --all-targets -- -D warnings
    cd apps/ade/ui && bun run lint

# Format Rust and TypeScript source code
fmt:
    cargo fmt --all

# Run backend test suite
test:
    cargo test

# Build production binary with embedded static SPA
build:
    cargo build --release

# Run local development server
dev:
    cargo run
