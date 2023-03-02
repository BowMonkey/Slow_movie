use fast_log::config::Config;
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::RollingType;
use fast_log::plugin::packer::GZipPacker;
use log::{error, info, warn, LevelFilter};

pub fn init_log() {
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
}
