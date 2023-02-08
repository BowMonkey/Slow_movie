use iced::alignment;
use iced::executor;
use iced::theme::Theme;
use iced::widget::{button, column, container, horizontal_space, pick_list, row, text, text_input};
use iced::{window, Application, Color, Command, Element, Length, Settings};

use native_dialog::FileDialog;

use std::env;

mod config;
mod do_wallpaper;

pub fn main() -> iced::Result {
    SlowMovie::run(Settings {
        window: window::Settings {
            size: (800, 400),
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
    time_magnification: i32,
    time_type: Option<Timetype>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Timetype {
    Second,
    Minute,
    Hour,
    None,
}
impl Timetype {
    const ALL: [Timetype; 3] = [Timetype::Second, Timetype::Minute, Timetype::Hour];
}
impl Default for Timetype {
    fn default() -> Timetype {
        Timetype::Second
    }
}
impl std::fmt::Display for Timetype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Timetype::Second => "Second",
                Timetype::Minute => "Minute",
                Timetype::Hour => "Hour",
                Timetype::None => "None",
            }
        )
    }
}

#[derive(Debug, Clone)]
enum Message {
    SetTime(Timetype),
    TimeInputChanged(String),
    ButtonSelect,
    Confirm,
    Flush(config::Config),
}

impl Application for SlowMovie {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let cur_config = config::load();
        let mut data = SlowMovie {
            theme: Theme::Dark,
            ..Self::default()
        };
        data.movie_path = cur_config.get_movie_path();
        data.time_str = cur_config.get_time_interval().to_string();
        data.time_type = match cur_config.get_time_type() {
            1 => Some(Timetype::Second),
            2 => Some(Timetype::Minute),
            3 => Some(Timetype::Hour),
            _ => Some(Timetype::Second),
        };
        (data, Command::none())
    }

    fn title(&self) -> String {
        String::from("简帧")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ButtonSelect => {
                let cur_path = env::current_dir().unwrap();
                let movie_file = FileDialog::new()
                    .set_location(&cur_path)
                    .show_open_single_file()
                    .unwrap();
                self.movie_path = movie_file.unwrap().display().to_string();
            }
            Message::SetTime(timetype) => {
                self.time_magnification = match timetype {
                    Timetype::None => 1,
                    Timetype::Second => 1,
                    Timetype::Minute => 60,
                    Timetype::Hour => 60 * 60,
                };
                self.time_type = Some(timetype);
            }
            Message::TimeInputChanged(value) => self.time_str = value,
            Message::Confirm => {
                let mut conf = config::Config::new();
                conf.set_movie_path(self.movie_path.clone());
                conf.set_time_interval(self.time_str.parse::<i32>().unwrap());
                conf.set_time_type(self.time_type.unwrap());
                config::write(&conf);
                //hide current window
                // spawn a thread to run set_wallpaper(&config)
                //do_wallpaper::set_wallpaper();
                //SetMode(Hidden)
                return window::close();
            }
            Message::Flush(value) => {
                self.movie_path = value.get_movie_path();
                self.time_str = value.get_time_interval().to_string();
                self.time_type = match value.get_time_type() {
                    1 => Some(Timetype::Second),
                    2 => Some(Timetype::Minute),
                    3 => Some(Timetype::Hour),
                    _ => None,
                };
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let title = text("Slow Movie")
            .width(Length::Fill)
            .size(100)
            .style(Color::from([0.5, 0.5, 0.5]))
            .horizontal_alignment(alignment::Horizontal::Center);
        let filepath_input = text(&self.movie_path).size(30);

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

        let pick_list = pick_list(&Timetype::ALL[..], self.time_type, Message::SetTime)
            .placeholder("Choose a Timetype...")
            .text_size(30);
        let ok_button = button("confirm").padding(10).on_press(Message::Confirm);
        let exit_button = button("exit").padding(10).on_press(Message::Confirm);

        let content = column![
            title,
            row![select_file_button, filepath_input].spacing(10),
            row![time_label, time_input, pick_list].spacing(10),
            row![horizontal_space(Length::Fill), ok_button, exit_button].spacing(10),
        ]
        .spacing(20)
        .padding(20)
        .max_width(800);

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
