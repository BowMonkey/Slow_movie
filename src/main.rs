use iced::alignment::{self, Alignment};
use iced::theme::{self, Theme};
use iced::widget::{button, column, container, horizontal_space, radio, row, text, text_input};
use iced::{window, Color, Element, Length, Sandbox, Settings};

pub fn main() -> iced::Result {
    //Styling::run(Settings::default())
    Styling::run(Settings {
        window: window::Settings {
            size: (600, 400),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Default)]
struct Styling {
    theme: Theme,
    movie_path: String,
    time_str:String,
    time_interval: i32,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    SetTime(i32),
    TimeInputChanged(String),
    ButtonSelect,
    ButtonOk,
    ButtonExit,
}

impl Sandbox for Styling {
    type Message = Message;

    fn new() -> Self {
        Styling {
            theme: Theme::Dark,
            ..Self::default()
        }
    }

    fn title(&self) -> String {
        String::from("简帧")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => self.movie_path = value,
            Message::ButtonSelect => {
                todo!();
            }
            Message::SetTime(value) => self.time_interval = value,
            Message::TimeInputChanged(value) => self.movie_path = value,
            Message::ButtonOk => {
                todo!();
            }
            Message::ButtonExit => {
                todo!();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let title = text("Slow Movie")
            .width(Length::Fill)
            .size(100)
            .style(Color::from([0.5, 0.5, 0.5]))
            .horizontal_alignment(alignment::Horizontal::Center);
        let filepath_input = text_input("file path...", &self.movie_path, Message::InputChanged)
            .padding(10)
            .size(20);

        let select_file_button = button("Choose a Movie")
            .padding(10)
            .on_press(Message::ButtonSelect);

        let time_label = text("Time interval:").size(30);

        let time_input = text_input("Time of interval ... ", &self.time_str, Message::TimeInputChanged).padding(10).size(20);
        let second_radio = radio("Seconds", 1, Some(1), Message::SetTime);
        let min_radio = radio("Minutes", 60, Some(60), Message::SetTime);
        let hour_radio = radio("Hours", 60 * 60, Some(60 * 60), Message::SetTime);

        let ok_button = button("confirm").padding(10).on_press(Message::ButtonOk);
        let exit_button = button("exit").padding(10).on_press(Message::ButtonExit);

        let content = column![
            title,
            row![select_file_button, filepath_input].spacing(10),
            row![time_label, time_input, second_radio, min_radio, hour_radio].spacing(10),
            //row![second_radio, min_radio, hour_radio].spacing(10),
            row![horizontal_space(Length::Fill), ok_button, exit_button].spacing(10),    
        ]
        .spacing(20)
        .padding(20)
        .max_width(600);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
