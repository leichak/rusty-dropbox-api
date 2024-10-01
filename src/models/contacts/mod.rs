pub mod delete_manual_contacts;
pub mod delete_manual_contacts_batch;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteManualContactsArg();

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteManualContactsResult();

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteManualContactsBatchArg {
    email_addresses: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteManualContactsBatchResult();
