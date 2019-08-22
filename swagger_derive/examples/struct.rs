#![allow(dead_code)]

extern crate swagger;
use swagger::JsonSchemaDefinition;
#[macro_use]
extern crate swagger_derive;

#[macro_use]
extern crate serde_json;

#[derive(Swagger)]
struct SimpleStruct {
    val1: u8,
    val2: String,
}


#[test]
fn simple_struct() {
    let t = SimpleStruct::get_schema_type();

    assert_eq!(t, json!({
        "id":"SimpleStruct",
        "properties": {
            "val1": {
                "maximum":255,
                "minimum":0,
                "type":"integer",
            },
            "val2": {
                "type":"string",
            }
        },
        "required":["val1","val2"],
        "type":"object",
    }));
}

#[derive(Swagger)]
struct StructWithArrays {
    val1: Vec<u8>,
    val2: std::vec::Vec<String>,
    val3: Vec<Vec<Vec<u8>>>,
}

#[test]
fn struct_with_arrays() {
    let t = StructWithArrays::get_schema_type();
    
    assert_eq!(t, json!({
        "id":"StructWithArrays",
        "properties": {
            "val1": {
                "type": "array",
                "items": {
                    "maximum":255,
                    "minimum":0,
                    "type":"integer",
                },
            },
            "val2": {
                "type": "array",
                "items": {
                    "type":"string",
                },
            },
            "val3": {
                "type": "array",
                "items": {
                    "type": "array",
                    "items": {
                        "type": "array",
                        "items": {
                            "maximum":255,
                            "minimum":0,
                            "type":"integer",
                        },
                    },
                },
            },
        },
        "required":["val1", "val2", "val3"],
        "type":"object",
    }));
}

#[derive(Swagger)]
struct StructWithOption {
    val1: u8,
    val2: Option<u8>,
    val3: Option<String>,
}

#[test]
fn struct_with_option() {
    let t = StructWithOption::get_schema_type();

    assert_eq!(t, json!({
        "id":"StructWithOption",
        "properties": {
            "val1": {
                "maximum":255,
                "minimum":0,
                "type":"integer",
            },
            "val2": {
                "maximum":255,
                "minimum":0,
                "type":"integer",
            },
            "val3": {
                "type":"string",
            },
        },
        "required":["val1"],
        "type":"object",
    }));
}


#[derive(Swagger)]
struct MySubStruct {
    val1: u8,
    val2: String,
}

#[derive(Swagger)]
struct StructWithStruct {
    val1: u8,
    val2: MySubStruct,
    val3: Vec<MySubStruct>,
}

#[test]
fn struct_with_struct() {
    let t = StructWithStruct::get_schema_type();

    assert_eq!(t, json!({
        "id":"StructWithStruct",
        "properties": {
            "val1": {
                "maximum":255,
                "minimum":0,
                "type":"integer",
            },
            "val2": {
                "id": "MySubStruct",
                "type": "object",
                "properties": {
                    "val1": {
                        "maximum":255,
                        "minimum":0,
                        "type":"integer",
                    },
                    "val2": {
                        "type": "string",
                    },
                },
                "required": ["val1", "val2"],
            },
            "val3": {
                "type":"array",
                "items": {
                    "id": "MySubStruct",
                    "type": "object",
                    "properties": {
                        "val1": {
                            "maximum":255,
                            "minimum":0,
                            "type":"integer",
                        },
                        "val2": {
                            "type": "string",
                        },
                    },
                    "required": ["val1", "val2"],
                },
            },
        },
        "required":["val1", "val2", "val3"],
        "type":"object",
    }));
}
