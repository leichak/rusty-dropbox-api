pub mod set_profile_photo;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetProfilePhotoArg {
    pub photo: PhotoSourceArg,
}

/// Per Stone spec — internally tagged union with a struct variant.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum PhotoSourceArg {
    Base64Data { base64_data: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetProfilePhotoResult {
    pub profile_photo_url: String,
}

/// Per Stone spec — error union for `set_profile_photo`. Wasn't typed before.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum SetProfilePhotoError {
    FileTypeError,
    FileSizeError,
    DimensionError,
    ThumbnailError,
    TransientError,
}
