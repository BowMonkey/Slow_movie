#![windows_subsystem = "windows"]

use utillib::Errors;
pub fn main() -> Result<(), Errors> {
    let lock = named_lock::NamedLock::create("slowmovie")?;
    let _guard = lock.try_lock();
    match _guard {
        Ok(_) => Ok(()),
        Err(e) => {
            match e {
                named_lock::Error::WouldBlock => {
                    println!("double open, call gui!");
                    //call gui
                    Ok(())
                }
                _ => {
                    println!("Other Error!");
                    return Err(Errors::DoubleOpenError(e));
                }
            }
        }
    }
}
