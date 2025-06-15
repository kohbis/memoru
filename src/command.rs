use crate::Result;
use chrono::Local;
use rusqlite::Connection;

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

pub fn delete_memo(conn: &Connection, id: i64) -> Result<()> {
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

    loop {
        println!("\n=== Memoru Interactive Mode ===");
        println!("1. Add new memo");
        println!("2. List all memos");
        println!("3. View a memo");
        println!("4. Update a memo");
        println!("5. Delete a memo");
        println!("6. Exit");
        print!("\nSelect an option (1-6): ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim();

        match choice {
            "1" => {
                print!("Enter memo content: ");
                io::stdout().flush()?;
                let mut content = String::new();
                io::stdin().read_line(&mut content)?;
                add_memo(conn, content.trim())?;
            }
            "2" => {
                list_memos(conn)?;
            }
            "3" => {
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
            "4" => {
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
            "5" => {
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
            "6" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid option. Please try again.");
            }
        }
    }

    Ok(())
}
