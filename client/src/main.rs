use iced::{
    executor,
    widget::{button, container, row, text},
    Application, Command, Settings, Theme,
};

#[derive(Debug, Clone)]
enum Message {}

struct App {}

impl Application for App {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (App, iced::Command<Self::Message>) {
        (App {}, Command::none())
    }

    fn title(&self) -> String {
        String::from("muChat")
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let top_bar = row![
            button("Server"),
            button("Self")
        ].spacing(10);

        container(top_bar).padding(10).into()
    }
}

fn main() -> iced::Result {
    App::run(Settings::default())
}

