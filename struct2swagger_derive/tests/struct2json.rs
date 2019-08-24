#![allow(dead_code)]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate quote;
use proc_macro2::{Ident, Punct, Spacing, Span, TokenTree};
use std::iter::FromIterator;

extern crate struct2swagger;
#[macro_use]
extern crate struct2swagger_derive;

use struct2swagger::JsonSchemaDefinition;

#[test]
fn test_1() {
    let call_ident1 = Ident::new("a", Span::call_site());
    let call_ident2 = Ident::new("gg", Span::call_site());
    let call_ident3: Option<Ident> = None;

    let q = quote! {
        #call_ident1::#call_ident2#call_ident3();
    };

    let s = q.to_string();

    assert_eq!(s, "a :: gg ( ) ;");
}

#[test]
fn test_2() {
    let mut tokens = Vec::new();

    tokens.push(TokenTree::Punct(Punct::new('(', Spacing::Alone)));
    tokens.push(TokenTree::Punct(Punct::new('{', Spacing::Alone)));

    tokens.push(TokenTree::Punct(Punct::new('}', Spacing::Alone)));
    tokens.push(TokenTree::Punct(Punct::new(')', Spacing::Alone)));

    let tokens = proc_macro2::TokenStream::from_iter(tokens.into_iter());
    let q = quote! {
        impl JsonSchemaDefinition for MyStructName {
            fn get_json_schema_definition() -> serde_json::Value {
                json! #tokens;
            }
        }
    };
    let s = q.to_string();

    assert_eq!(s, "impl JsonSchemaDefinition for MyStructName { fn get_json_schema_definition ( ) -> serde_json :: Value { json ! ( { } ) ; } }");
}

#[test]
fn test_3() {
    let q = quote! {
        Value::Bool(true)
    };

    let mut iter = q.into_iter();

    assert_eq!(iter.next().unwrap().to_string(), "Value");
    assert_eq!(iter.next().unwrap().to_string(), ":");
    assert_eq!(iter.next().unwrap().to_string(), ":");
    assert_eq!(iter.next().unwrap().to_string(), "Bool");
}

#[derive(Swagger)]
struct SimpleStruct {
    val1: u8,
    val2: String,
}

#[test]
fn simple_struct() {
    let t = SimpleStruct::get_json_schema_definition();

    assert_eq!(
        t,
        json!({
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
        })
    );
}

#[derive(Swagger)]
struct StructWithArrays {
    val1: Vec<u8>,
    val2: std::vec::Vec<String>,
    val3: Vec<Vec<Vec<u8>>>,
}

#[test]
fn struct_with_arrays() {
    let t = StructWithArrays::get_json_schema_definition();

    assert_eq!(
        t,
        json!({
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
        })
    );
}

#[derive(Swagger)]
struct StructWithOption {
    val1: u8,
    val2: Option<u8>,
    val3: Option<String>,
}

#[test]
fn struct_with_option() {
    let t = StructWithOption::get_json_schema_definition();

    assert_eq!(
        t,
        json!({
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
        })
    );
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
    let t = StructWithStruct::get_json_schema_definition();

    assert_eq!(
        t,
        json!({
            "properties": {
                "val1": {
                    "maximum":255,
                    "minimum":0,
                    "type":"integer",
                },
                "val2": {
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
        })
    );
}
