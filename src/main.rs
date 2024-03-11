use user_idle::UserIdle;
use rusqlite::{params, Connection, Result};
use chrono::Local;

#[derive(Debug)]
struct Data {
    id: i32,
    date: String,
    duration: u64,
}

fn main() -> Result<()> {
    let daemonized = true;
    let threshold = 5;
    let mut idle_duration: u64 = 0;
    let conn = Connection::open_in_memory()?;

    while daemonized {
        let idle = UserIdle::get_time().unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS data (id INTEGER PRIMARY KEY, date DATE NOT NULL, duration BIGINT NOT NULL)",
            [],
        )?;

        if idle.as_seconds() > threshold {
            idle_duration = idle.as_seconds() - threshold;

            let prepared = Data {
                id: 0,
                date: Local::now().format("%Y-%m-%d").to_string(),
                duration: idle_duration,
            };

            let _ = conn.execute("
                INSERT OR REPLACE INTO data (id, date, duration) values (
                    (SELECT id FROM data WHERE date = ?1),
                    (SELECT date FROM data WHERE date = ?1),
                    ?2
                )
            ", params![prepared.date, prepared.duration]);

            print!("\r{}", idle_duration);
        } else {
            print!("\r{}", idle_duration);
        }
    }

    Ok(())
}
