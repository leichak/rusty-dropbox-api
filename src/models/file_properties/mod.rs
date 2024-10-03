pub mod properties_add;
pub mod properties_overwrite;
pub mod properties_remove;
pub mod properties_search;
pub mod properties_search_continue;
pub mod properties_update;
pub mod templates_add_for_user;
pub mod templates_get_for_user;
pub mod templates_list_for_user;
pub mod templates_remove_for_user;
pub mod templates_update_for_user;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PropertyGroup {
    pub fields: Vec<Field>,
    pub template_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PathWithPropertyGroupsArgs {
    pub path: String,
    pub property_groups: Vec<PropertyGroup>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PathWithTemplateIdsArgs {
    pub path: String,
    pub property_template_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mode {
    #[serde(rename = ".tag")]
    pub tag: String,
    pub field_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Query {
    logical_operator: String,
    pub mode: Mode,
    pub query: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueriesWithTemplateFilterArgs {
    pub queries: Vec<Query>,
    pub template_filter: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Match {
    pub id: String,
    pub is_deleted: bool,
    pub path: String,
    pub property_groups: Vec<PropertyGroup>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchesWithPropertyGroupsResult {
    pub matches: Vec<Match>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CursorArgs {
    pub cursor: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddOrUpdateField {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePropertyGroup {
    pub add_or_update_fields: Vec<AddOrUpdateField>,
    pub remove_fields: Vec<String>,
    pub template_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PathWithUpdatePropertyGroupsArgs {
    pub path: String,
    pub update_property_groups: Vec<UpdatePropertyGroup>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FieldDescription {
    pub description: String,
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PropertyTemplateArgs {
    pub description: String,
    pub fields: Vec<FieldDescription>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaggedFieldType {
    #[serde(rename = ".tag")]
    pub tag: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FieldWithTaggedType {
    pub description: String,
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: TaggedFieldType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PropertyTemplateWithTaggedTypeResult {
    pub description: String,
    pub fields: Vec<FieldWithTaggedType>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateIdsResult {
    pub template_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateIdResult {
    pub template_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateIdArgs {
    pub template_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddField {
    pub description: String,
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddFieldsToTemplateArgs {
    pub add_fields: Vec<AddField>,
    pub description: String,
    pub name: String,
    pub template_id: String,
}
