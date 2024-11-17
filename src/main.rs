use chrono::Local;
use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::time::Duration;
use std::{env, thread};
use user_idle::UserIdle;

#[derive(Debug)]
struct Data {
    id: i32,
    date: String,
    duration: u64,
}

fn main() -> Result<()> {
    let threshold = 60;
    let mut idle_duration: u64 = 0;
    let mut is_idle = false;

    let db_file = env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            println!("Could not determine the home directory from $HOME.");
            std::process::exit(1);
        })
        .join("rust-timer.sqlite");

    println!(
        "[{}]: Using database file at {}.",
        Local::now().format("%y-%m-%d %H:%M"),
        db_file.display()
    );

    let conn = Connection::open(&db_file).unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS data (
            id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            date DATETIME NOT NULL UNIQUE,
            duration BIGINT NOT NULL
        )",
        [],
    )?;

    loop {
        let idle = UserIdle::get_time().unwrap();
        let idle_seconds = idle.as_seconds();

        if is_idle && idle_seconds <= threshold {
            let mut stmt = conn.prepare("SELECT MAX(id) FROM data")?;
            let max_id: Option<i32> = stmt.query_row([], |row| row.get(0)).unwrap_or(None);

            let new_id = max_id.unwrap_or(0) + 1;

            let prepared = Data {
                id: new_id,
                date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                duration: idle_duration,
            };

            conn.execute(
                "INSERT INTO data (id, date, duration) VALUES (?1, ?2, ?3)",
                (prepared.id, prepared.date, prepared.duration),
            )
            .unwrap();

            println!(
                "[{}]: Active again. Idle duration: {} seconds inserted.",
                Local::now().format("%y-%m-%d %H:%M"),
                idle_duration
            );

            is_idle = false;
        }

        if idle_seconds > threshold {
            if !is_idle {
                println!(
                    "[{}]: System is idle...",
                    Local::now().format("%y-%m-%d %H:%M")
                );
            }

            is_idle = true;
            idle_duration = idle_seconds - threshold;
        }

        thread::sleep(Duration::from_secs(1));
    }
}
