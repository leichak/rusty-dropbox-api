use anyhow::Result;
use api::{
    anyhow, get_endpoint_url, implement_service, ApiError, AsyncClient, BoxFuture, Endpoint,
    Headers, Service, SyncClient,
};

use serde::{Deserialize, Serialize};

use std::{collections::HashMap, future::Future, pin::Pin};

use crate::{
    strum_macros::AsRefStr,
    utils::{self, Utils},
};

/// Member identification type enum
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum MemberIdentificationType<'a> {
    dropbox_id(&'a str),
    email(&'a str),
}

/// Member access type enum
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum MemberAccessLevel {
    owner,
    editor,
    viewer,
    viewer_no_comment,
    traverse,
    no_access,
}

/// Add members to folder
/// https://www.dropbox.com/developers/documentation/http/documentation#sharing-add_folder_member
#[derive(Debug, PartialEq)]
pub struct AddFolderMemberRequest<'a> {
    access_token: &'a str,
    custom_message: &'a str,
    members: Vec<(MemberAccessLevel, MemberIdentificationType<'a>)>,
    quiet: bool,
    shared_folder_id: &'a str,
}

/// Response struct for adding folder member
#[derive(Deserialize, Debug)]
pub struct AddFolderMemberResponse {
    profile_photo_url: String,
}

/// Implementation of trait for payload
impl utils::Utils for AddFolderMemberRequest<'_> {
    fn payload(&self) -> Option<impl Serialize + Deserialize> {
        #[derive(Serialize, Deserialize)]
        struct Member<'a> {
            access_level: &'a str,
            member: HashMap<&'a str, &'a str>,
        }

        #[derive(Serialize, Deserialize)]
        struct Payload<'a> {
            custom_message: &'a str,
            members: Vec<Member<'a>>,
            quiet: bool,
            shared_folder_id: &'a str,
        }

        let mut members = vec![];
        for (access, ident) in &self.members {
            let id = match ident {
                MemberIdentificationType::dropbox_id(id) => id,
                MemberIdentificationType::email(id) => id,
            };
            let hm = HashMap::from([(".tag", access.as_ref()), (access.as_ref(), id)]);
            let member = Member {
                access_level: ident.as_ref(),
                member: hm,
            };
            members.push(member);
        }

        Some(Payload {
            custom_message: self.custom_message,
            members,
            quiet: self.quiet,
            shared_folder_id: self.shared_folder_id,
        })
    }
}

implement_service!(
    AddFolderMemberRequest<'_>,
    AddFolderMemberResponse,
    Endpoint::AddFolderMemberPost,
    vec![Headers::ContentTypeAppJson]
);
