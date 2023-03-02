use super::errors::*;
use super::utils::*;

use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio};
pub fn generate_frame_count(ffprobe: &String, moviepath: &String) -> Result<u64, Errors> {
    let get_frame_count = ffprobe.to_owned() + &String::from(" -v error -select_streams v:0 -count_packets -show_entries stream=nb_read_packets -of csv=p=0 ") + &moviepath ;
    let child = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &get_frame_count])
            .creation_flags(0x08000000)
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute process")
    } else {
        return Err(Errors::FfprobeLost);
    };
    let output = child.wait_with_output().expect("failed to wait on child");
    let total_frame = String::from_utf8_lossy(&output.stdout).to_string();
    let total_frame = strip_trailing_newline(&total_frame);
    match total_frame.parse::<u64>() {
        Ok(num) => return Ok(num),
        Err(e) => {
            return Err(Errors::FrameCountError(e.to_string()));
        }
    };
}

pub fn generate_frame_picture(
    ffmpeg: &String,
    movie_path: &String,
    cur_frame: u64,
    picutre_path: &str,
) -> Result<(), Errors> {
    let mut picture_path = std::path::PathBuf::from(picutre_path);
    picture_path.push("frame.png");
    let picture_path = picture_path.display().to_string();

    let get_frame_picture = ffmpeg.to_owned()
        + &String::from(" -i ")
        + &movie_path.clone()
        + &String::from(r" -vf select=eq(n\,")
        + &cur_frame.to_string()
        + &String::from(r") -vsync 0 -vframes 1 -f image2 ")
        + &picture_path;

    let child = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &get_frame_picture])
            .creation_flags(0x08000000)
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute process")
    } else {
        return Err(Errors::OSTypeError);
    };
    let output = child.wait_with_output().expect("Faile to wait on child.");
    if !output.status.success() {
        return Err(Errors::FfmpegRunError(
            String::from_utf8_lossy(&output.stdout).to_string(),
        ));
    }
    Ok(())
}
