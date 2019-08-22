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

#[cfg(test)]
mod tests {
    use super::*;
    /*
    #[test]
    fn get_fields_should_return_the_right_fields () {
        let expanded = quote!(
            struct MyStructName1 {
                val1: u8,
                val2: String,
                val3: Vec<u8>,
                val4: Vec<String>,
                val5: std::Vec<u8>,
            }
        );
        let input = TokenStream::from(expanded);
        let ast = syn::parse2(input).unwrap();
        let fields = get_fields(ast);

        assert_eq!(fields, vec![
            Field {
                name: "val1".to_owned(),
                field_type: "u8".to_owned(),
                generic_type: None,
            },
            Field {
                name: "val2".to_owned(),
                field_type: "String".to_owned(),
                generic_type: None,
            },
            Field {
                name: "val3".to_owned(),
                field_type: "Vec".to_owned(),
                generic_type: Some("u8".to_owned()),
            },
            Field {
                name: "val4".to_owned(),
                field_type: "Vec".to_owned(),
                generic_type: Some("String".to_owned()),
            },
            Field {
                name: "val5".to_owned(),
                field_type: "Vec".to_owned(),
                generic_type: Some("u8".to_owned()),
            },
            /*
            */

        ]);
    }

    /*
    #[test]
    fn it_works() {

        let expanded = quote!(
            struct MyStructName {
                val1: u8,
                val2: String,
            }
        );
        let input = TokenStream::from(expanded);

        let output = create_implements(input);

        assert_eq!(r#"{"swagger":"2.0","info":{"description":"The description","version":"1.2.1","title":"The title"},"host":"localhost","basePath":"/","tags":[],"schemes":["http","https"],"paths":{"/foo":{"post":{"tags":[],"summary":"The /foo post summary","description":"The /foo post description","responses":{"200":{"description":"the output description"}}},"get":{"tags":[],"summary":"The /foo get summary","description":"The /foo get description","parameters":[{"in":"query","name":"val1","description":"val1 description","type":"u128ber","required":true},{"in":"query","name":"val2","description":"val2 description","type":"string","required":true},{"in":"query","name":"val1","description":"val1 description","type":"u128ber","required":true},{"in":"query","name":"val2","description":"val2 description","type":"string","required":true}],"responses":{"200":{"description":"the output description","schema":{"properties":{"val1":{"type":"u128ber"},"val2":{"type":"string"}},"required":["val1","val2"],"type":"object"}}}}},"/bar":{"get":{"tags":[],"summary":"The /bar get summary","description":"The /bar get description","responses":{"200":{"description":"the output description"}}}}}}"#, output.to_string());

        println!("input: {}", output.to_string());
    }
    */
    */
}
