use iced::{
    alignment::{Horizontal, Vertical},
    executor,
    widget::{column, container, pick_list, row, text, Button, Row, Text, TextInput}, Application, Command, Length, Settings, Theme,
};
use iced_aw::Card;

#[derive(Debug, Clone)]
enum Message {
    ServerDropDownAction(String),
    SelfDropDownAction(String),
    CloseModal,
    CancelButtonPressed,
    OkButtonPressed,
    ServerAddrInpuChanged(String),
}

struct App {
    show_modal: bool,
}

impl Application for App {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (App, iced::Command<Self::Message>) {
        (App { show_modal: false }, Command::none())
    }

    fn title(&self) -> String {
        String::from("muChat")
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::CloseModal => self.show_modal = false,
            Message::OkButtonPressed => self.show_modal = false,
            Message::CancelButtonPressed => self.show_modal = false,
            Message::ServerDropDownAction(action) => {
                if action == "Connect".to_string() {
                    self.show_modal = true;
                } else if action == "Disconnect".to_string() {
                    self.show_modal = false;
                }
            }
            Message::SelfDropDownAction(_action) => {}
            Message::ServerAddrInpuChanged(_text) => {}
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Message> {
        println!("Open modal closed ");
        let server_pk_list = {
            let tags = vec!["Connect".to_string(), "Disconnect".to_string()];
            let tag_el = pick_list(tags, None::<String>, Message::ServerDropDownAction)
                .placeholder("Server");
            tag_el
        };
        let self_pk_list = {
            let tags = vec!["Mute Self".to_string(), "Defean Self".to_string()];
            let tag_el =
                pick_list(tags, None::<String>, Message::SelfDropDownAction).placeholder("Self");
            tag_el
        };

        let top_bar = row![server_pk_list, self_pk_list].height(50).spacing(10);

        if !self.show_modal {
            container(top_bar).padding(10).into()
        } else {
                let server_conn_modal = container(
                    Card::new(
                        Text::new("Server "),
                        container(
                            column![
                            row![
                            text("Server Address"),
                            TextInput::new("Server Address", "Some val")
                            .on_input(Message::ServerAddrInpuChanged)
                            ]
                            .spacing(20),
                            row![
                            text("Server Password"),
                            TextInput::new("Password", "Some val")
                            .on_input(Message::ServerAddrInpuChanged)
                            ]
                            .spacing(20),
                            row![
                            text("Server Nickname"),
                            TextInput::new("nickname", "Some val")
                            .on_input(Message::ServerAddrInpuChanged)
                            ]
                            .spacing(20)
                            ]
                            .spacing(30),
                            ),
                            )
                                .foot(
                                    Row::new()
                                    .spacing(10)
                                    .padding(5)
                                    .width(Length::Fill)
                                    .push(
                                        Button::new(
                                            Text::new("Cancel").horizontal_alignment(Horizontal::Center),
                                        )
                                        .width(Length::Fill)
                                        .on_press(Message::CancelButtonPressed),
                                    )
                                    .push(
                                        Button::new(Text::new("Ok").horizontal_alignment(Horizontal::Center))
                                        .width(Length::Fill)
                                        .on_press(Message::OkButtonPressed),
                                    ),
                                )
                                .max_width(400.0)
                                .max_height(400.0)
                                .on_close(Message::CloseModal),
                                )
                                    .height(Length::Fill)
                                    .align_x(Horizontal::Center)
                                    .align_y(Vertical::Center);
            container(column![top_bar, server_conn_modal]).into()
        }
    }
}

fn main() -> iced::Result {
    App::run(Settings::default())
}
