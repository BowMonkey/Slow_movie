use thiserror::Error;
#[derive(Error, Debug)]
pub enum Errors {
    #[error("Can not find gui.exe")]
    GuiLost,

    #[error("Run gui.exe error. Message:{0}")]
    GuiRunError(String),

    #[error("Can not find ffprobe.exe")]
    FfprobeLost,

    #[error("Can not find ffmpeg.exe")]
    FfmpegLost,

    #[error("Can not find frame.png")]
    FramePictureLost,

    #[error("Run ffmpeg.exe error. Message:{0}")]
    FfmpegRunError(String),

    #[error("Delete file failed. Message:{0}")]
    DeleteFileError(String),

    #[error("Convert frame count from command-line ffmprobe.exe error. Message:{0}")]
    FrameCountError(String),

    #[error("This Program is only for windows")]
    OSTypeError,

    #[error(transparent)]
    DoubleOpenError(#[from] named_lock::Error),

    #[error(transparent)]
    InnerIOError(#[from] std::io::Error),

    #[error(transparent)]
    LogError(#[from] fast_log::error::LogError),

    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}
