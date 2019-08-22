
use std::collections::HashMap;

use serde::{Serialize, Serializer};
use serde_json::Value;

pub enum SwaggerVersion {
    V2,
}
impl Serialize for SwaggerVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(match *self {
            SwaggerVersion::V2 => "2.0",
        })
    }
}

#[derive(Serialize)]
pub struct InfoObject {
    description: String,
    version: String,
    title: String,
}

#[derive(Serialize)]
pub struct TagObject {
    name: String,
    description: String,
}

pub type SchemaObject = serde_json::Value;

#[derive(Clone)]
#[derive(Serialize)]
pub struct ResponseObject {
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<SchemaObject>,
}

#[derive(Clone)]
#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterIn {
    Path,
    Body,
    Query,
}

#[derive(Clone)]
#[derive(Serialize)]
pub struct ParameterObject {
    r#in: ParameterIn,
    name: String,
    description: String,
    r#type: String,
    required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<SchemaObject>,
}

#[derive(Clone)]
#[derive(Serialize)]
pub struct MethodObject {
    tags: Vec<String>,
    summary: String,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    consumes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    produces: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    parameters: Vec<ParameterObject>,
    responses: HashMap<u8, ResponseObject>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SwaggerMethod {
    GET,
    POST,
}
impl Serialize for SwaggerMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(match *self {
            SwaggerMethod::GET => "get",
            SwaggerMethod::POST => "post",
        })
    }
}

pub type PathObject = HashMap<SwaggerMethod, MethodObject>;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwaggerObject {
    swagger: SwaggerVersion,
    info: InfoObject,
    host: String,
    base_path: String,
    tags: Vec<TagObject>,
    schemes: Vec<String>,
    paths: HashMap<String, PathObject>
}
