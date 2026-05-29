use iced::time::{Duration, Instant};

use iced::widget::{button, column, container, row, text};
use iced::{Center, Element, Fill, Subscription, time};

#[derive(Default)]
struct App {
    running: bool,
    elapsed: Duration,
    last_tick: Option<Instant>,
}

#[derive(Debug, Clone)]
enum Message {
    Start,
    Tick(Instant),
    Pause,
    Reset,
}

impl App {
    fn dirty(&self) -> bool {
        self.elapsed != Duration::ZERO
    }

    fn pause(&mut self) {
        self.last_tick = None;
        self.running = false;
    }

    fn reset(&mut self) {
        self.last_tick = None;
        self.running = false;
        self.elapsed = Duration::ZERO;
    }

    fn update(&mut self, msg: Message) {
        match msg {
            Message::Start => self.running = true,
            Message::Tick(now) => {
                let d = match self.last_tick {
                    Some(last) => now - last,
                    None => Duration::ZERO,
                };
                self.elapsed += d;
                self.last_tick = Some(now);
            }
            Message::Pause => self.pause(),
            Message::Reset => self.reset(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let start = button("Start").on_press_maybe((!self.running).then_some(Message::Start));
        let pause = button("Pause").on_press_maybe((self.running).then_some(Message::Pause));
        let reset = button("Reset").on_press_maybe(self.dirty().then_some(Message::Reset));

        let btn_row = row![start, pause, reset].spacing(8);

        let display = format!("{:?}", self.elapsed);

        container(
            column![text(display).size(48), btn_row,]
                .spacing(16)
                .align_x(Center),
        )
        .center(Fill)
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.running {
            return time::every(Duration::from_millis(100)).map(Message::Tick);
        }

        Subscription::none()
    }
}

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .subscription(App::subscription)
        .title("iced learning playground")
        .window_size((480.0, 640.0))
        .resizable(false)
        .run()
}
