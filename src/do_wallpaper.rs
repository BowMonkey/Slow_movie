use std::process::{Command, Stdio};
use std::time::Duration;
use std::{env, fs, thread};

use native_dialog::{MessageDialog, MessageType};

use fast_log::config::Config;
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::RollingType;
use fast_log::plugin::packer::GZipPacker;
use log::{error, info, warn, LevelFilter};

use utillib::{config, video_process::*, utils::*, Errors};

pub fn main() -> Result<(), Errors> {
    // called by main process
    // stop as main toad
    // read config from config file and do loop
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
    let conf = config::load();
    log::debug!("{:#?}", dbg!(conf));

    let handle = thread::spawn(|| {
        loop {
            //1. get movie frame count
            // here we use command line to call ffmpeg to generate piticular frame
            let ffprobe = match env::current_dir() {
                Ok(mut path) => {
                    path.push("ffmpeg");
                    path.push("ffprobe.exe");
                    path.display().to_string()
                }
                Err(e) => {
                    log::error!("ffmpeg is not exist! Error:{}", e);
                    let _result = MessageDialog::new()
                        .set_title("Error")
                        .set_text("Get ffprobe path failed! ")
                        .set_type(MessageType::Error)
                        .show_alert();
                    return Err(Errors::InnerIOError(e));
                }
            };
            let total_frame = match generate_frame_count(&ffprobe, &conf.get_movie_path()){
                Ok(n) => n,
                Err(e) => {
                    log::error!("Get total frame number from ffmprobe error!");
                    let _result = MessageDialog::new()
                    .set_title("Error")
                    .set_text("Get total frame number failed! ")
                    .set_type(MessageType::Error)
                    .show_alert();
                    return Err(e);
                }
            };
            log::info!("total frames:{}", total_frame);

            //2. delete old frame picture if it exist
            log::info!("Start to delete old frame.png");
            let frame_picture = match env::current_dir() {
                Ok(mut path) => {
                    path.push("frame.png");
                    path.display().to_string()
                }
                Err(e) => {
                    log::warn!("Get current dir error! Error:{}", e);
                    return e;
                }
            };

            match delete_file(&frame_picture){
                Ok(_) => { log::info!("Delete old frame.png ok!");},
                Err(e) => {
                    log::error!("Delete old frame.png error! Error:{}", e);
                    return e;
                }
            }
           
            //3. convert frame to picture
            log::info!("Start to convert frame to frame.png");
            let ffmpeg = match env::current_dir() {
                Ok(mut path) => {
                    path.push("ffmpeg");
                    path.push("ffmpeg.exe");
                    path.display().to_string()
                }
                Err(e) => {
                    log::error!("ffmpeg is not exist! Error:{}", e);
                    let _result = MessageDialog::new()
                        .set_title("Error")
                        .set_text("Get ffmpeg path failed! ")
                        .set_type(MessageType::Error)
                        .show_alert();
                    return;
                }
            };

            let mut cur_frame = conf.get_frame_count() + 1;
            cur_frame = cur_frame % total_frame;

            let mut picture_path = env::current_dir().unwrap();
            picture_path.push("frame.png");
            let picture_path = picture_path.display().to_string();
            match generate_frame_picture(&ffmpeg, &conf.get_movie_path(), cur_frame, &picture_path) {
                Ok(_) => {
                    log::info!("Convert frame to picture ok.");
                },
                Err(e) => {
                    log::error!(
                        "Convert frame to picture error! Error:{}",
                        String::from_utf8_lossy(&output.stdout).to_string()
                    );
                    let _result = MessageDialog::new()
                    .set_title("Error")
                    .set_text("Convert frame to picture failed! ")
                    .set_type(MessageType::Error)
                    .show_alert();
                    return e;
                }
            }

            //4. if all is ok, now we have a picture frame.png . It's time to set it as wallpaper.
            log::info!("Start to check new frame.png.");
            let frame_picture = match env::current_dir() {
                Ok(mut path) => {
                    path.push("frame.png");
                    path
                }
                Err(e) => {
                    log::error!("frame.png is not exist! Error:{}", e);
                    let _result = MessageDialog::new()
                        .set_title("Error")
                        .set_text("Get frame picture failed! ")
                        .set_type(MessageType::Error)
                        .show_alert();
                    return;
                }
            };
            match frame_picture.try_exists() {
                Ok(exist) => {
                    if !exist {
                        log::error!("frame.png is not exist!");
                        let _result = MessageDialog::new()
                            .set_title("Error")
                            .set_text("Get frame picture failed! ")
                            .set_type(MessageType::Error)
                            .show_alert();
                        return;
                    }
                }
                Err(e) => {
                    log::error!("frame.png is not exist! Error:{}", e);
                    let _result = MessageDialog::new()
                        .set_title("Error")
                        .set_text("Get frame picture failed! ")
                        .set_type(MessageType::Error)
                        .show_alert();
                    return;
                }
            }

            log::info!("Start to set frame.png as wallpaper.");
            match wallpaper::set_from_path(&frame_picture.display().to_string()) {
                Ok(_) => {
                    // if set wallpaper ok, update config file
                    conf.set_frame_count(cur_frame);
                    conf.save();
                }
                Err(e) => {
                    log::error!("Set frame.png as wallper error! Error:{}", e);
                    let _result = MessageDialog::new()
                        .set_title("Error")
                        .set_text("Set frame as wallpaper failed! ")
                        .set_type(MessageType::Error)
                        .show_alert();
                }
            };

            //4. thread sleep
            thread::sleep(Duration::from_secs(u64::from(conf.get_time_interval())));
        }

    });

    Ok(())
}

pub fn set_wallpaper(mut conf: Config) {
    let handle = thread::spawn(|| {
        let _detached_handle = thread::spawn(move || {
            // Here we sleep to make sure that the first thread returns before.
            thread::sleep(Duration::from_millis(10));
           
    });

    handle.join().unwrap();
    log::info!("main thread closed here.");
}
