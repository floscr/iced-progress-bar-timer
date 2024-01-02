use std::env;
use std::time::Instant;

use iced::executor;
use iced::mouse;
use iced::widget::canvas::{self, Cache, Canvas, Geometry, Path};
use iced::{
    Application, Command, Element, Length, Point, Rectangle, Renderer, Size, Subscription, Theme,
};

pub fn main() -> iced::Result {
    Arc::run(iced::Settings {
        antialiasing: true,
        window: iced::window::Settings {
            size: (250, 250),
            resizable: true,
            decorations: false,
            transparent: true,
            ..Default::default()
        },
        ..Default::default()
    })
}

struct Arc {
    start: Instant,
    duration: f32,
    cache: Cache,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick,
}

impl Application for Arc {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let duration = match env::args().nth(1) {
            Some(arg) => arg.parse().unwrap_or(60.0),
            None => 60.0,
        };
        (
            Arc {
                start: Instant::now(),
                duration,
                cache: Cache::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Focus timer")
    }

    fn update(&mut self, _: Message) -> Command<Message> {
        self.cache.clear();

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(10)).map(|_| Message::Tick)
    }
}

impl<Message> canvas::Program<Message> for Arc {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            let palette = theme.palette();

            let elapsed_seconds = &self.start.elapsed().as_secs_f32();
            let progress_percentage = (elapsed_seconds / &self.duration).min(1.0);
            let current_width = frame.width() * progress_percentage;

            let rect = Path::new(|b| {
                b.rectangle(
                    Point {
                        x: bounds.x,
                        y: bounds.y,
                    },
                    Size {
                        width: current_width,
                        height: frame.height(),
                    },
                )
            });

            frame.fill(&rect, palette.text);
        });

        vec![geometry]
    }
}
