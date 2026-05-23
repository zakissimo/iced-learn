use anyhow::{Context, Result};
use iced::widget::image::Handle;
use iced::widget::{button, column, container, image, text};
use iced::{Center, Element, Fill, Task};
use rand::Rng;
use serde::Deserialize;

const ENDPOINT: &str = "https://pokeapi.co/api/v2/pokemon";

type MsgResult<T> = std::result::Result<T, String>;

#[derive(Default, Debug, Deserialize, Clone)]
struct Sprites {
    front_default: String,
}

#[derive(Default, Debug, Deserialize, Clone)]
struct Pokemon {
    name: String,
    sprites: Sprites,
}

async fn load_pokemon(id: i32) -> Result<(Pokemon, Handle)> {
    let pokemon = reqwest::get(format!("{}/{}", ENDPOINT, id))
        .await
        .context("fetching pokemon data")?
        .json::<Pokemon>()
        .await
        .context("deserializing pokemon data")?;

    let raw_image = reqwest::get(&pokemon.sprites.front_default)
        .await
        .context("fetching pokemon image")?
        .bytes()
        .await
        .context("deserializing pokemon image to bytes")?;

    Ok((pokemon, Handle::from_bytes(raw_image)))
}

#[derive(Default)]
struct App {
    pokemon: Option<Pokemon>,
    handle: Option<Handle>,
    loading: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Load,
    Loaded(MsgResult<(Pokemon, Handle)>),
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Load => {
                self.loading = true;

                let mut rng = rand::thread_rng();
                let id = rng.gen_range(1..=1025);

                Task::perform(load_pokemon(id), |r| {
                    Message::Loaded(r.map_err(|e| format!("{e:#}")))
                })
            }
            Message::Loaded(handle) => {
                match handle {
                    Ok((pokemon, handle)) => {
                        self.handle = Some(handle);
                        self.pokemon = Some(pokemon);
                    }
                    Err(e) => {
                        eprintln!("couldn't fetch pokemon {e}");
                    }
                };

                self.loading = false;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let placeholder = |label: &'static str| -> Element<'_, Message> {
            container(text(label))
                .width(96)
                .height(96)
                .style(container::bordered_box)
                .align_x(Center)
                .align_y(Center)
                .into()
        };

        let sprite: Element<'_, Message> = if self.loading {
            placeholder("Loading...")
        } else if let Some(handle) = &self.handle {
            container(image(handle))
                .width(96)
                .height(96)
                .style(container::bordered_box)
                .align_x(Center)
                .align_y(Center)
                .into()
        } else {
            placeholder("Click Load")
        };

        let name = self.pokemon.as_ref().map(|p| p.name.as_str()).unwrap_or("");

        let load = button("Load").on_press_maybe((!self.loading).then_some(Message::Load));

        let content = column![text(name), sprite, load]
            .spacing(16)
            .align_x(Center);

        container(content).center(Fill).into()
    }
}

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("iced learning playground")
        .window_size((480.0, 640.0))
        .resizable(false)
        .run()
}
