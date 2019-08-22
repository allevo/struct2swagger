use crate::quote::ToTokens;
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use std::iter::FromIterator;
use syn::{Data, DeriveInput, Fields};

use crate::Field;

fn get_fields(ast: &DeriveInput) -> Vec<Field> {
    match &ast.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(named_fields) => {
                let mut fields = vec![];

                for field in named_fields.named.iter() {
                    let field_name: String = field.ident.as_ref().unwrap().to_string().to_owned();
                    let mut token_stream = TokenStream::new();
                    field.ty.to_tokens(&mut token_stream);
                    let field_tokens: Vec<TokenTree> = token_stream.into_iter().collect();

                    fields.push(Field {
                        name: field_name,
                        ty: field_tokens,
                    });
                }

                fields
            }
            _ => unimplemented!("Only named struct is implemented. Please send PR!"),
        },
        _ => unimplemented!("Only struct is implemented. Please send PR!"),
    }
}

fn get_struct_name(ast: &DeriveInput) -> String {
    ast.ident.to_string()
}

fn contains_option(tt: &[TokenTree]) -> bool {
    tt.iter().any(|t| match t {
        TokenTree::Ident(ident) => *ident == "Option",
        _ => false,
    })
}

fn get_json_schema_definition(
    fields: &[Field],
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut tokens = Vec::new();

    let mut properties = Vec::new();
    for field in fields {
        properties.push(TokenTree::Literal(Literal::string(&field.name)));
        properties.push(TokenTree::Punct(Punct::new(':', Spacing::Alone)));

        properties.push(TokenTree::Punct(Punct::new('<', Spacing::Alone)));

        properties.append(&mut field.ty.clone());

        properties.push(TokenTree::Punct(Punct::new('>', Spacing::Alone)));

        properties.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
        properties.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));

        properties.push(TokenTree::Ident(Ident::new(
            "get_schema_type",
            Span::call_site(),
        )));

        properties.push(TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            proc_macro2::TokenStream::new(),
        )));

        properties.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));
    }
    let properties = proc_macro2::TokenStream::from_iter(properties.into_iter());

    tokens.push(TokenTree::Group(Group::new(Delimiter::Brace, properties)));
    let tokens = proc_macro2::TokenStream::from_iter(tokens.into_iter());

    let mut required_properties: Vec<TokenTree> = Vec::new();
    for field in fields {
        let has_option = contains_option(&field.ty);

        if has_option {
            continue;
        }
        required_properties.push(TokenTree::Literal(Literal::string(&field.name)));
        required_properties.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));
    }
    let required_properties = proc_macro2::TokenStream::from_iter(required_properties.into_iter());

    (required_properties, tokens)
}

pub fn implements_swagger_trait(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let ast = syn::parse2(input.clone()).unwrap();
    let struct_name = get_struct_name(&ast);
    let fields = get_fields(&ast);

    let (required_properties, tokens) = get_json_schema_definition(&fields);

    let struct_name_ident = TokenTree::Ident(Ident::new(&struct_name, Span::call_site()));
    let struct_name_literal = TokenTree::Literal(Literal::string(&struct_name));

    let q = quote! {
        impl JsonSchemaDefinition for #struct_name_ident {
            fn get_schema_type() -> serde_json::Value {
                json!({
                    "id": #struct_name_literal,
                    "type": "object",
                    "required": [ #required_properties ],
                    "properties": #tokens,
                })
            }
        }
    };

    q

    // input

    /*
    let ast = syn::parse2(input.clone()).unwrap();
    let fields = get_fields(ast);
    let json_schema = proc_macro2::TokenStream::from_iter(get_json_schema_definition(&fields));

    let im = quote! {
        impl JsonSchemaDefinition for MyStructName {
            fn get_schema_type() -> serde_json::Value {
                json! #json_schema;
            }
        }
    };

    im
    */
    // proc_macro2::TokenStream::new()
    // input
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Field;

    #[test]
    fn get_schema_type_u8() {
        let expanded = quote!(
            struct MyStructName1 {
                val1: u8,
                val2: String,
                val3: Vec<u8>,
                // val4: Vec<String>,
                // val5: std::Vec<u8>,
            }
        );
        let input = TokenStream::from(expanded);
        let ast = syn::parse2(input).unwrap();
        let fields = get_fields(&ast);
        let impl_json_schema_definition = get_json_schema_definition(&fields);

        assert_eq!(
            fields,
            vec![
                Field {
                    name: "val1".to_owned(),
                    ty: vec![],
                },
                Field {
                    name: "val2".to_owned(),
                    ty: vec![],
                },
                Field {
                    name: "val3".to_owned(),
                    ty: vec![],
                },
                /*
                Field {
                    name: "val4".to_owned(),
                    field_type: "::Vec".to_owned(),
                    generic_type: Some("String".to_owned()),
                },
                Field {
                    name: "val5".to_owned(),
                    field_type: "::std::Vec".to_owned(),
                    generic_type: Some("u8".to_owned()),
                },
                */
            ]
        );

        // assert_eq!(impl_json_schema_definition.to_string(), "");
    }
}
