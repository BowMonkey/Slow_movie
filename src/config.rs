use std::env;
use std::fs;

use serde::{Deserialize, Serialize};

use crate::Timetype;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    config_path: String,
    movie_path: String,
    time_interval: i32,
    time_type: i32,
    frame_count: u64,
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

    pub fn set_time_interval(&mut self, time_interval: i32) {
        self.time_interval = time_interval;
    }
    pub fn get_time_interval(&self) -> i32 {
        self.time_interval
    }

    pub fn set_movie_path(&mut self, path: String) {
        self.movie_path = path;
    }

    pub fn get_movie_path(&self) -> String {
        self.movie_path.clone()
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
            time_interval: ((60 * 60) / 24) as i32,
            time_type: 1,
            frame_count: 1,
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
