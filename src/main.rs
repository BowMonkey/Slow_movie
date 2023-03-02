#![windows_subsystem = "windows"]
use std::thread;

use fast_log::config::Config;
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::RollingType;
use fast_log::plugin::packer::GZipPacker;
use log::LevelFilter;

mod do_wallpaper;
mod run_gui;

use utillib::{config, Errors};

pub fn main() -> Result<(), Errors> {
    // prevent muti instances of program
    let lock = named_lock::NamedLock::create("slowmovie")?;
    let _guard = lock.try_lock();
    match _guard {
        Ok(_) => {
            println!("I'm the chosen one!");
        }
        Err(e) => match e {
            named_lock::Error::WouldBlock => {
                println!("Find brothers!");
                return run_gui::run();
            }
            _ => {
                println!("Other Error!");
                return Err(Errors::DoubleOpenError(e));
            }
        },
    }

    // log
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

    //init exit flag
    let mut conf = config::load();
    conf.set_exit_flag(false);
    config::save_config(&conf);

    //gui
    run_gui::run()?;

    //check exit
    let conf = config::load();
    if conf.should_exit() {
        return Ok(());
    }

    let handle = thread::spawn(do_wallpaper::run);
    match handle.join().unwrap() {
        Ok(_) => return Ok(()),
        Err(e) => {
            log::error!("Runtime error! ");
            return Err(e);
        }
    }
}
