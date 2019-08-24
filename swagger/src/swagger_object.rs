use std::collections::HashMap;

use serde::{Serialize, Serializer};

#[derive(Debug, Clone)]
pub enum SwaggerVersion {
    V300,
}
impl Serialize for SwaggerVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match *self {
            SwaggerVersion::V300 => "3.0.0",
        })
    }
}

type HttpStatusCode = u16;

#[derive(Serialize, Debug, Clone)]
pub struct ContactObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct LicenseObject {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ServerVariableObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<String>>,
    pub r#default: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ServerObject {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, ServerVariableObject>>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InfoObject {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<ContactObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<LicenseObject>,
    pub version: String,
}

macro_rules! or_reference {
    ($name: ident, $ty: ident) => {
        #[derive(Serialize, Debug, Clone)]
        #[serde(untagged)]
        pub enum $name {
            $ty(Box<$ty>),
            ReferenceObject(ReferenceObject),
        }
    };
}

or_reference!(SchemaObjectOrReferenceObject, SchemaObject);
or_reference!(ResponseObjectOrReferenceObject, ResponseObject);
or_reference!(ParameterObjectOrReferenceObject, ParameterObject);
or_reference!(ExampleObjectOrReferenceObject, ExampleObject);
or_reference!(RequestBodyObjectOrReferenceObject, RequestBodyObject);
or_reference!(HeaderObjectOrReferenceObject, HeaderObject);
or_reference!(SecuritySchemeObjectOrReferenceObject, SecuritySchemeObject);
or_reference!(LinkObjectOrReferenceObject, LinkObject);
or_reference!(CallbackObjectOrReferenceObject, CallbackObject);

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum AnyOrExpression {
    Any(serde_json::Value),
    Expression(String),
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ComponentsObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<HashMap<String, SchemaObjectOrReferenceObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<HashMap<String, ResponseObjectOrReferenceObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, ParameterObjectOrReferenceObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<HashMap<String, ExampleObjectOrReferenceObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_bodies: Option<HashMap<String, RequestBodyObjectOrReferenceObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, HeaderObjectOrReferenceObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_schemes: Option<HashMap<String, SecuritySchemeObjectOrReferenceObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<HashMap<String, LinkObjectOrReferenceObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callbacks: Option<HashMap<String, CallbackObjectOrReferenceObject>>,
}

pub type PathsObject = HashMap<String, PathItemObject>;

#[derive(Serialize, Debug, Clone)]
pub struct PathItemObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<ServerObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ParameterObjectOrReferenceObject>>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OperationObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterObjectOrReferenceObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<RequestBodyObjectOrReferenceObject>,
    responses: ResponsesObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callbacks: Option<HashMap<String, CallbackObjectOrReferenceObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<SecurityRequirementObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<ServerObject>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ExternalDocumentationObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub url: String,
}

#[derive(Clone, Serialize, Debug)]
#[serde(rename_all = "lowercase", untagged)]
pub enum ParameterIn {
    Query,
    Header,
    Path,
    Cookie,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ParameterObject {
    pub name: String,
    pub r#in: ParameterIn,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_empty_value: Option<bool>,
}

#[derive(Serialize, Debug, Clone)]
pub struct RequestBodyObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub content: HashMap<String, MediaTypeObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

#[derive(Serialize, Debug, Clone)]
pub struct MediaTypeObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<SchemaObjectOrReferenceObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<HashMap<String, ExampleObjectOrReferenceObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<HashMap<String, EncodingObject>>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EncodingObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, HeaderObjectOrReferenceObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_reserved: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct ResponsesObject {
    pub default: Option<ResponseObjectOrReferenceObject>,
    pub responses_per_http_status_codes: Option<HashMap<HttpStatusCode, ResponseObjectOrReferenceObject>>,
}
impl Serialize for ResponsesObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut r: HashMap<String, ResponseObjectOrReferenceObject> = HashMap::new();
        if self.responses_per_http_status_codes.is_some() {
            for (k, value) in self.responses_per_http_status_codes.as_ref().unwrap().iter() {
                r.insert(k.to_string().to_owned(), value.clone());
            }
        }
        if self.default.is_some() {
            r.insert("default".to_owned(), self.default.as_ref().unwrap().clone());
        }

        r.serialize(serializer)
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct ResponseObject {
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, HeaderObjectOrReferenceObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<HashMap<String, MediaTypeObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<HashMap<String, LinkObjectOrReferenceObject>>,
}

type CallbackObject = HashMap<String, PathItemObject>;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExampleObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_value: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LinkObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, AnyOrExpression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<AnyOrExpression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<ServerObject>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HeaderObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_empty_value: Option<bool>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TagObject {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentationObject>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ReferenceObject {
    pub r#ref: String,
}

type SchemaObject = serde_json::Value;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiscriminatorObject {
    pub property_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapping: Option<HashMap<String, String>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct XMLObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrapped: Option<bool>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SecuritySchemeObject {
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub name: String,
    pub r#in: ParameterIn,
    pub scheme: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearer_format: Option<String>,
    pub flows: OAuthFlowsObject,
    pub open_id_connect_url: String,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OAuthFlowsObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit: Option<OAuthFlowObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<OAuthFlowObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_credentials: Option<OAuthFlowObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<OAuthFlowObject>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OAuthFlowObject {
    pub authorization_url: String,
    pub token_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    pub scopes: HashMap<String, String>,
}

type SecurityRequirementObject = HashMap<String, String>;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SwaggerObject {
    pub openapi: SwaggerVersion,
    pub info: InfoObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<ServerObject>>,
    pub paths: PathsObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<ComponentsObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<SecurityRequirementObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentationObject>,
}


impl SwaggerObject {
    pub fn add_route(
        self: &mut Self,
        method: &str,
        path: String,
        _parameters: Option<Vec<ParameterObject>>,
        request_body: Option<RequestBodyObject>,
        responses: Vec<(HttpStatusCode, (&str, serde_json::Value))>,
    ) {
        if !self.paths.contains_key(&path) {
            self.paths.insert(path.clone(), PathItemObject {
                r#ref: None,
                summary: None,
                description: None,
                get: None,
                put: None,
                post: None,
                delete: None,
                options: None,
                head: None,
                patch: None,
                trace: None,
                servers: None,
                parameters: None,
            });
        }
        let path_object = self.paths.get_mut(&path).unwrap();

        let mut responses_per_http_status_codes = HashMap::new();
        for (status_code, (description, value)) in responses {
            let mut content_map = HashMap::new();
            content_map.insert("application/json".to_owned(), MediaTypeObject {
                schema: Some(SchemaObjectOrReferenceObject::SchemaObject(Box::new(value))),
                example: None,
                examples: None,
                encoding: None,
            });
            responses_per_http_status_codes.insert(status_code, ResponseObjectOrReferenceObject::ResponseObject(Box::new(ResponseObject {
                description: description.to_owned(),
                headers: None,
                content: Some(content_map),
                links: None,
            })));
        }

        let request_body = match request_body {
            Some(rq) => Some(RequestBodyObjectOrReferenceObject::RequestBodyObject(Box::new(rq))),
            None => None,
        };
        let operation_object = OperationObject {
            responses: ResponsesObject {
                default: None,
                responses_per_http_status_codes: Some(responses_per_http_status_codes),
            },
            tags: None,
            summary: None,
            description: None,
            external_docs: None,
            operation_id: None,
            parameters: None,
            request_body,
            callbacks: None,
            deprecated: None,
            security: None,
            servers: None,
        };

        match method {
            "GET" => path_object.get = Some(operation_object),
            "POST" => path_object.post = Some(operation_object),
            _ => unimplemented!("Unknown method: Send a PR!"),
        }
        
    }
}

