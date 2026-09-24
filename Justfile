# Liquid ADE — Deterministic Workspace Automation

default:
    @just --list

# Validate all specifications, formatting, linters, and type checking
check:
    python3 .agents/skills/scpe/scripts/scpe.py validate --strict
    cargo fmt --all -- --check
    cd apps/ade/ui && bunx prettier --check "src/**/*.{ts,tsx,css,json}"
    cargo clippy --workspace --all-targets -- -D warnings
    cd apps/ade/ui && bun run lint
    cd apps/ade/ui && bunx tsc -b --noEmit

# Run linters on backend and frontend
lint:
    cargo clippy --workspace --all-targets -- -D warnings
    cd apps/ade/ui && bun run lint

# Format Rust and TypeScript/React source code
fmt:
    cargo fmt --all
    cd apps/ade/ui && bunx prettier --write "src/**/*.{ts,tsx,css,json}"

# Run backend test suite
test:
    cargo test

# Build production binary with embedded static SPA
build:
    cargo build --release

# Run local development server
dev:
    cargo run

# Start a feature/epic development branch from main
branch-start name:
    git checkout -b feat/{{name}}

# Squash-merge a completed feature branch into main with title and curated body, then delete branch
branch-merge name title body="":
    git checkout main
    git merge --squash feat/{{name}}
    @if [ -n "{{body}}" ]; then \
        git commit -m "{{title}}" -m "{{body}}"; \
    else \
        git commit -m "{{title}}"; \
    fi
    git branch -D feat/{{name}}


