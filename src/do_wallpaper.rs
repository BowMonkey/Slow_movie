use std::env;
use std::time;
use std::thread;

pub fn set_wallpaper() {
    let current_path = env::current_dir().unwrap();
    loop {
        let final_path: String = current_path.display().to_string() + "\\frame.png";
        wallpaper::set_from_path(&final_path).unwrap();
        thread::sleep(time::Duration::from_millis(5000));
    }
}
