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

use utillib::load as config_load;
use utillib::save_config;
use utillib::Timetype;

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
    frame_str: String,
    time_type: Timetype,
    change_flag: bool,
}

#[derive(Debug, Clone)]
enum Message {
    SetTime(Timetype),
    TimeInputChanged(String),
    FrameInputChanged(String),
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
        let cur_config = config_load();
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
                self.change_flag = true;
            }
            Message::SetTime(timetype) => {
                self.time_type = timetype;
            }
            Message::TimeInputChanged(value) => {
                self.time_str = value;
            }
            Message::FrameInputChanged(value) => {
                self.frame_str = value;
            }
            Message::Confirm => {
                let mut conf = config_load();
                conf.set_movie_path(self.movie_path.clone());
                let time = match self.time_str.parse::<u32>() {
                    Ok(mut t) => {
                        if t < 3 {
                            t = 3;
                        }
                        t
                    }
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

                let frame_sec = match self.frame_str.parse::<u64>() {
                    Ok(mut t) => {
                        if t < 0 {
                            t = 0;
                        }
                        match self.time_type {
                            Second => t,
                            Minute => t * 60,
                            Hour => t * 60 * 60,
                            _ => t,
                        }
                    }
                    Err(e) => {
                        log::warn!("User input an invalid frame time string. Error:{}", e);
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
                conf.set_exit_flag(false);
                conf.set_frame_count(frame_sec * 24);
                save_config(&conf);
                return window::close();
            }
            Message::Exit => {
                let mut conf = config_load();
                conf.set_exit_flag(true);
                save_config(&conf);
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

        let time_pick_list = pick_list(&Timetype::ALL[..], Some(self.time_type), Message::SetTime)
            .placeholder("Choose a Timetype...")
            .text_size(30);

        let frame_label = text("Frame Start time:").size(30);
        let frame_input = text_input(
            "Start time of movie ... ",
            &self.frame_str,
            Message::FrameInputChanged,
        )
        .padding(10)
        .size(20);

        let frame_pick_list = pick_list(&Timetype::ALL[..], Some(self.time_type), Message::SetTime)
            .placeholder("Choose a Timetype...")
            .text_size(30);

        let ok_button = button("confirm").padding(10).on_press(Message::Confirm);
        let exit_button = button("exit").padding(10).on_press(Message::Exit);

        let content = column![
            title,
            row![select_file_button, filepath_input].spacing(10),
            row![time_label, time_input, time_pick_list].spacing(10),
            row![frame_label, frame_input, frame_pick_list].spacing(10),
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
