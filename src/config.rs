use std::env;
use std::fs;
use std::io::{Read, Write};

use image::error::ParameterErrorKind;
use serde::{Deserialize, Serialize};

use crate::Timetype;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    movie_path: String,
    time_interval: i32,
    time_type: i32,
    frame_count: u64,
}

impl Config {
    pub fn new() -> Config {
        Config {
            movie_path: String::from(""),
            time_interval: ((60 * 60) / 24) as i32,
            time_type: 150,
            frame_count: 1,
        }
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
        let mut config_path = env::current_dir().unwrap();
        config_path.push("Ayanami_Rei.mp4");
        Config {
            movie_path: String::from(config_path.to_str().unwrap()),
            time_interval: ((60 * 60) / 24) as i32,
            time_type: 150,
            frame_count: 1,
        }
    }
}

pub fn load() -> Config {
    let mut config_path = env::current_dir().unwrap();
    config_path.push("config.json");
    let mut conf = Config::new();

    if config_path.is_file() {
        let configs = fs::read_to_string(config_path).unwrap();
        conf = serde_json::from_str(configs.as_str()).unwrap();
    } else {
        conf = Config::default();
    }

    conf
}

pub fn write(config: &Config) {
    let mut config_path = env::current_dir().unwrap();
    config_path.push("config.json");
    let mut f = fs::File::create(config_path).unwrap();
    let ret = f.write_all(serde_json::to_string(config).unwrap().as_bytes());
    match ret {
        Ok(_) => {}
        Err(why) => {
            println!("error:{}", why);
        }
    }
}
