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
pub mod swagger_object;

#[derive(Debug)]
struct Field {
    name: String,
    ty: Vec<proc_macro2::TokenTree>,
}

pub use impl_swagger_trait::implements_swagger_trait;

pub trait JsonSchemaDefinition {
    fn get_json_schema_definition() -> serde_json::Value;
}
pub trait QueryDefinition {
    fn get_query_definition() -> serde_json::Value;
}

#[macro_export]
macro_rules! swagger_add_router {
    ($swagger_object:expr, "GET", $path:literal, $response:ident) => {
        $swagger_object.add_route(
            SwaggerMethod::GET,
            String::from($path),
            $response::get_json_schema_definition(),
        )
    };
}
