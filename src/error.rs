use image::ImageError;
use color_thief::Error as ColorThiefError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Image error: {0}")] Image(#[from] ImageError),

    #[error("Color extraction error: {0}")] ColorThief(#[from] ColorThiefError),

    #[error("IO error: {0}")] Io(#[from] std::io::Error),

    #[error("{0}")] Custom(String),

    #[error("Invalid hex color format")] InvalidHexColor,
}
