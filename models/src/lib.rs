mod app;
mod check_job_status;
mod check_remove_member_job_status;
mod check_share_jobs_status;
mod copy;
mod copy_batch;
mod copy_batch_check;
mod copy_reference_get;
mod copy_reference_save;
mod count;
mod create;
mod create_folder;
mod create_folder_batch;
mod create_folder_batch_check;
mod create_shared_link_with_settings;
mod delete_all_closed;
mod delete_manual_contacts;
mod delete_manual_contacts_batch;
mod get;
mod get_account;
mod get_account_batch;
mod get_current_account;
mod get_file_lock_batch;
mod get_file_metadata;
mod get_file_metadata_batch;
mod get_folder_metadata;
mod get_metadata;
mod get_preview;
mod get_shared_link_file;
mod get_shared_link_metadata;
mod get_space_usage;
mod get_temporary_link;
mod get_temporary_upload_link;
mod get_thumbnail;
mod get_thumbnail_batch;
mod list;
mod list_continue;
mod list_file_members;
mod list_file_members_batch;
mod list_file_members_continue;
mod list_folder;
mod list_folder_continue;
mod list_folder_get_latest_cursor;
mod list_folder_longpoll;
mod list_folder_members;
mod list_folder_members_continue;
mod list_folders;
mod list_folders_continue;
mod list_mountable_folders;
mod list_mountable_folders_continue;
mod list_received_files;
mod list_received_files_continue;
mod list_revisions;
mod list_shared_links;
mod lock_file_batch;
mod modify_shared_link_settings;
mod move_batch_check;
mod properties_add;
mod properties_overwrite;
mod properties_remove;
mod properties_search;
mod properties_search_continue;
mod properties_update;
mod relinquish_file_membership;
mod relinquish_folder_membership;
mod remove_file_member_2;
mod remove_folder_member;
mod restore;
mod revoke_shared_link;
mod save_url_check_job_status;
mod search_continue;
mod set_profile_photo;
mod tags_add;
mod tags_get;
mod tags_remove;
mod templates_add_for_user;
mod templates_get_for_user;
mod templates_list_for_user;
mod templates_remove_for_user;
mod templates_update_for_user;
mod token_revoke;
mod transfer_folder;
mod upload_session_finish_batch_check;
mod user;

#[cfg(test)]
use api::MOCK_SERVER;

pub use set_profile_photo::{SetProfilePhotoRequest, SetProfilePhotoResponse};

mod utils {
    use serde::{Deserialize, Serialize};

    pub trait Utils {
        fn parameters(&self) -> impl Serialize + Deserialize;
    }
}
