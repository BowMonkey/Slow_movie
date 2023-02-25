#![windows_subsystem = "windows"]
use iced::alignment;
use iced::executor;
use iced::theme::Theme;
use iced::widget::{button, column, container, horizontal_space, pick_list, row, text, text_input};
use iced::{window, Application, Color, Command, Element, Length, Settings};

use native_dialog::{FileDialog, MessageDialog, MessageType};

use fast_log::config::Config;
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::RollingType;
use fast_log::plugin::packer::GZipPacker;
use log::{error, info, warn, LevelFilter};

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
    time_type: Timetype,
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
    Exit,
}

impl Application for SlowMovie {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        // logs
        fast_log::init(
            Config::new()
                .chan_len(Some(100000))
                .level(LevelFilter::Debug)
                .file_split(
                    "logs/", // current_exe dir
                    LogSize::MB(5),
                    RollingType::KeepNum(5),
                    GZipPacker {},
                ),
        )
        .unwrap();

        //configs
        log::debug!("start to load config file ...");
        let cur_config = config::load();
        let mut data = SlowMovie {
            theme: Theme::Dark,
            ..Self::default()
        };
        data.movie_path = cur_config.get_movie_path();
        log::debug!("Movie path from config file:{}", data.movie_path);
        data.time_str = cur_config.get_time_interval().to_string();
        log::debug!("Time from config file:{}", data.time_str);
        data.time_type = match cur_config.get_time_type() {
            1 => Timetype::Second,
            2 => Timetype::Minute,
            3 => Timetype::Hour,
            _ => Timetype::Second,
        };
        log::debug!(
            "Time type  from config file:{} [1:second 2:minute 3:hour 4:unknow]",
            cur_config.get_time_type()
        );

        (data, Command::none())
    }

    fn title(&self) -> String {
        String::from("简帧")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ButtonSelect => {
                let cur_path = env::current_dir();

                let cur_path = match cur_path {
                    Ok(path) => path,
                    Err(e) => {
                        error!("Read current dir failed! Reason:{} Exit...", e);
                        let _result = MessageDialog::new()
                            .set_title("Error")
                            .set_text("Read current dir path failed! ")
                            .set_type(MessageType::Error)
                            .show_alert();
                        return Command::none();
                    }
                };
                let movie_file = match FileDialog::new()
                    .set_location(&cur_path)
                    .show_open_single_file()
                {
                    Ok(f) => match f {
                        Some(f) => f,
                        None => {
                            log::info!("User chosed file path is none.");
                            return Command::none();
                        }
                    },
                    Err(e) => {
                        log::info!("User didend choose any file. Error:{}", e);
                        return Command::none();
                    }
                };
                self.movie_path = movie_file.display().to_string();
                return Command::none();
            }
            Message::SetTime(timetype) => {
                self.time_type = timetype;
            }
            Message::TimeInputChanged(value) => {
                self.time_str = value;
            }
            Message::Confirm => {
                let mut conf = config::Config::new();
                conf.set_movie_path(self.movie_path.clone());
                let time = match self.time_str.parse::<i32>() {
                    Ok(t) => t,
                    Err(e) => {
                        log::warn!("User input an invalid time string. Error:{}", e);
                        let _result = MessageDialog::new()
                            .set_title("Error")
                            .set_text(format!("Input of time is invlid! Please input an number! Current input is \"{}\"", self.time_str).as_str())
                            .set_type(MessageType::Error)
                            .show_alert();
                        return Command::none();
                    }
                };
                conf.set_time_interval(time);
                conf.set_time_type(self.time_type);
                config::save_config(&conf);
                // spawn a thread to run set_wallpaper(&config)
                //do_wallpaper::set_wallpaper();
                //return window::minimize(true);
                return window::set_mode(window::Mode::Hidden);
            }
            Message::Exit => {
                return window::close();
            }
        }
        log::logger().flush();
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

        let pick_list = pick_list(&Timetype::ALL[..], Some(self.time_type), Message::SetTime)
            .placeholder("Choose a Timetype...")
            .text_size(30);
        let ok_button = button("confirm").padding(10).on_press(Message::Confirm);
        let exit_button = button("exit").padding(10).on_press(Message::Exit);

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
