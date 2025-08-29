use thiserror::Error;

pub type Result<T, E = ItamisuError> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum ItamisuError {
    #[error("can't parse this file: {0}")]
    CantParseFile(#[from] toml::de::Error),
    #[error("can't convert a type to text: {0}")]
    CantConvertFile(#[from] toml::ser::Error),
    #[error("can't read/write the file: {0}")]
    CantAccessFile(#[from] std::io::Error),
    #[error("unknown data store error")]
    Unknown,
}

pub fn pretty_exit<T>(error: T) -> !
where
    T: AsRef<str>,
{
    eprintln!("{}", error.as_ref());

    std::process::exit(1)
}
