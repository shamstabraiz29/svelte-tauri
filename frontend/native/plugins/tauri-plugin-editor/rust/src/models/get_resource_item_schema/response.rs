use std::collections::HashMap;

use common_dtos::editor_client::types::resource_item_schema::{
    DateValidationInfo, EmailValidationInfo, MapDefinitionSource, NumberValidationInfo,
    RangeValidationInfo, ResourceItemSchema, ResourceNodeMultipleConfig, ResourceNodeSchema,
    ResourcePropertySchema, ResourceRelationshipSchema, TelValidationInfo, TextValidationInfo,
    TimeValidationInfo, UrlValidationInfo, ValueType,
};
use serde::Serialize;
use specta::Type;

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GetResourceItemSchemaResponse {
    #[serde(rename = "reqId")]
    pub req_id: String,
    pub schema: ResourceItemSchemaResponseData,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ResourceItemSchemaResponseData {
    Schema(ResourceItemSchemaDto),
    NoneFound,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ResourcePropertySchemaDto {
    pub value_type: ValueTypeDto,
    pub required: bool,
    pub multiple: Option<ResourceNodeMultipleConfigDto>,
    pub valid_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ResourceNodeMultipleConfigDto {
    pub repeated_values: bool,
    pub min_count: u32,
    pub max_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(tag = "type")]
pub enum ValueTypeDto {
    Text(TextValidationInfoDto),
    Number(NumberValidationInfoDto),
    Email(EmailValidationInfoDto),
    Date(DateValidationInfoDto),
    Time(TimeValidationInfoDto),
    Url(UrlValidationInfoDto),
    Tel(TelValidationInfoDto),
    Range(RangeValidationInfoDto),
    Map(MapDefinitionSourceDto),
    Boolean,
    Color,
    Json,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct TextValidationInfoDto {
    pub min_length: Option<u32>,
    pub max_length: Option<u32>,
    pub pattern: Option<String>,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct NumberValidationInfoDto {
    pub min: Option<f32>,
    pub max: Option<f32>,
    pub step: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct EmailValidationInfoDto {
    pub pattern: Option<String>,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct DateValidationInfoDto {
    pub min: Option<String>,
    pub max: Option<String>,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct TimeValidationInfoDto {
    pub min: Option<u32>,
    pub max: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct UrlValidationInfoDto {
    pub pattern: Option<String>,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct TelValidationInfoDto {
    pub pattern: Option<String>,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct RangeValidationInfoDto {
    pub min: Option<f32>,
    pub max: Option<f32>,
    pub step: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct MapDefinitionSourceDto {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ResourceNodeSchemaDto {
    pub r#type: String,
    pub cursor_icon_url: String,
    pub labels: Vec<String>,
    pub properties: HashMap<String, ResourcePropertySchemaDto>,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ResourceRelationshipSchemaDto {
    pub r#type: String,
    pub labels: Vec<String>,
    pub cardinality: String,
    pub properties: Vec<ResourcePropertySchemaDto>,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase", tag = "itemType")]
pub enum ResourceItemSchemaDto {
    Node(ResourceNodeSchemaDto),
    Relationship(ResourceRelationshipSchemaDto),
}

// Implement From for ResourceItemSchema
impl From<ResourceItemSchema> for ResourceItemSchemaDto {
    fn from(item_schema: ResourceItemSchema) -> Self {
        match item_schema {
            ResourceItemSchema::Node(node_schema) => {
                ResourceItemSchemaDto::Node(node_schema.into())
            }
            ResourceItemSchema::Relationship(rel_schema) => {
                ResourceItemSchemaDto::Relationship(rel_schema.into())
            }
        }
    }
}

// Implement From for ResourceNodeSchema
impl From<ResourceNodeSchema> for ResourceNodeSchemaDto {
    fn from(node_schema: ResourceNodeSchema) -> Self {
        ResourceNodeSchemaDto {
            r#type: node_schema.r#type,
            cursor_icon_url: node_schema.cursor_icon_url,
            labels: node_schema.labels,
            properties: node_schema
                .properties
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

// Implement From for ResourceRelationshipSchema
impl From<ResourceRelationshipSchema> for ResourceRelationshipSchemaDto {
    fn from(relationship: ResourceRelationshipSchema) -> Self {
        ResourceRelationshipSchemaDto {
            r#type: relationship.r#type,
            labels: relationship.labels,
            cardinality: relationship.cardinality,
            properties: relationship
                .properties
                .into_iter()
                .map(|p| p.into())
                .collect(),
        }
    }
}

// Implement From for ResourcePropertySchema
impl From<ResourcePropertySchema> for ResourcePropertySchemaDto {
    fn from(property: ResourcePropertySchema) -> Self {
        ResourcePropertySchemaDto {
            value_type: property.value_type.into(),
            required: property.required,
            multiple: property.multiple.map(|m| m.into()),
            valid_values: property.valid_values,
        }
    }
}

// Implement From for ResourceNodeMultipleConfig
impl From<ResourceNodeMultipleConfig> for ResourceNodeMultipleConfigDto {
    fn from(config: ResourceNodeMultipleConfig) -> Self {
        ResourceNodeMultipleConfigDto {
            repeated_values: config.repeated_values,
            min_count: config.min_count,
            max_count: config.max_count,
        }
    }
}

// Implement From for ValueType
impl From<ValueType> for ValueTypeDto {
    fn from(value_type: ValueType) -> Self {
        match value_type {
            ValueType::Text(info) => ValueTypeDto::Text(info.into()),
            ValueType::Number(info) => ValueTypeDto::Number(info.into()),
            ValueType::Email(info) => ValueTypeDto::Email(info.into()),
            ValueType::Date(info) => ValueTypeDto::Date(info.into()),
            ValueType::Time(info) => ValueTypeDto::Time(info.into()),
            ValueType::Url(info) => ValueTypeDto::Url(info.into()),
            ValueType::Tel(info) => ValueTypeDto::Tel(info.into()),
            ValueType::Range(info) => ValueTypeDto::Range(info.into()),
            ValueType::Map(info) => ValueTypeDto::Map(info.into()),
            ValueType::Boolean => ValueTypeDto::Boolean,
            ValueType::Color => ValueTypeDto::Color,
            ValueType::Json => ValueTypeDto::Json,
        }
    }
}

// Implement From for TextValidationInfo
impl From<TextValidationInfo> for TextValidationInfoDto {
    fn from(info: TextValidationInfo) -> Self {
        TextValidationInfoDto {
            min_length: info.min_length,
            max_length: info.max_length,
            pattern: info.pattern,
        }
    }
}

// Implement From for NumberValidationInfo
impl From<NumberValidationInfo> for NumberValidationInfoDto {
    fn from(info: NumberValidationInfo) -> Self {
        NumberValidationInfoDto {
            min: info.min,
            max: info.max,
            step: info.step,
        }
    }
}

// Implement From for EmailValidationInfo
impl From<EmailValidationInfo> for EmailValidationInfoDto {
    fn from(info: EmailValidationInfo) -> Self {
        EmailValidationInfoDto {
            pattern: info.pattern,
        }
    }
}

// Implement From for DateValidationInfo
impl From<DateValidationInfo> for DateValidationInfoDto {
    fn from(info: DateValidationInfo) -> Self {
        DateValidationInfoDto {
            min: info.min,
            max: info.max,
        }
    }
}

// Implement From for TimeValidationInfo
impl From<TimeValidationInfo> for TimeValidationInfoDto {
    fn from(info: TimeValidationInfo) -> Self {
        TimeValidationInfoDto {
            min: info.min,
            max: info.max,
        }
    }
}

// Implement From for UrlValidationInfo
impl From<UrlValidationInfo> for UrlValidationInfoDto {
    fn from(info: UrlValidationInfo) -> Self {
        UrlValidationInfoDto {
            pattern: info.pattern,
        }
    }
}

// Implement From for TelValidationInfo
impl From<TelValidationInfo> for TelValidationInfoDto {
    fn from(info: TelValidationInfo) -> Self {
        TelValidationInfoDto {
            pattern: info.pattern,
        }
    }
}

// Implement From for RangeValidationInfo
impl From<RangeValidationInfo> for RangeValidationInfoDto {
    fn from(info: RangeValidationInfo) -> Self {
        RangeValidationInfoDto {
            min: info.min,
            max: info.max,
            step: info.step,
        }
    }
}

// Implement From for MapDefinitionSource
impl From<MapDefinitionSource> for MapDefinitionSourceDto {
    fn from(source: MapDefinitionSource) -> Self {
        MapDefinitionSourceDto { name: source.name }
    }
}
