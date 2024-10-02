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

// pub struct for the first JSON
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
pub struct PathWithPropertyGroups {
    pub path: String,
    pub property_groups: Vec<PropertyGroup>,
}

// pub struct for the second JSON
#[derive(Serialize, Deserialize, Debug)]
pub struct PathWithTemplateIds {
    pub path: String,
    pub property_template_ids: Vec<String>,
}

// pub struct for the third JSON
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
pub struct QueriesWithTemplateFilter {
    pub queries: Vec<Query>,
    pub template_filter: String,
}

// pub struct for the fourth JSON
#[derive(Serialize, Deserialize, Debug)]
pub struct Match {
    pub id: String,
    pub is_deleted: bool,
    pub path: String,
    pub property_groups: Vec<PropertyGroup>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchesWithPropertyGroups {
    pub matches: Vec<Match>,
}

// pub struct for the fifth JSON
#[derive(Serialize, Deserialize, Debug)]
pub struct Cursor {
    pub cursor: String,
}

// pub struct for the seventh JSON
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
pub struct PathWithUpdatePropertyGroups {
    pub path: String,
    pub update_property_groups: Vec<UpdatePropertyGroup>,
}

// pub struct for the eighth JSON
#[derive(Serialize, Deserialize, Debug)]
pub struct FieldDescription {
    pub description: String,
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PropertyTemplate {
    pub description: String,
    pub fields: Vec<FieldDescription>,
    pub name: String,
}

// pub struct for the ninth JSON
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
pub struct PropertyTemplateWithTaggedType {
    pub description: String,
    pub fields: Vec<FieldWithTaggedType>,
    pub name: String,
}

// pub struct for the tenth JSON
#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateIds {
    pub template_ids: Vec<String>,
}

// pub struct for the tenth JSON
#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateId {
    pub template_id: String,
}

// pub struct for the eleventh JSON
#[derive(Serialize, Deserialize, Debug)]
pub struct AddField {
    pub description: String,
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddFieldsToTemplate {
    pub add_fields: Vec<AddField>,
    pub description: String,
    pub name: String,
    pub template_id: String,
}
