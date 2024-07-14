mod app;
mod delete_manual_contacts;
mod delete_manual_contacts_batch;
mod properties_add;
mod set_profile_photo;
mod token_revoke;
mod user;

pub use set_profile_photo::{SetProfilePhotoRequest, SetProfilePhotoResponse};

mod utils {
    use serde::{Deserialize, Serialize};

    pub trait Utils {
        fn parameters(&self) -> impl Serialize + Deserialize;
    }
}
