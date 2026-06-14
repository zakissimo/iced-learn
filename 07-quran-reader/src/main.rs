use iced::Alignment::Center;
use iced::alignment::Horizontal::Right;
use iced::widget::{Column, column, container, pick_list, scrollable, text};
use iced::{Element, Fill};
use rusqlite::Connection;

const DB: &str = "assets/data/quran.db";

#[derive(Default, Clone)]
pub struct Surah {
    pub nb: i64,
    pub basmala: Option<String>,
}

#[derive(Default, Clone)]
pub struct Verse {
    pub surah: i64,
    pub ayah: i64,
    pub body: String,
}

#[derive(Default)]
struct App {
    current_surah: Option<i64>,
    verses: Vec<Verse>,
    surahs: Vec<Surah>,
}

fn load_surahs() -> rusqlite::Result<Vec<Surah>> {
    let db = Connection::open(DB)?;
    let mut stmt = db.prepare("SELECT surah, basmala FROM surahs ORDER BY surah")?;
    stmt.query_map([], |row| {
        let nb = row.get("surah")?;
        let basmala = row.get("basmala")?;
        Ok(Surah { nb, basmala })
    })?
    .collect::<rusqlite::Result<Vec<Surah>>>()
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
            current_surah: Some(1),
            verses: load_verses().expect("run cargo run --bin import first"),
            surahs: load_surahs().expect("run cargo run --bin import first"),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    SurahSelected(i64),
}

impl App {
    fn update(&mut self, msg: Message) {
        match msg {
            Message::SurahSelected(surah) => self.current_surah = Some(surah),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let current_surah = self.current_surah.unwrap_or(1);

        // scrollable content: optional basmala heading, then the verses
        let mut content = Column::new().spacing(12).width(Fill).padding(20);

        if let Some(surah) = self.surahs.iter().find(|s| s.nb == current_surah)
            && let Some(basmala) = &surah.basmala
        {
            content = content.push(
                container(text(basmala).size(28))
                    .width(Fill)
                    .align_x(Center),
            );
        }

        for v in self.verses.iter().filter(|v| v.surah == current_surah) {
            content = content.push(
                container(text(&v.body).size(28))
                    .width(Fill)
                    .align_x(Right),
            );
        }

        let page = scrollable(content).width(Fill);

        let surah_list: Vec<i64> = self.surahs.iter().map(|s| s.nb).collect();
        let pl = pick_list(surah_list, self.current_surah, Message::SurahSelected)
            .placeholder("Select surah...")
            .width(Fill);

        column![pl, page].into()
    }
}

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("iced learning playground")
        .window_size((480.0, 640.0))
        .resizable(false)
        .run()
}
