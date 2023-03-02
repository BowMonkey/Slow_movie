use std::env;
use std::os::windows::process::CommandExt;
use std::process::Command;

use utillib::{utils::*, Errors};

pub fn run() -> Result<(), Errors> {
    let gui = match env::current_exe() {
        Ok(mut path) => {
            path.pop();
            path.push("gui.exe");
            path.display().to_string()
        }
        Err(e) => {
            log::error!("Get gui path error! Error:{}", e);
            alert_dialog("Get gui path failed! ");
            return Err(Errors::InnerIOError(e));
        }
    };
    let child_status = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &gui])
            .creation_flags(0x08000000)
            .status()
            .expect("failed to execute gui.exe")
    } else {
        return Err(Errors::OSTypeError);
    };

    if !child_status.success() {
        alert_dialog("Gui run error!");
        log::error!("Gui run error!");
        return Err(Errors::GuiRunError(format!(
            "Run gui.exe failed. Finished with:{}",
            child_status
        )));
    }

    Ok(())
}
