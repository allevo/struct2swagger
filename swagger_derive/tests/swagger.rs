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
    SwaggerMethod,
};

#[derive(Swagger)]
struct SimpleStruct {
    val1: u8,
    val2: String,
}

#[test]
fn simple_struct() {
    let mut swagger_object = SwaggerObject {
        swagger: SwaggerVersion::V2,
        info: InfoObject {
            description: "My desc".to_owned(),
            version: "1.1.1".to_owned(),
            title: "the swagger".to_owned(),
        },
        host: "localhost".to_owned(),
        base_path: "/".to_owned(),
        tags: vec![],
        schemes: vec![ "http".to_owned() ],
        paths: HashMap::new(),
    };

    swagger_add_router!(swagger_object, "GET", "/", SimpleStruct);

    let stringified = serde_json::to_string(&swagger_object).unwrap();
    let values: serde_json::Value = serde_json::from_str(&stringified).unwrap();

    assert_eq!(values, json!({
        "swagger":"2.0",
        "info": {
            "description": "My desc",
            "version": "1.1.1",
            "title": "the swagger",
        },
        "host": "localhost",
        "basePath": "/",
        "tags": [],
        "schemes": ["http"],
        "paths": {
            "/": {
                "get": {
                    "tags": [],
                    "summary":"",
                    "description":"",
                    "responses": {
                        "200": {
                            "description": "",
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
                                "required":["val1","val2"],
                                "type":"object",
                            },
                        },
                    },
                },
            },
        },
    }));
}
