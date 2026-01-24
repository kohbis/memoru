use crate::Result;
use chrono::{DateTime, Local};
use comfy_table::{Attribute, Cell, ContentArrangement, Table, presets::UTF8_FULL};
use rusqlite::Connection;

fn format_datetime(datetime_str: &str) -> String {
    DateTime::parse_from_rfc3339(datetime_str)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|_| datetime_str.to_string())
}

pub fn add_memo(conn: &Connection, content: &str) -> Result<()> {
    let now = Local::now().to_rfc3339();

    conn.execute(
        "INSERT INTO memos (content, created_at) VALUES (?1, ?2)",
        [content, &now],
    )?;

    println!("Memo added successfully!");
    Ok(())
}

pub fn list_memos(conn: &Connection) -> Result<()> {
    let mut stmt =
        conn.prepare("SELECT id, content, created_at, updated_at FROM memos ORDER BY id ASC")?;
    let memo_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, Option<String>>(3)?,
        ))
    })?;

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("ID").add_attribute(Attribute::Bold),
            Cell::new("Content").add_attribute(Attribute::Bold),
            Cell::new("Timestamp").add_attribute(Attribute::Bold),
        ]);

    for memo in memo_iter {
        let (id, content, created_at, updated_at) = memo?;
        let preview = if content.len() > 30 {
            format!("{}...", &content[..27])
        } else {
            content
        };
        let timestamp = updated_at.as_ref().unwrap_or(&created_at);
        table.add_row(vec![id.to_string(), preview, format_datetime(timestamp)]);
    }

    println!("{table}");

    Ok(())
}

pub fn view_memo(conn: &Connection, id: i64) -> Result<()> {
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
            let timestamp = updated_at.as_ref().unwrap_or(&created_at);
            println!("Timestamp: {}", format_datetime(timestamp));
            println!("Content: {}", content);
        }
        Err(_) => {
            println!("Memo with ID {} not found", id);
        }
    }

    Ok(())
}

pub fn update_memo(conn: &Connection, id: i64, content: &str) -> Result<()> {
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

fn confirm_delete(id: i64) -> Result<bool> {
    use std::io::{self, Write};

    print!("Are you sure you want to delete memo {}? [y/N]: ", id);
    io::stdout().flush()?;

    let mut response = String::new();
    io::stdin().read_line(&mut response)?;

    Ok(response.trim().eq_ignore_ascii_case("y"))
}

pub fn delete_memo(conn: &Connection, id: i64) -> Result<()> {
    if !confirm_delete(id)? {
        println!("Delete cancelled.");
        return Ok(());
    }

    let rows_affected = conn.execute("DELETE FROM memos WHERE id = ?1", [id])?;

    if rows_affected > 0 {
        println!("Memo deleted successfully!");
    } else {
        println!("Memo with ID {} not found", id);
    }

    Ok(())
}

pub fn interactive_mode(conn: &Connection) -> Result<()> {
    use std::io::{self, Write};

    // Show list on startup
    list_memos(conn)?;

    loop {
        println!("\n=== Memoru Interactive Mode ===");
        println!("[a] Add new memo");
        println!("[l] List all memos");
        println!("[v] View a memo");
        println!("[u] Update a memo");
        println!("[d] Delete a memo");
        println!("[q] Quit");
        print!("\nSelect an option (or enter ID to view): ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim().to_lowercase();
        println!();

        match choice.as_str() {
            "a" => {
                print!("Enter memo content: ");
                io::stdout().flush()?;
                let mut content = String::new();
                io::stdin().read_line(&mut content)?;
                add_memo(conn, content.trim())?;
            }
            "l" => {
                list_memos(conn)?;
            }
            "v" => {
                print!("Enter memo ID: ");
                io::stdout().flush()?;
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str)?;
                if let Ok(id) = id_str.trim().parse::<i64>() {
                    view_memo(conn, id)?;
                } else {
                    println!("Invalid ID format");
                }
            }
            "u" => {
                print!("Enter memo ID: ");
                io::stdout().flush()?;
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str)?;

                print!("Enter new content: ");
                io::stdout().flush()?;
                let mut content = String::new();
                io::stdin().read_line(&mut content)?;

                if let Ok(id) = id_str.trim().parse::<i64>() {
                    update_memo(conn, id, content.trim())?;
                } else {
                    println!("Invalid ID format");
                }
            }
            "d" => {
                print!("Enter memo ID: ");
                io::stdout().flush()?;
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str)?;
                if let Ok(id) = id_str.trim().parse::<i64>() {
                    delete_memo(conn, id)?;
                } else {
                    println!("Invalid ID format");
                }
            }
            "q" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                if let Ok(id) = choice.parse::<i64>() {
                    view_memo(conn, id)?;
                } else {
                    println!("Invalid option. Please try again.");
                }
            }
        }
    }

    Ok(())
}
