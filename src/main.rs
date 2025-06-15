use clap::{Parser, Subcommand};
use rusqlite::Connection;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::path::PathBuf;

mod command;

#[derive(Debug)]
enum AppError {
    Sqlite(rusqlite::Error),
    Io(io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Sqlite(e) => write!(f, "Database error: {}", e),
            AppError::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl Error for AppError {}

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> AppError {
        AppError::Sqlite(err)
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> AppError {
        AppError::Io(err)
    }
}

type Result<T> = std::result::Result<T, AppError>;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new memo
    Add {
        /// Content of the memo
        #[arg(required = true)]
        content: String,
    },
    /// List all memos
    List,
    /// View a specific memo
    View {
        /// ID of the memo to view
        #[arg(required = true)]
        id: i64,
    },
    /// Update a memo
    Update {
        /// ID of the memo to update
        #[arg(required = true)]
        id: i64,
        /// New content for the memo
        #[arg(required = true)]
        content: String,
    },
    /// Delete a memo
    Delete {
        /// ID of the memo to delete
        #[arg(required = true)]
        id: i64,
    },
}

fn main() -> Result<()> {
    let data_dir = get_data_dir();
    // Handle std::io::Error separately
    if let Err(e) = fs::create_dir_all(&data_dir) {
        eprintln!("Failed to create data directory: {}", e);
        std::process::exit(1);
    }

    let db_path = data_dir.join("memoru.db");
    let conn = Connection::open(db_path)?;

    // Initialize the database
    conn.execute(
        "CREATE TABLE IF NOT EXISTS memos (
            id INTEGER PRIMARY KEY,
            content TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT
        )",
        [],
    )?;

    let cli = Cli::parse();

    match cli.command {
        Some(command) => match command {
            Commands::Add { content } => {
                command::add_memo(&conn, &content)?;
            }
            Commands::List => {
                command::list_memos(&conn)?;
            }
            Commands::View { id } => {
                command::view_memo(&conn, id)?;
            }
            Commands::Update { id, content } => {
                command::update_memo(&conn, id, &content)?;
            }
            Commands::Delete { id } => {
                command::delete_memo(&conn, id)?;
            }
        },
        None => {
            command::interactive_mode(&conn)?;
        }
    }

    Ok(())
}

fn get_data_dir() -> PathBuf {
    let home = dirs::home_dir().expect("Could not find home directory");
    home.join(".memoru")
}
