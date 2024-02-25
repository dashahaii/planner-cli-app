use rusqlite::{Connection, Result, params};
use chrono::{DateTime, Local};

// Function to open a connection to the database
pub fn open_db(db_file: &str) -> Result<Connection> {
    Connection::open(db_file)
}

// CREATE TASKS
// ================================
// id - integer (pk) (required)
// name - string (required)
// time_due - string (NOT required)
// priority - string (NOT required)
// note - string (NOT required)
// time_added - string (required)
// ================================
pub fn create_table(connection: &Connection) -> Result<()> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            time_due TEXT,
            priority INTEGER,
            note TEXT,
            time_added TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

// INSERT TASKS
pub fn insert_tasks(
    connection: &Connection, 
    name: &str, 
    time_due: &str,
    priority: i32,
    note: &str) -> Result<()> {
    // caputure time of execution
    // execute insertion command w/ current time & above params
    let time_added: DateTime<Local> = Local::now();
    connection.execute(
        "INSERT INTO tasks (name, time_due, priority, note, time_added) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![name, time_due, priority, note, time_added.to_string()],
    )?;
    Ok(())
}

// QUERY TASKS (BASIC)
pub fn query_tasks(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT name, time_due, priority, note, time_added FROM tasks")?;
    let task_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, i32>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?
        ))
    })?;

    for task in task_iter {
        println!("Found task: {:?}", task?);
    }

    Ok(())
}
