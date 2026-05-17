use iced::Alignment::Center;
use iced::widget::{container, row, text_input};
use iced::{Element, Fill};

#[derive(Default)]
struct Temperature {
    celsius: f64,
}

#[derive(Default)]
struct State {
    temperature: Temperature,
    celsius_input: String,
    fahrenheit_input: String,
}

#[derive(Debug, Clone)]
enum Message {
    CelsiusChanged(String),
    FahrenheitChanged(String),
}

impl Temperature {
    pub fn try_from_celsius_string(c: &str) -> Option<Self> {
        Some(Self {
            celsius: c.parse::<f64>().ok()?,
        })
    }

    pub fn try_from_fahrenheit_string(f: &str) -> Option<Self> {
        Some(Self {
            celsius: (f.parse::<f64>().ok()? - 32.0) * 5.0 / 9.0,
        })
    }

    pub fn as_celsius_string(&self) -> String {
        format!("{:.2}", self.celsius)
    }

    pub fn as_fahrenheit_string(&self) -> String {
        let f = self.celsius * 9.0 / 5.0 + 32.0;
        format!("{:.2}", f)
    }
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::CelsiusChanged(s) => {
                if let Some(c) = Temperature::try_from_celsius_string(&s) {
                    self.temperature = c;
                    self.fahrenheit_input = self.temperature.as_fahrenheit_string();
                }
                self.celsius_input = s;
            }
            Message::FahrenheitChanged(s) => {
                if let Some(f) = Temperature::try_from_fahrenheit_string(&s) {
                    self.temperature = f;
                    self.celsius_input = self.temperature.as_celsius_string();
                }
                self.fahrenheit_input = s;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let c = text_input("°C", &self.celsius_input).on_input(Message::CelsiusChanged);
        let f = text_input("°F", &self.fahrenheit_input).on_input(Message::FahrenheitChanged);

        container(row![c, f].spacing(12).align_y(Center))
            .width(Fill)
            .max_width(700)
            .center(Fill)
            .into()
    }
}

fn main() -> iced::Result {
    iced::application(State::default, State::update, State::view)
        .title("iced learning playground")
        .run()
}
