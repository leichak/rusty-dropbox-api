pub mod delete_manual_contacts;
pub mod delete_manual_contacts_batch;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteManualContactsBatchArg {
    pub email_addresses: Vec<String>,
}
