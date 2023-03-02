use std::time::Duration;
use std::{env, thread};

use utillib::{config, utils::*, video_process::*, Errors};

pub fn run() -> Result<(), Errors> {
    loop {
        let mut conf = config::load();
        if conf.should_exit() {
            return Ok(());
        }
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
                alert_dialog("Get ffprobe path failed! ");
                return Err(Errors::InnerIOError(e));
            }
        };

        let total_frame = match generate_frame_count(&ffprobe, &conf.get_movie_path()) {
            Ok(n) => n,
            Err(e) => {
                log::error!("Get total frame number from ffmprobe error!");
                alert_dialog("Get total frame number failed! ");
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
                return Err(Errors::InnerIOError(e));
            }
        };

        match delete_file(&frame_picture) {
            Ok(_) => {
                log::info!("Delete old frame.png ok!");
            }
            Err(e) => {
                log::error!("Delete old frame.png error! Error:{}", e);
                return Err(e);
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
                alert_dialog("Get ffmpeg path failed! ");
                return Err(Errors::FfmpegLost);
            }
        };

        let mut cur_frame = conf.get_frame_count() + 1;
        cur_frame %= total_frame;

        let mut picture_path = env::current_dir().unwrap();
        picture_path.push("frame.png");
        let picture_path = picture_path.display().to_string();
        match generate_frame_picture(&ffmpeg, &conf.get_movie_path(), cur_frame, &picture_path) {
            Ok(_) => {
                log::info!("Convert frame to picture ok.");
            }
            Err(e) => {
                log::error!("Convert frame to picture error! Err:{}", e);
                alert_dialog("Convert frame to picture failed! ");
                return Err(e);
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
                alert_dialog("Get frame picture failed! ");
                return Err(Errors::InnerIOError(e));
            }
        };
        match frame_picture.try_exists() {
            Ok(exist) => {
                if !exist {
                    log::error!("frame.png is not exist!");
                    alert_dialog("Get frame picture failed! ");
                    return Err(Errors::FramePictureLost);
                }
            }
            Err(e) => {
                log::error!("frame.png is not exist! Error:{}", e);
                alert_dialog("Get frame picture failed! ");
                return Err(Errors::FramePictureLost);
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
                alert_dialog("Set frame as wallpaper failed! ");
            }
        };

        //4. thread sleep
        thread::sleep(Duration::from_secs(u64::from(conf.get_time_interval())));
    }
}
