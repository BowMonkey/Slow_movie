#![windows_subsystem = "windows"]

use std::{thread, time::Duration};
use utillib::Errors;
pub fn main() -> Result<(), Errors> {
    let lock = named_lock::NamedLock::create("slowmovie")?;
    let _guard = lock.try_lock();
    match _guard {
        Ok(_) => {
            println!("I'm the chosen one!");
        },
        Err(e) => {
            match e {
                named_lock::Error::WouldBlock => {
                    println!("Find brothers, call gui!");
                    //call gui
                }
                _ => {
                    println!("Other Error!");
                }
            }
        }
    }
    thread::sleep(Duration::from_secs(10));
    println!("die!");
    Ok(())
}
