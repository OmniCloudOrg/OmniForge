use rusqlite::{params, Connection, Result};

fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS file_types (
                  extension TEXT PRIMARY KEY,
                  container TEXT NOT NULL
                  )",
        [],
    )?;
    Ok(())
}

fn insert_file_type(conn: &Connection, extension: &str, container: &str) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO file_types (extension, container) VALUES (?1, ?2)",
        params![extension, container],
    )?;
    Ok(())
}

fn get_container_by_extension(conn: &Connection, extension: &str) -> Result<String> {
    conn.query_row(
        "SELECT container FROM file_types WHERE extension = ?1",
        params![extension],
        |row| row.get(0),
    )
}

fn main() -> Result<()> {
    let conn = Connection::open("file_types.db")?;
    create_table(&conn)?;

    let file_types = vec![
        ("rs", "ghcr.io/devcontainers/features/rust:latest"),
        ("py", "ghcr.io/devcontainers/features/python:latest"),
        ("java", "ghcr.io/devcontainers/features/java:latest"),
        ("js", "ghcr.io/devcontainers/features/node:latest"),
        ("ts", "ghcr.io/devcontainers/features/node:latest"),
        // Add other file types here...
    ];

    for (extension, container) in file_types {
        insert_file_type(&conn, extension, container)?;
    }

    let extension = "rs";
    match get_container_by_extension(&conn, extension) {
        Ok(container) => println!("Container for {}: {}", extension, container),
        Err(_) => println!("No container found for {}", extension),
    }

    Ok(())
}
