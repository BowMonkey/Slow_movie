use native_dialog::{MessageDialog, MessageType};
use std::process::{Stdio,Command};
use std::time::Duration;
use std::{env, fs, thread};

use std::os::windows::process::CommandExt;

use crate::config::*;

pub fn set_wallpaper(mut conf: Config) {
    let handle = thread::spawn(|| {
        let _detached_handle = thread::spawn(move || {
             // Here we sleep to make sure that the first thread returns before.
            thread::sleep(Duration::from_millis(10));
            loop {
                //1. get movie frame count
                // here we use command line to call ffmpeg to generate piticular frame
                let ffprobe = match env::current_dir() {
                    Ok(mut path) => {
                        path.push("ffmpeg");
                        path.push("ffprobe.exe");
                        path
                    }
                    Err(e) => {
                        log::error!("ffmpeg is not exist! Error:{}", e);
                        let _result = MessageDialog::new()
                            .set_title("Error")
                            .set_text("Get ffprobe path failed! ")
                            .set_type(MessageType::Error)
                            .show_alert();
                        return;
                    }
                };
                let get_frame_count = ffprobe.display().to_string() + &String::from(" -v error -select_streams v:0 -count_packets -show_entries stream=nb_read_packets -of csv=p=0 ") + &conf.get_movie_path().clone() ;
                let child = if cfg!(target_os = "windows") {
                    Command::new("cmd")
                        .args(["/C",  &get_frame_count])
                        .creation_flags(0x08000000)
                        .stdout(Stdio::piped())
                        .spawn()
                        .expect("failed to execute process")
                } else {
                    log::error!("Run ffprobe to get total frame of selected video error!");
                    return;
                };
                let output = child.wait_with_output().expect("failed to wait on child");
                let total_frame = String::from_utf8_lossy(&output.stdout).to_string();
                let total_frame = strip_trailing_newline(&total_frame);
                let total_frame = match total_frame.parse::<u64>() {
                    Ok(num) => num,
                    Err(e) => {
                        log::error!("Get frame count error! Error:{}", e);
                        let _result = MessageDialog::new()
                            .set_title("Error")
                            .set_text("Get frame count failed! ")
                            .set_type(MessageType::Error)
                            .show_alert();
                        return;
                    }
                };
                log::info!("total frames:{}", total_frame);

                //2. delete old frame picture if it exist
                log::info!("Start to delete old frame.png");
                let frame_picture = match env::current_dir() {
                    Ok(mut path) => {
                        path.push("frame.png");
                        path
                    }
                    Err(e) => {
                        log::warn!("Get current dir error! Error:{}", e);
                        return;
                    }
                };
                match frame_picture.try_exists() {
                    Ok(exist) => {
                        if exist {
                            //delete old picture
                            match fs::remove_file(frame_picture) {
                                Ok(_) => {
                                    log::info!("Delete old frame.png ok!");
                                }
                                Err(e) => {
                                    log::error!("Delete old frame.png error! Error:{}", e);
                                    let _result = MessageDialog::new()
                                        .set_title("Error")
                                        .set_text("Delete old frame.png failed! ")
                                        .set_type(MessageType::Error)
                                        .show_alert();
                                    return;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        //if old frame.png is not exist, it is ok.
                    }
                }

                //3. convert frame to picture
                log::info!("Start to convert frame to frame.png");
                let ffmpeg = match env::current_dir() {
                    Ok(mut path) => {
                        path.push("ffmpeg");
                        path.push("ffmpeg.exe");
                        path
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
                cur_frame  = cur_frame % total_frame;

                let mut picture_path = env::current_dir().unwrap();
                picture_path.push("frame.png");
                let picture_path = picture_path.display().to_string();

                let get_frame_picture = ffmpeg.display().to_string()
                    + &String::from(" -i ")
                    + &conf.get_movie_path().clone()
                    + &String::from(r" -vf select=eq(n\,")
                    + &cur_frame.to_string()
                    + &String::from(r") -vsync 0 -vframes 1 -f image2 ")
                    + &picture_path;
                log::info!("{}", get_frame_picture);
                let child = if cfg!(target_os = "windows") {
                    Command::new("cmd")
                        .args(["/C", &get_frame_picture])
                        .creation_flags(0x08000000)
                        .stdout(Stdio::piped())
                        .spawn()
                        .expect("failed to execute process")
                } else {
                    log::error!("Run ffmpeg to extract frmae to picture error!");
                    return;
                };
                let output = child.wait_with_output().expect("Faile to wait on child.");
                if !output.status.success() {
                    log::error!(
                        "Convert frame to picture error! Error:{}",
                        String::from_utf8_lossy(&output.stdout).to_string()
                    );
                    let _result = MessageDialog::new()
                        .set_title("Error")
                        .set_text("Conver frame to picture failed! ")
                        .set_type(MessageType::Error)
                        .show_alert();
                    return;
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
    });

    handle.join().unwrap();
    log::info!("main thread closed here.");
}

fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}
