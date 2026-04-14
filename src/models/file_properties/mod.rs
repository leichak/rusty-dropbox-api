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

// =============================================================================
// Type names follow the Dropbox Stone spec
// (`dropbox-api-spec/file_properties.stone`).
// =============================================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct PropertyField {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PropertyGroup {
    pub fields: Vec<PropertyField>,
    pub template_id: String,
}

/// `AddPropertiesArg` and `OverwritePropertyGroupArg` have identical wire
/// shape per spec; Dropbox names them separately for documentation. Both
/// names are exposed via a type alias.
#[derive(Serialize, Deserialize, Debug)]
pub struct AddPropertiesArg {
    pub path: String,
    pub property_groups: Vec<PropertyGroup>,
}

pub type OverwritePropertyGroupArg = AddPropertiesArg;

#[derive(Serialize, Deserialize, Debug)]
pub struct RemovePropertiesArg {
    pub path: String,
    pub property_template_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PropertiesSearchMode {
    #[serde(rename = ".tag")]
    pub tag: String,
    pub field_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PropertiesSearchQuery {
    pub logical_operator: String,
    pub mode: PropertiesSearchMode,
    pub query: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PropertiesSearchArg {
    pub queries: Vec<PropertiesSearchQuery>,
    pub template_filter: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PropertiesSearchMatch {
    pub id: String,
    pub is_deleted: bool,
    pub path: String,
    pub property_groups: Vec<PropertyGroup>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PropertiesSearchResult {
    pub matches: Vec<PropertiesSearchMatch>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PropertiesSearchContinueArg {
    pub cursor: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PropertyGroupUpdate {
    pub add_or_update_fields: Vec<PropertyField>,
    pub remove_fields: Vec<String>,
    pub template_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePropertiesArg {
    pub path: String,
    pub update_property_groups: Vec<PropertyGroupUpdate>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PropertyFieldTemplate {
    pub description: String,
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddTemplateArg {
    pub description: String,
    pub fields: Vec<PropertyFieldTemplate>,
    pub name: String,
}

/// Wire shape `{".tag": "string"}`. The tag-only union has one named
/// variant in spec (`string`); modelled as a struct for forward compat.
#[derive(Serialize, Deserialize, Debug)]
pub struct PropertyType {
    #[serde(rename = ".tag")]
    pub tag: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PropertyFieldTemplateTagged {
    pub description: String,
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: PropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTemplateResult {
    pub description: String,
    pub fields: Vec<PropertyFieldTemplateTagged>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListTemplateResult {
    pub template_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddTemplateResult {
    pub template_id: String,
}

pub type UpdateTemplateResult = AddTemplateResult;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTemplateArg {
    pub template_id: String,
}

pub type RemoveTemplateArg = GetTemplateArg;

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTemplateArg {
    pub add_fields: Vec<PropertyFieldTemplate>,
    pub description: String,
    pub name: String,
    pub template_id: String,
}
