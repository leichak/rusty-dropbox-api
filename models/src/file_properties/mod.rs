mod properties_add;
mod properties_overwrite;
mod properties_remove;
mod properties_search;
mod properties_search_continue;
mod properties_update;
mod templates_add_for_user;
mod templates_get_for_user;
mod templates_list_for_user;
mod templates_remove_for_user;
mod templates_update_for_user;

use serde::{Deserialize, Serialize};

// pub structfor the first JSON
#[derive(Serialize, Deserialize, Debug)]
pub structField {
    name: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub structPropertyGroup {
    fields: Vec<Field>,
    template_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub structPathWithPropertyGroups {
    path: String,
    property_groups: Vec<PropertyGroup>,
}

// pub structfor the second JSON
#[derive(Serialize, Deserialize, Debug)]
pub structPathWithTemplateIds {
    path: String,
    property_template_ids: Vec<String>,
}

// pub structfor the third JSON
#[derive(Serialize, Deserialize, Debug)]
pub structMode {
    #[serde(rename = ".tag")]
    tag: String,
    field_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub structQuery {
    logical_operator: String,
    mode: Mode,
    query: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub structQueriesWithTemplateFilter {
    queries: Vec<Query>,
    template_filter: String,
}

// pub structfor the fourth JSON
#[derive(Serialize, Deserialize, Debug)]
pub structMatch {
    id: String,
    is_deleted: bool,
    path: String,
    property_groups: Vec<PropertyGroup>,
}

#[derive(Serialize, Deserialize, Debug)]
pub structMatchesWithPropertyGroups {
    matches: Vec<Match>,
}

// pub structfor the fifth JSON
#[derive(Serialize, Deserialize, Debug)]
pub structCursor {
    cursor: String,
}

// pub structfor the sixth JSON
// Same as MatchesWithPropertyGroups

// pub structfor the seventh JSON
#[derive(Serialize, Deserialize, Debug)]
pub structAddOrUpdateField {
    name: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub structUpdatePropertyGroup {
    add_or_update_fields: Vec<AddOrUpdateField>,
    remove_fields: Vec<String>,
    template_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub structPathWithUpdatePropertyGroups {
    path: String,
    update_property_groups: Vec<UpdatePropertyGroup>,
}

// pub structfor the eighth JSON
#[derive(Serialize, Deserialize, Debug)]
pub structFieldDescription {
    description: String,
    name: String,
    #[serde(rename = "type")]
    field_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub structPropertyTemplate {
    description: String,
    fields: Vec<FieldDescription>,
    name: String,
}

// pub structfor the ninth JSON
#[derive(Serialize, Deserialize, Debug)]
pub structTaggedFieldType {
    #[serde(rename = ".tag")]
    tag: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub structFieldWithTaggedType {
    description: String,
    name: String,
    #[serde(rename = "type")]
    field_type: TaggedFieldType,
}

#[derive(Serialize, Deserialize, Debug)]
pub structPropertyTemplateWithTaggedType {
    description: String,
    fields: Vec<FieldWithTaggedType>,
    name: String,
}

// pub structfor the tenth JSON
#[derive(Serialize, Deserialize, Debug)]
pub structTemplateIds {
    template_ids: Vec<String>,
}

// pub structfor the eleventh JSON
#[derive(Serialize, Deserialize, Debug)]
pub structAddField {
    description: String,
    name: String,
    #[serde(rename = "type")]
    field_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub structAddFieldsToTemplate {
    add_fields: Vec<AddField>,
    description: String,
    name: String,
    template_id: String,
}
