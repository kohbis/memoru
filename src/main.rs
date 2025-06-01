use chrono::Local;
use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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

    match &cli.command {
        Commands::Add { content } => {
            add_memo(&conn, content)?;
        }
        Commands::List => {
            list_memos(&conn)?;
        }
        Commands::View { id } => {
            view_memo(&conn, *id)?;
        }
        Commands::Update { id, content } => {
            update_memo(&conn, *id, content)?;
        }
        Commands::Delete { id } => {
            delete_memo(&conn, *id)?;
        }
    }

    Ok(())
}

fn get_data_dir() -> PathBuf {
    let home = dirs::home_dir().expect("Could not find home directory");
    home.join(".memoru")
}

fn add_memo(conn: &Connection, content: &str) -> Result<()> {
    let now = Local::now().to_rfc3339();

    conn.execute(
        "INSERT INTO memos (content, created_at) VALUES (?1, ?2)",
        [content, &now],
    )?;

    println!("Memo added successfully!");
    Ok(())
}

fn list_memos(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, content, created_at FROM memos ORDER BY id DESC")?;
    let memo_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;

    println!("ID | Content | Created At");
    println!("------------------------");

    for memo in memo_iter {
        let (id, content, created_at) = memo?;
        let preview = if content.len() > 30 {
            format!("{}...", &content[..27])
        } else {
            content
        };
        println!("{} | {} | {}", id, preview, created_at);
    }

    Ok(())
}

fn view_memo(conn: &Connection, id: i64) -> Result<()> {
    let mut stmt =
        conn.prepare("SELECT id, content, created_at, updated_at FROM memos WHERE id = ?1")?;
    let memo = stmt.query_row([id], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, Option<String>>(3)?,
        ))
    });

    match memo {
        Ok((id, content, created_at, updated_at)) => {
            println!("ID: {}", id);
            println!("Content: {}", content);
            println!("Created At: {}", created_at);
            if let Some(updated) = updated_at {
                println!("Updated At: {}", updated);
            }
        }
        Err(_) => {
            println!("Memo with ID {} not found", id);
        }
    }

    Ok(())
}

fn update_memo(conn: &Connection, id: i64, content: &str) -> Result<()> {
    let now = Local::now().to_rfc3339();

    let rows_affected = conn.execute(
        "UPDATE memos SET content = ?1, updated_at = ?2 WHERE id = ?3",
        [content, &now, &id.to_string()],
    )?;

    if rows_affected > 0 {
        println!("Memo updated successfully!");
    } else {
        println!("Memo with ID {} not found", id);
    }

    Ok(())
}

fn delete_memo(conn: &Connection, id: i64) -> Result<()> {
    let rows_affected = conn.execute("DELETE FROM memos WHERE id = ?1", [id])?;

    if rows_affected > 0 {
        println!("Memo deleted successfully!");
    } else {
        println!("Memo with ID {} not found", id);
    }

    Ok(())
}
