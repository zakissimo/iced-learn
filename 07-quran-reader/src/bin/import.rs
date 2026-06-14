use anyhow::{Context, Result};
use regex::Regex;
use rusqlite::Connection;
use rusqlite::params;

const DUMP_FILE: &str = "assets/data/quran-uthmani.sql";

fn main() -> Result<()> {
    let mut conn = Connection::open("assets/data/quran.db").context("opening db connection")?;

    // surahs first (verses references it); drop verses first for the same reason.
    conn.execute_batch(
        "DROP TABLE IF EXISTS verses;
         DROP TABLE IF EXISTS surahs;
         CREATE TABLE surahs (
             surah   INTEGER PRIMARY KEY,
             basmala TEXT            -- NULL for surah 1 (basmala is verse 1) and surah 9 (none)
         );
         CREATE TABLE verses (
             id    INTEGER PRIMARY KEY,
             surah INTEGER NOT NULL REFERENCES surahs(surah),
             ayah  INTEGER NOT NULL,
             body  TEXT    NOT NULL
         );",
    )?;

    let re = Regex::new(r"\((\d+),\s*(\d+),\s*(\d+),\s*'(.*)'\)")?;
    let data = std::fs::read_to_string(DUMP_FILE)?;

    let tx = conn.transaction()?;
    let mut verses = 0usize;
    {
        let mut surah_stmt = tx.prepare("INSERT INTO surahs (surah, basmala) VALUES (?1, ?2);")?;
        let mut verse_stmt =
            tx.prepare("INSERT INTO verses (surah, ayah, body) VALUES (?1, ?2, ?3);")?;

        for line in data.lines() {
            if !line.starts_with('(') {
                continue;
            }
            let Some(cap) = re.captures(line) else {
                continue;
            };
            let (_, [_idx, surah, ayah, full_body]) = cap.extract();
            let surah: i64 = surah.parse()?;
            let ayah: i64 = ayah.parse()?;

            // The basmala is a per-surah opening. For every surah except 1
            // (where it IS verse 1) and 9 (which has none), it is prepended to
            // verse 1's text. Split it off the first verse and store it on the
            // surah row instead; keep the rest as the verse body.
            let mut body = full_body;
            if ayah == 1 {
                let basmala: Option<String> = if surah == 1 || surah == 9 {
                    None
                } else {
                    // first 4 space-separated words are the basmala; the 5th
                    // piece is the rest of the verse (untouched). Word-position
                    // split is robust to the shadda variants in surahs 95 & 97.
                    let parts: Vec<&str> = full_body.splitn(5, ' ').collect();
                    body = parts[4];
                    Some(parts[..4].join(" "))
                };
                surah_stmt.execute(params![surah, basmala])?;
            }

            verse_stmt.execute(params![surah, ayah, body])?;
            verses += 1;
        }
    }
    tx.commit()?;

    eprintln!("imported {verses} verses");
    Ok(())
}
