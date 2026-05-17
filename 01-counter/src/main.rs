use iced::Alignment::Center;
use iced::widget::{button, column, container, row, text};
use iced::{Element, Fill};

#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    Reset,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
            Message::Reset => {
                self.value = 0;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let increment = button("+").on_press_maybe((self.value < 10).then_some(Message::Increment));
        let decrement = button("-").on_press_maybe((self.value > 0).then_some(Message::Decrement));
        let reset = button("reset").on_press(Message::Reset);

        let counter = text(self.value).size(48);

        container(
            column![
                row![decrement, counter, increment]
                    .spacing(12)
                    .align_y(Center),
                reset
            ]
            .align_x(Center),
        )
        .center(Fill)
        .into()
    }
}

fn main() -> iced::Result {
    iced::application(Counter::default, Counter::update, Counter::view)
        .title("iced learning playground")
        .run()
}
