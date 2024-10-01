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
    name: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PropertyGroup {
    fields: Vec<Field>,
    template_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PathWithPropertyGroups {
    path: String,
    property_groups: Vec<PropertyGroup>,
}

// pub struct for the second JSON
#[derive(Serialize, Deserialize, Debug)]
pub struct PathWithTemplateIds {
    path: String,
    property_template_ids: Vec<String>,
}

// pub struct for the third JSON
#[derive(Serialize, Deserialize, Debug)]
pub struct Mode {
    #[serde(rename = ".tag")]
    tag: String,
    field_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Query {
    logical_operator: String,
    mode: Mode,
    query: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueriesWithTemplateFilter {
    queries: Vec<Query>,
    template_filter: String,
}

// pub struct for the fourth JSON
#[derive(Serialize, Deserialize, Debug)]
pub struct Match {
    id: String,
    is_deleted: bool,
    path: String,
    property_groups: Vec<PropertyGroup>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchesWithPropertyGroups {
    matches: Vec<Match>,
}

// pub struct for the fifth JSON
#[derive(Serialize, Deserialize, Debug)]
pub struct Cursor {
    cursor: String,
}

// pub struct for the sixth JSON
// Same as MatchesWithPropertyGroups

// pub struct for the seventh JSON
#[derive(Serialize, Deserialize, Debug)]
pub struct AddOrUpdateField {
    name: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePropertyGroup {
    add_or_update_fields: Vec<AddOrUpdateField>,
    remove_fields: Vec<String>,
    template_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PathWithUpdatePropertyGroups {
    path: String,
    update_property_groups: Vec<UpdatePropertyGroup>,
}

// pub struct for the eighth JSON
#[derive(Serialize, Deserialize, Debug)]
pub struct FieldDescription {
    description: String,
    name: String,
    #[serde(rename = "type")]
    field_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PropertyTemplate {
    description: String,
    fields: Vec<FieldDescription>,
    name: String,
}

// pub struct for the ninth JSON
#[derive(Serialize, Deserialize, Debug)]
pub struct TaggedFieldType {
    #[serde(rename = ".tag")]
    tag: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FieldWithTaggedType {
    description: String,
    name: String,
    #[serde(rename = "type")]
    field_type: TaggedFieldType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PropertyTemplateWithTaggedType {
    description: String,
    fields: Vec<FieldWithTaggedType>,
    name: String,
}

// pub struct for the tenth JSON
#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateIds {
    template_ids: Vec<String>,
}

// pub struct for the tenth JSON
#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateId {
    template_id: String,
}

// pub struct for the eleventh JSON
#[derive(Serialize, Deserialize, Debug)]
pub struct AddField {
    description: String,
    name: String,
    #[serde(rename = "type")]
    field_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddFieldsToTemplate {
    add_fields: Vec<AddField>,
    description: String,
    name: String,
    template_id: String,
}
