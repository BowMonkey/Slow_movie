use iced::alignment;
use iced::theme::Theme;
use iced::widget::{button, column, container, horizontal_space, radio, row, text, text_input};
use iced::{window, Color, Element, Length, Application,Command, Settings,};
use iced::executor;

mod config;
mod do_wallpaper;

pub fn main() -> iced::Result {
    SlowMovie::run(Settings {
        window: window::Settings {
            size: (600, 400),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Default)]
struct SlowMovie {
    theme: Theme,
    movie_path: String,
    time_str: String,
    time_interval: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Timetype {
    Second,
    Minute,
    Hour,
}
#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    SetTime(Timetype),
    TimeInputChanged(String),
    ButtonSelect,
    Confirm,
    Exit,
}

impl Application for SlowMovie {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let cur_config = config::load();
        (SlowMovie {
            theme: Theme::Dark,
            ..Self::default()
        },
        Command::none())
    }

    fn title(&self) -> String {
        String::from("简帧")
    }

    fn update(&mut self, message: Message)  -> Command<Message>{
        match message {
            Message::InputChanged(value) => self.movie_path = value,
            Message::ButtonSelect => {
                todo!();
            }
            Message::SetTime(timetype) => {
                let value = match timetype{
                    Timetype::Second =>  1,
                    Timetype::Minute => 60,
                    Timetype::Hour => 60*60, 
                };
                self.time_interval = value;
            },
            Message::TimeInputChanged(value) => self.movie_path = value,
            Message::Confirm => {
                do_wallpaper::set_wallpaper();
            }
            Message::Exit =>{ return window::close();},
        }
        Command::none()
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

        let time_input = text_input(
            "Time of interval ... ",
            &self.time_str,
            Message::TimeInputChanged,
        )
        .padding(10)
        .size(20);
        let second_radio = radio("Seconds", Timetype::Second, Some(Timetype::Second), Message::SetTime);
        let min_radio = radio("Minutes", Timetype::Minute, Some(Timetype::Minute), Message::SetTime);
        let hour_radio = radio("Hours", Timetype::Hour, Some(Timetype::Hour), Message::SetTime);

        let ok_button = button("confirm").padding(10).on_press(Message::Confirm);
        let exit_button = button("exit").padding(10).on_press(Message::Exit);

        let content = column![
            title,
            row![select_file_button, filepath_input].spacing(10),
            row![time_label, time_input].spacing(10),
            row![horizontal_space(Length::Fill), second_radio, min_radio, hour_radio].spacing(10),
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
