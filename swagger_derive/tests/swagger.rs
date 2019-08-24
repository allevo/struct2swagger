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

use swagger::swagger_object::SwaggerObject;
use swagger::JsonSchemaDefinition;

#[derive(Swagger)]
struct SimpleStruct {
    val1: u8,
    val2: String,
}

const TITLE: &str = "the title";
const VERSION: &str = "1.0.1";
const DESCRIPTION: &str = "the description";

#[test]
fn with_response() {
    let mut swagger_object = SwaggerObject::new(TITLE, VERSION);

    swagger_add_router!(swagger_object, "GET", "/", 200, DESCRIPTION, SimpleStruct);

    let stringified = serde_json::to_string(&swagger_object).unwrap();
    let values: serde_json::Value = serde_json::from_str(&stringified).unwrap();

    assert_eq!(
        values,
        json!({
            "openapi": "3.0.0",
            "info": {
                "title": TITLE,
                "version": VERSION,
            },
            "paths": {
                "/": {
                    "get": {
                        "responses": {
                            "200": {
                                "description": DESCRIPTION,
                                "content": {
                                    "application/json": {
                                        "schema": SimpleStruct::get_json_schema_definition(),
                                    },
                                },
                            },
                        },
                    },
                },
            },
        })
    );
}

#[test]
fn with_body() {
    let mut swagger_object = SwaggerObject::new(TITLE, VERSION);

    swagger_add_router!(
        swagger_object,
        "POST",
        "/",
        "request_body",
        SimpleStruct,
        200,
        DESCRIPTION,
        SimpleStruct
    );

    let stringified = serde_json::to_string(&swagger_object).unwrap();
    let values: serde_json::Value = serde_json::from_str(&stringified).unwrap();

    assert_eq!(
        values,
        json!({
            "openapi": "3.0.0",
            "info": {
                "title": TITLE,
                "version": VERSION,
            },
            "paths": {
                "/": {
                    "post": {
                        "requestBody": {
                            "content": {
                                "application/json": {
                                    "schema": SimpleStruct::get_json_schema_definition(),
                                },
                            },
                            "required":true,
                        },
                        "responses": {
                            "200": {
                                "description": DESCRIPTION,
                                "content": {
                                    "application/json": {
                                        "schema": SimpleStruct::get_json_schema_definition(),
                                    },
                                },
                            },
                        },
                    },
                },
            },
        })
    );
}
