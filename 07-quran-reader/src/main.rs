use iced::widget::{column, container, text};
use iced::{Center, Element, Fill};

#[derive(Default)]
struct App;

#[derive(Debug, Clone)]
enum Message {}

impl App {
    fn update(&mut self, _: Message) {}

    fn view(&self) -> Element<'_, Message> {
        container(
            column![
                text("Hello, iced!").size(48),
                text("Edit src/main.rs to begin.").size(16),
            ]
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
        .run()
}
