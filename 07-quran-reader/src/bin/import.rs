use anyhow::{Context, Result};
use regex::Regex;
use rusqlite::Connection;
use rusqlite::params;

const DUMP_FILE: &str = "assets/data/quran-uthmani.sql";

fn main() -> Result<()> {
    let mut conn = Connection::open("assets/data/quran.db").context("opening db connection")?;
    conn.execute("DROP TABLE IF EXISTS verses;", ())?;
    conn.execute(
        "CREATE TABLE verses (
            id    INTEGER PRIMARY KEY,
            sura  INTEGER NOT NULL,
            ayah  INTEGER NOT NULL,
            body  TEXT    NOT NULL
        );",
        (),
    )?;

    let tx = conn.transaction()?;

    let mut stmt = tx.prepare("INSERT INTO verses (sura, ayah, body) VALUES (?1, ?2, ?3);")?;
    let re = Regex::new(r"\((\d+),\s*(\d+),\s*(\d+),\s*'(.*)'\)").unwrap();
    let data = std::fs::read_to_string(DUMP_FILE)?;

    for line in data.lines() {
        if !line.starts_with('(') {
            continue;
        }

        if let Some(cap) = re.captures(line) {
            let (_, [_idx, sura, ayah, body]) = cap.extract();
            let sura: i64 = sura.parse()?;
            let ayah: i64 = ayah.parse()?;
            stmt.execute(params![sura, ayah, body])?;
        }
    }

    drop(stmt);

    tx.commit()?;

    Ok(())
}
