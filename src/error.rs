use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("general io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("route error: {0}")]
    Route(String),
    #[error("utf8 string error: {0}")]
    Utf8String(#[from] std::string::FromUtf8Error),
    #[error("utf8 str error: {0}")]
    Utf8Str(#[from] std::str::Utf8Error),
    #[error("mac parse error : {0}")]
    Mac(#[from] eui48::ParseError),
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "freebsd"))]
    #[error("nix error : {0}")]
    Nix(#[from] nix::Error),
}
