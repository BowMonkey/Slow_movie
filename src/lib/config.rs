use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Timetype {
    Second,
    Minute,
    Hour,
    None,
}
impl Timetype {
    pub const ALL: [Timetype; 3] = [Timetype::Second, Timetype::Minute, Timetype::Hour];
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    config_path: String,
    movie_path: String,
    time_interval: u32,
    time_type: i32,
    frame_count: u64,
    exit_flag: i32,
}

impl Config {
    pub fn new() -> Config {
        let conf = Config::default();

        let conf_file = std::path::PathBuf::from(conf.config_path.clone());
        if !conf_file.is_file() {
            save_config(&conf);
        }

        conf
    }

    pub fn set_time_type(&mut self, time_type: Timetype) {
        self.time_type = match time_type {
            Timetype::None => 0,
            Timetype::Second => 1,
            Timetype::Minute => 2,
            Timetype::Hour => 3,
        };
    }
    pub fn get_time_type(&self) -> i32 {
        self.time_type
    }

    pub fn set_time_interval(&mut self, time_interval: u32) {
        self.time_interval = time_interval;
    }
    pub fn get_time_interval(&self) -> u32 {
        self.time_interval
    }

    pub fn set_movie_path(&mut self, path: String) {
        self.movie_path = path;
    }

    pub fn get_movie_path(&self) -> String {
        self.movie_path.clone()
    }

    pub fn set_frame_count(&mut self, count: u64) {
        self.frame_count = count;
    }

    pub fn get_frame_count(&self) -> u64 {
        self.frame_count
    }

    pub fn save(&self) {
        save_config(self);
    }

    pub fn set_exit_flag(&mut self, flag: bool) {
        self.exit_flag = match flag {
            true => 0,
            _ => 1,
        }
    }

    pub fn get_exit_flag(&mut self) -> bool {
        match self.exit_flag {
            0 => false,
            _ => true,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut movie_path = env::current_dir().unwrap();
        movie_path.push("Ayanami_Rei.mp4");

        let mut config_file_path = env::current_dir().unwrap();
        config_file_path.push("config.json");

        let conf = Config {
            config_path: String::from(config_file_path.to_str().unwrap()),
            movie_path: String::from(movie_path.to_str().unwrap()),
            time_interval: ((60 * 60) / 24) as u32,
            time_type: 1,
            frame_count: 1,
            exit_flag: 0,
        };

        conf
    }
}

pub fn load() -> Config {
    let mut conf = Config::new();
    let conf_file = std::path::PathBuf::from(conf.config_path.clone());
    if conf_file.is_file() {
        let settings = fs::read_to_string(conf_file).unwrap();
        conf = match serde_json::from_str(settings.as_str()) {
            Ok(cont) => cont,
            Err(why) => {
                log::error!(
                    "Serde_json convert config from string failed! Error Reason:{}",
                    why
                );
                return conf;
            }
        };
    }
    conf
}

pub fn save_config(config: &Config) {
    let contents = match serde_json::to_string(config) {
        Ok(cont) => cont,
        Err(why) => {
            log::error!(
                "Serde_json convert config to string failed! Error Reason:{}",
                why
            );
            return;
        }
    };

    match fs::write(config.config_path.clone(), contents.as_bytes()) {
        Ok(_) => {
            log::info!("Save config file ok!");
        }
        Err(why) => {
            log::error!("Save config file failed! Error Reason:{}", why);
        }
    }
}
