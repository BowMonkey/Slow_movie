use std::env;
use std::thread;
use std::time;

use crate::config::Config;

pub fn set_wallpaper(conf:&Config) {
    // all in loop
    //1. read movie path to get a picture of frame
    //2. set picture as wallpaper 
    //3. sleep 
    let current_path = env::current_dir().unwrap();
    loop {
        let final_path: String = current_path.display().to_string() + "\\frame.png";
        //wallpaper::set_from_path(&final_path).unwrap();
        thread::sleep(time::Duration::from_millis(5000));
    }
}
