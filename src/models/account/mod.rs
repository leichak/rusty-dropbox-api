mod set_profile_photo;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetProfilePhotoArg {
    pub photo: PhotoSourceArg,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PhotoSourceArg {
    Base64Data {
        #[serde(rename = ".tag")]
        tag: String,
        base64_data: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetProfilePhotoResult {
    pub profile_photo_url: String,
}
