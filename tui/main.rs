use std::{env, path::PathBuf};

use chrono::Local;
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::{Color, Style},
    widgets::{BarChart, Block, Borders},
};
use rusqlite::{Connection, Result};

fn fetch_durations_by_day(conn: &Connection) -> Vec<(String, u64)> {
    let mut stmt = conn
        .prepare(
            "SELECT 
                strftime('%d', date) as day,
                SUM(duration) AS idle_duration
            FROM data
            GROUP BY day
            ORDER BY day;",
        )
        .unwrap();

    let results = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0).unwrap(),
                row.get::<_, u64>(1).unwrap(),
            ))
        })
        .unwrap();

    results.map(|r| r.unwrap()).collect()
}

fn render_barchart(frame: &mut ratatui::Frame, data: &[(String, u64)]) {
    let mut labels: Vec<String> = Vec::new();

    for (date, _) in data {
        labels.push(format!("{}-I", date));
    }

    let bar_data: Vec<(&str, u64)> = data
        .iter()
        .flat_map(|(_, idle)| vec![(labels[labels.len() - 1].as_str(), *idle)])
        .collect();

    let barchart = BarChart::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Productivity by Day"),
        )
        .data(&bar_data)
        .bar_width(3)
        .bar_gap(1)
        .bar_style(Style::default().fg(Color::Green))
        .value_style(Style::default().fg(Color::Yellow).bg(Color::Black));

    frame.render_widget(barchart, frame.area());
}

fn main() -> Result<()> {
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

    let data = fetch_durations_by_day(&conn);

    let mut terminal = ratatui::init();
    terminal.clear().unwrap();
    terminal
        .draw(|frame| render_barchart(frame, &data))
        .unwrap();

    loop {
        if let event::Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    ratatui::restore();
    Ok(())
}
