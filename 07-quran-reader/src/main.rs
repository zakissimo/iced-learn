use anyhow::Result;
use iced::widget::{button, column, container, text};
use iced::{Center, Element, Fill};
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

impl App {
    fn load_verses(&mut self) -> Result<()> {
        let db = Connection::open(DB)?;
        let mut stmt = db.prepare("SELECT surah, ayah, body FROM verses ORDER BY surah, ayah")?;
        self.verses = stmt
            .query_map([], |row| {
                let surah = row.get("surah")?;
                let ayah = row.get("ayah")?;
                let body = row.get("body")?;
                Ok(Verse { surah, ayah, body })
            })?
            .collect::<rusqlite::Result<Vec<Verse>>>()?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
enum Message {
    Load,
}

impl App {
    fn update(&mut self, msg: Message) {
        match msg {
            Message::Load => {
                self.load_verses().unwrap();
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let load = button("Load").on_press_maybe((self.verses.is_empty()).then_some(Message::Load));
        let msg = format!("{} verses loaded!", self.verses.len());

        container(
            column![text(msg).size(16), load]
                .spacing(16)
                .align_x(Center),
        )
        .center(Fill)
        .into()
    }
}

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("iced learning playground")
        .window_size((480.0, 640.0))
        .resizable(false)
        .run()
}
