use std::collections::HashMap;

use serde::{Serialize, Serializer};

pub enum SwaggerVersion {
    V2,
}
impl Serialize for SwaggerVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match *self {
            SwaggerVersion::V2 => "2.0",
        })
    }
}

#[derive(Serialize)]
pub struct InfoObject {
    pub description: String,
    pub version: String,
    pub title: String,
}

#[derive(Serialize)]
pub struct TagObject {
    pub name: String,
    pub description: String,
}

pub type SchemaObject = serde_json::Value;

#[derive(Clone, Serialize)]
pub struct ResponseObject {
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<SchemaObject>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterIn {
    Path,
    Body,
    Query,
}

#[derive(Clone, Serialize)]
pub struct ParameterObject {
    pub r#in: ParameterIn,
    pub name: String,
    pub description: String,
    pub r#type: String,
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<SchemaObject>,
}

#[derive(Clone, Serialize)]
pub struct MethodObject {
    pub tags: Vec<String>,
    pub summary: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub produces: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ParameterObject>,
    pub responses: HashMap<u8, ResponseObject>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SwaggerMethod {
    GET,
    POST,
}
impl Serialize for SwaggerMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
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
    pub swagger: SwaggerVersion,
    pub info: InfoObject,
    pub host: String,
    pub base_path: String,
    pub tags: Vec<TagObject>,
    pub schemes: Vec<String>,
    pub paths: HashMap<String, PathObject>,
}

impl SwaggerObject {
    pub fn add_route(
        self: &mut Self,
        method: SwaggerMethod,
        path: String,
        responses: Vec<(u8, (&str, serde_json::Value))>,
    ) {
        if !self.paths.contains_key(&path) {
            self.paths.insert(path.clone(), PathObject::new());
        }
        let path_object = self.paths.get_mut(&path).unwrap();

        if path_object.contains_key(&method) {
            unimplemented!("Please send a PR!");
        }

        let mut swagger_responses: HashMap<u8, ResponseObject> = HashMap::new();

        for (status_code, (description, schema)) in responses {
            let dd: String = description.to_string();
            swagger_responses.insert(
                status_code,
                ResponseObject {
                    description: dd,
                    schema: Some(schema.clone()),
                },
            );
        }

        let method_object = MethodObject {
            tags: vec![],
            summary: "".to_owned(),
            description: "".to_owned(),
            consumes: None,
            produces: None,
            parameters: vec![],
            responses: swagger_responses,
        };

        path_object.insert(method.clone(), method_object);
    }
}
