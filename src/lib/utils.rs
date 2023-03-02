pub fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}

use super::errors::*;
pub fn delete_file(path: &str) -> Result<(), Errors> {
    let frame_picture = std::path::PathBuf::from(path);
    match frame_picture.try_exists() {
        Ok(exist) => {
            if exist {
                //delete old picture
                match std::fs::remove_file(frame_picture) {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(e) => {
                        return Err(Errors::DeleteFileError(e.to_string()));
                    }
                }
            }
        }
        Err(_) => {
            // it is ok if file doesn't exist.
            return Ok(());
        }
    }
    Ok(())
}
