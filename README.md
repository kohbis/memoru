# Memoru

`Memoru` is a simple CLI memo application built with Rust and SQLite3. It allows you to create, read, update, and delete memos from the command line.

"Memo-ru" means "to make a memo" or "to take notes" in Japanese.

## Features

- Add new memos
- List all memos
- View a specific memo
- Update existing memos
- Delete memos
- Automatic timestamp tracking (creation and update times)
- Interactive mode for menu-driven operation

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

### Interactive Mode

Run `memoru` without any arguments to enter interactive mode:

```bash
memoru
```

This will display a menu-driven interface:

```
=== Memoru Interactive Mode ===
1. Add new memo
2. List all memos
3. View a memo
4. Update a memo
5. Delete a memo
6. Exit

Select an option (1-6):
```

The interactive mode is perfect for users who prefer a guided, menu-driven approach to managing their memos.

### Command-Line Mode

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
