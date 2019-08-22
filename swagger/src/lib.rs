#![recursion_limit = "128"]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod impl_data_types;
mod impl_swagger_trait;
mod swagger_object;

pub use impl_swagger_trait::implements_swagger_trait;

#[derive(Debug)]
struct Field {
    name: String,
    ty: Vec<proc_macro2::TokenTree>,
}

pub trait JsonSchemaDefinition {
    fn get_schema_type() -> serde_json::Value;
}
