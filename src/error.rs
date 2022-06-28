use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    FileNotExist,
    InvalidImage,
    ReadFileError,
    FontError,
    ImageWriteError,
}

impl From<&Error> for &'static str {
    fn from(m: &Error) -> &'static str {
        match m {
            &Error::FileNotExist => "file not exist",
            &Error::InvalidImage => "invalid image",
            &Error::ReadFileError => "read file error",
            &Error::FontError => "font parse error",
            &Error::ImageWriteError => "image write error",
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.into())
    }
}
