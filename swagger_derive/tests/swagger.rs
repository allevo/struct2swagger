#![allow(dead_code)]

use std::collections::HashMap;

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate swagger;
#[macro_use]
extern crate swagger_derive;

use swagger::{
    JsonSchemaDefinition,
};
use swagger::swagger_object::{
    SwaggerObject,
    SwaggerVersion,
    InfoObject,
    MediaTypeObject,
    RequestBodyObject,
    SchemaObjectOrReferenceObject,
};

#[derive(Swagger)]
struct SimpleStruct {
    val1: u8,
    val2: String,
}

#[test]
fn with_response() {
    let mut swagger_object = SwaggerObject {
        openapi: SwaggerVersion::V300,
        info: InfoObject {
            title: "the swagger".to_owned(),
            version: "1.1.1".to_owned(),
            description: None,
            terms_of_service: None,
            contact: None,
            license: None,
        },
        servers: None,
        paths: HashMap::new(),
        components: None,
        security: None,
        tags: None,
        external_docs: None,
    };

    swagger_add_router!(swagger_object, "GET", "/", 200, SimpleStruct);

    let stringified = serde_json::to_string(&swagger_object).unwrap();
    let values: serde_json::Value = serde_json::from_str(&stringified).unwrap();

    assert_eq!(values, json!({
        "openapi": "3.0.0",
        "info": {
            "title": "the swagger",
            "version": "1.1.1",
        },
        "paths": {
            "/": {
                "get": {
                    "responses": {
                        "200": {
                            "description": "",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "val1": {
                                                "maximum": 255,
                                                "minimum": 0,
                                                "type": "integer",
                                            },
                                            "val2": {
                                                "type": "string",
                                            },
                                        },
                                        "required": [ "val1", "val2" ],
                                    },
                                },
                            },
                        },
                    },
                },
            },
        },
    }));
}


#[test]
fn with_body() {
    let mut swagger_object = SwaggerObject {
        openapi: SwaggerVersion::V300,
        info: InfoObject {
            title: "the swagger".to_owned(),
            version: "1.1.1".to_owned(),
            description: None,
            terms_of_service: None,
            contact: None,
            license: None,
        },
        servers: None,
        paths: HashMap::new(),
        components: None,
        security: None,
        tags: None,
        external_docs: None,
    };

    swagger_add_router!(swagger_object, "POST", "/", "request_body", SimpleStruct, 200, SimpleStruct);

    let stringified = serde_json::to_string(&swagger_object).unwrap();
    let values: serde_json::Value = serde_json::from_str(&stringified).unwrap();

    assert_eq!(values, json!({
        "openapi": "3.0.0",
        "info": {
            "title": "the swagger",
            "version": "1.1.1",
        },
        "paths": {
            "/": {
                "post": {
                    "requestBody": {
                        "content": {
                            "application/json": {
                                "schema": {
                                    "properties": {
                                        "val1": {
                                            "maximum": 255,
                                            "minimum": 0,
                                            "type": "integer",
                                        },
                                        "val2": {
                                            "type": "string",
                                        },
                                    },
                                    "required": ["val1","val2"],
                                    "type":"object",
                                },
                            },
                        },
                        "required":true,
                    },
                    "responses": {
                        "200": {
                            "description": "",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "properties": {
                                            "val1": {
                                                "maximum": 255,
                                                "minimum": 0,
                                                "type": "integer",
                                            },
                                            "val2": {
                                                "type": "string",
                                            },
                                        },
                                        "required": ["val1","val2"],
                                        "type": "object",
                                    },
                                },
                            },
                        },
                    },
                },
            },
        },
    }));
}
