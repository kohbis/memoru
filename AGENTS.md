# AGENTS.md

This file provides guidance for AI agents working with this codebase.

## Project Overview

**Memoru** is a simple CLI memo application built with Rust and SQLite3. The name comes from Japanese "Memo-ru" (メモる), meaning "to take notes."

## Tech Stack

- **Language**: Rust (Edition 2024)
- **Database**: SQLite3 (via `rusqlite`)
- **CLI Framework**: `clap` with derive macros
- **Table Display**: `comfy-table`
- **Date/Time**: `chrono`

## Project Structure

```
memoru/
├── Cargo.toml          # Project dependencies and metadata
├── src/
│   ├── main.rs         # Entry point, CLI parsing, database setup
│   └── command.rs      # CRUD operations and interactive mode
└── ~/.memoru/
    └── memoru.db       # SQLite database (created at runtime)
```

## Key Components

### main.rs
- CLI argument parsing with `clap`
- Database connection setup
- Command routing to `command.rs` functions
- Custom error type `AppError` for SQLite and IO errors

### command.rs
- `add_memo()` - Create a new memo
- `list_memos()` - Display all memos in a table (ordered by ID ASC)
- `view_memo()` - Display a single memo in table format
- `update_memo()` - Modify existing memo content
- `delete_memo()` - Remove a memo
- `interactive_mode()` - Menu-driven TUI interface

## Interactive Mode

When run without arguments, memoru enters interactive mode:
- Shows memo list on startup
- Menu options use alphabet keys: `a` (add), `l` (list), `v` (view), `u` (update), `d` (delete), `q` (quit)
- Entering a number directly views that memo ID

## Database Schema

```sql
CREATE TABLE memos (
    id INTEGER PRIMARY KEY,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT
)
```

Timestamps are stored in RFC3339 format.

## Build & Run

```bash
# Build
cargo build

# Run in interactive mode
cargo run

# Run with subcommand
cargo run -- add "memo content"
cargo run -- list
cargo run -- view 1
cargo run -- update 1 "new content"
cargo run -- delete 1
```

## Code Style Guidelines

- Use `Result<T>` type alias with custom `AppError`
- Table output uses `comfy-table` with `UTF8_FULL` preset
- Timestamps displayed as `YYYY-MM-DD HH:MM:SS`
- Interactive mode prompts flush stdout before reading input

## Testing

Currently no automated tests. Manual testing via CLI commands and interactive mode.

## Distribution

- Published to crates.io
- Homebrew tap: `kohbis/memoru/memoru`
