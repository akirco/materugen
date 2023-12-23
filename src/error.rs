use image::ImageError;
use color_thief::Error as ColorThiefError;

#[derive(Debug)]
pub struct Error(pub String);

impl From<ImageError> for Error {
    fn from(value: ImageError) -> Self {
        Self(value.to_string())
    }
}

impl From<ColorThiefError> for Error {
    fn from(value: ColorThiefError) -> Self {
        Self(format!("{}", value))
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self(value.to_string())
    }
}
