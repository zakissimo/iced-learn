use iced::widget::{button, checkbox, column, container, row, space, text, text_input};
use iced::{Center, Element, Fill, Task};
use serde::{Deserialize, Serialize};
use std::{mem, result};
use tokio::fs;

type Result<T> = result::Result<T, String>;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
enum ItemState {
    #[default]
    Idle,
    Editing(String),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct Item {
    text: String,
    done: bool,
    state: ItemState,
}

impl Item {
    fn toggle(&mut self) {
        self.done = !self.done;
    }

    fn view_label(&self, idx: usize) -> Element<'_, Message> {
        match &self.state {
            ItemState::Idle => text(&self.text).into(),
            ItemState::Editing(draft) => text_input(&self.text, draft)
                .on_input(move |new| Message::Edit(idx, Edit::Update(new)))
                .on_submit(Message::Edit(idx, Edit::Commit))
                .into(),
        }
    }

    fn update(&mut self, msg: Edit) {
        match msg {
            Edit::Start => {
                self.state = ItemState::Editing(self.text.clone());
            }
            Edit::Update(s) => {
                if matches!(self.state, ItemState::Editing(_)) {
                    self.state = ItemState::Editing(s);
                }
            }
            Edit::Commit => {
                if let ItemState::Editing(draft) = std::mem::take(&mut self.state) {
                    self.text = draft;
                }
            }
        }
    }
}

const SAVED_TODOS: &str = "/tmp/todos.json";

async fn load_todos() -> Result<Vec<Item>> {
    let bytes = fs::read(SAVED_TODOS).await.map_err(|e| format!("{e}"))?;

    serde_json::from_slice(&bytes).map_err(|e| format!("{e}"))
}

async fn save_todos(todos: Vec<Item>) -> Result<()> {
    let bytes = serde_json::to_string(&todos).map_err(|e| format!("{e}"))?;

    fs::write(SAVED_TODOS, bytes)
        .await
        .map_err(|e| format!("{e}"))
}

#[derive(Default)]
struct State {
    input: String,
    todos: Vec<Item>,
}

#[derive(Debug, Clone)]
enum Edit {
    Start,
    Update(String),
    Commit,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    Add,
    Toggle(usize),
    Delete(usize),
    Edit(usize, Edit),
    Load,
    Loaded(Result<Vec<Item>>),
    Save,
    Saved(Result<()>),
}

impl State {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::InputChanged(s) => {
                self.input = s;
                Task::none()
            }
            Message::Add => {
                let mut item = Item::default();
                mem::swap(&mut item.text, &mut self.input);
                self.todos.push(item);
                Task::none()
            }
            Message::Toggle(i) => {
                if let Some(item) = self.todos.get_mut(i) {
                    item.toggle()
                }
                Task::none()
            }
            Message::Delete(i) => {
                self.todos.remove(i);
                Task::none()
            }
            Message::Edit(i, msg) => {
                if let Some(item) = self.todos.get_mut(i) {
                    item.update(msg);
                }
                Task::none()
            }
            Message::Load => Task::perform(load_todos(), Message::Loaded),
            Message::Loaded(items) => {
                match items {
                    Ok(items) => self.todos = items,
                    Err(e) => eprintln!("couldn't load: {e}"),
                }
                Task::none()
            }
            Message::Save => Task::perform(save_todos(self.todos.clone()), Message::Saved),
            Message::Saved(status) => {
                match status {
                    Ok(_) => {}
                    Err(e) => eprintln!("couldn't save: {e}"),
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let load = button("Load").on_press(Message::Load);
        let save = button("Save").on_press_maybe((!self.todos.is_empty()).then_some(Message::Save));

        let header = row![load, save].spacing(3).padding(7);

        let input = text_input("New todo", &self.input)
            .on_input(Message::InputChanged)
            .on_submit(Message::Add);

        let add =
            button("Add").on_press_maybe((!self.input.trim().is_empty()).then_some(Message::Add));

        let footer = row![input, add].align_y(Center);

        let items = self.todos.iter().enumerate().map(|(i, todo)| {
            row![
                checkbox(todo.done).on_toggle(move |_| Message::Toggle(i)),
                todo.view_label(i),
                space::horizontal(),
                button("✏").on_press_maybe(
                    matches!(todo.state, ItemState::Idle).then_some(Message::Edit(i, Edit::Start))
                ),
                button("×").on_press(Message::Delete(i)),
            ]
            .spacing(10)
            .align_y(Center)
            .into()
        });

        let panel = container(column(items).spacing(8).push(footer))
            .padding(14)
            .width(Fill)
            .height(Fill)
            .style(container::bordered_box);

        let frame = container(column![header, panel])
            .padding(14)
            .width(Fill)
            .height(Fill)
            .style(container::rounded_box);

        frame.into()
    }
}

fn main() -> iced::Result {
    iced::application(State::default, State::update, State::view)
        .title("iced learning playground")
        .window_size((480.0, 640.0))
        .resizable(false)
        .run()
}
