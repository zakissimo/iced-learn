use iced::alignment::Horizontal::Right;
use iced::widget::{column, container, scrollable, text};
use iced::{Element, Fill};
use rusqlite::Connection;

const DB: &str = "assets/data/quran.db";

#[derive(Default, Clone)]
pub struct Verse {
    pub surah: i64,
    pub ayah: i64,
    pub body: String,
}

#[derive(Default)]
struct App {
    verses: Vec<Verse>,
}

fn load_verses() -> rusqlite::Result<Vec<Verse>> {
    let db = Connection::open(DB)?;
    let mut stmt = db.prepare("SELECT surah, ayah, body FROM verses ORDER BY surah, ayah")?;
    stmt.query_map([], |row| {
        let surah = row.get("surah")?;
        let ayah = row.get("ayah")?;
        let body = row.get("body")?;
        Ok(Verse { surah, ayah, body })
    })?
    .collect::<rusqlite::Result<Vec<Verse>>>()
}

impl App {
    fn new() -> Self {
        App {
            verses: load_verses().expect("run cargo run --bin import first"),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {}

impl App {
    fn update(&mut self, msg: Message) {
        match msg {}
    }

    fn view(&self) -> Element<'_, Message> {
        let fatiha = self.verses.iter().filter(|v| v.surah == 1).map(|v| {
            container(text(&v.body).size(28))
                .width(Fill)
                .align_x(Right)
                .into()
        });

        scrollable(column(fatiha).spacing(12).width(Fill).padding(20))
            .width(Fill)
            .into()
    }
}

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("iced learning playground")
        .window_size((480.0, 640.0))
        .resizable(false)
        .run()
}
