# Memoru

`Memoru` is a simple CLI memo application built with Rust and SQLite3. It allows you to create, read, update, and delete memos from the command line.

“Memo-ru” means “to make a memo” or “to take notes” in Japanese.

## Features

- Add new memos
- List all memos
- View a specific memo
- Update existing memos
- Delete memos
- Automatic timestamp tracking (creation and update times)

## Installation

### From homebrew

```bash
brew install kohbis/memoru/memoru
```

### From Crates.io

```bash
cargo install memoru
```

## Usage

### Basic Commands

- **Add a memo**:
  ```
  memoru add "Your memo content"
  ```

- **List all memos**:
  ```
  memoru list
  ```

- **View a specific memo**:
  ```
  memoru view <memo_id>
  ```

- **Update a memo**:
  ```
  memoru update <memo_id> "New content"
  ```

- **Delete a memo**:
  ```
  memoru delete <memo_id>
  ```

- **Get help**:
  ```
  memoru --help
  ```

### Examples

```bash
# Add a new memo
memoru add "Buy groceries: milk, eggs, bread"

# List all memos
memoru list

# View memo with ID 1
memoru view 1

# Update memo with ID 1
memoru update 1 "Buy groceries: milk, eggs, bread, cheese"

# Delete memo with ID 1
memoru delete 1
```

## Data Storage

Memos are stored in a SQLite database located at `~/.memoru/memoru.db`. The database is created automatically when you first use the application.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
