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
        properties.push(TokenTree::Punct(Punct::new(':', Spacing::Alone)));

        properties.push(TokenTree::Ident(Ident::new(
            "get_json_schema_definition",
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

fn get_query_definitions(fields: &[Field]) -> proc_macro2::TokenStream {
    let mut query: Vec<TokenTree> = Vec::new();

    for field in fields {
        query.push(TokenTree::Ident(Ident::new(
            "struct2swagger",
            Span::call_site(),
        )));
        query.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
        query.push(TokenTree::Punct(Punct::new(':', Spacing::Alone)));
        query.push(TokenTree::Ident(Ident::new(
            "ParameterObject",
            Span::call_site(),
        )));

        let mut query_field_description = Vec::<TokenTree>::new();

        query_field_description.push(TokenTree::Ident(Ident::new("name", Span::call_site())));
        query_field_description.push(TokenTree::Punct(Punct::new(':', Spacing::Alone)));
        query_field_description.push(TokenTree::Literal(Literal::string(&field.name)));
        query_field_description.push(TokenTree::Punct(Punct::new('.', Spacing::Alone)));
        query_field_description.push(TokenTree::Ident(Ident::new("to_string", Span::call_site())));
        query_field_description.push(TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::new(),
        )));
        query_field_description.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));

        query_field_description.push(TokenTree::Ident(Ident::new("where_in", Span::call_site())));
        query_field_description.push(TokenTree::Punct(Punct::new(':', Spacing::Alone)));
        query_field_description.push(TokenTree::Ident(Ident::new(
            "struct2swagger",
            Span::call_site(),
        )));
        query_field_description.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
        query_field_description.push(TokenTree::Punct(Punct::new(':', Spacing::Alone)));
        query_field_description.push(TokenTree::Ident(Ident::new(
            "ParameterIn",
            Span::call_site(),
        )));
        query_field_description.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
        query_field_description.push(TokenTree::Punct(Punct::new(':', Spacing::Alone)));
        query_field_description.push(TokenTree::Ident(Ident::new("Query", Span::call_site())));
        query_field_description.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));

        query_field_description.push(TokenTree::Ident(Ident::new(
            "description",
            Span::call_site(),
        )));
        query_field_description.push(TokenTree::Punct(Punct::new(':', Spacing::Alone)));
        query_field_description.push(TokenTree::Ident(Ident::new("None", Span::call_site())));
        query_field_description.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));

        query_field_description.push(TokenTree::Ident(Ident::new("required", Span::call_site())));
        query_field_description.push(TokenTree::Punct(Punct::new(':', Spacing::Alone)));
        query_field_description.push(TokenTree::Ident(Ident::new("Some", Span::call_site())));
        let required = {
            let mut required = Vec::new();
            let is_required = if contains_option(&field.ty) {
                "false"
            } else {
                "true"
            };
            required.push(TokenTree::Ident(Ident::new(is_required, Span::call_site())));
            proc_macro2::TokenStream::from_iter(required.into_iter())
        };
        query_field_description.push(TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            required,
        )));
        query_field_description.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));

        query_field_description.push(TokenTree::Ident(Ident::new(
            "deprecated",
            Span::call_site(),
        )));
        query_field_description.push(TokenTree::Punct(Punct::new(':', Spacing::Alone)));
        query_field_description.push(TokenTree::Ident(Ident::new("None", Span::call_site())));
        query_field_description.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));

        query_field_description.push(TokenTree::Ident(Ident::new(
            "allow_empty_value",
            Span::call_site(),
        )));
        query_field_description.push(TokenTree::Punct(Punct::new(':', Spacing::Alone)));
        query_field_description.push(TokenTree::Ident(Ident::new("None", Span::call_site())));
        query_field_description.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));

        query_field_description.push(TokenTree::Ident(Ident::new("schema", Span::call_site())));
        query_field_description.push(TokenTree::Punct(Punct::new(':', Spacing::Alone)));
        query_field_description.push(TokenTree::Ident(Ident::new("Some", Span::call_site())));
        let schema_or_ref = {
            let mut schema_or_ref = Vec::new();
            schema_or_ref.push(TokenTree::Ident(Ident::new(
                "struct2swagger",
                Span::call_site(),
            )));
            schema_or_ref.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
            schema_or_ref.push(TokenTree::Punct(Punct::new(':', Spacing::Alone)));
            schema_or_ref.push(TokenTree::Ident(Ident::new(
                "swagger_object",
                Span::call_site(),
            )));
            schema_or_ref.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
            schema_or_ref.push(TokenTree::Punct(Punct::new(':', Spacing::Alone)));
            schema_or_ref.push(TokenTree::Ident(Ident::new(
                "SchemaObjectOrReferenceObject",
                Span::call_site(),
            )));
            schema_or_ref.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
            schema_or_ref.push(TokenTree::Punct(Punct::new(':', Spacing::Alone)));
            schema_or_ref.push(TokenTree::Ident(Ident::new(
                "SchemaObject",
                Span::call_site(),
            )));

            let boxed = {
                let mut boxed = Vec::new();

                boxed.push(TokenTree::Ident(Ident::new("Box", Span::call_site())));
                boxed.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
                boxed.push(TokenTree::Punct(Punct::new(':', Spacing::Alone)));
                boxed.push(TokenTree::Ident(Ident::new("new", Span::call_site())));

                let query_field_definition = {
                    let mut query_field_definition = Vec::new();
                    query_field_definition.push(TokenTree::Punct(Punct::new('<', Spacing::Alone)));
                    query_field_definition.append(&mut field.ty.clone());
                    query_field_definition.push(TokenTree::Punct(Punct::new('>', Spacing::Alone)));
                    query_field_definition.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
                    query_field_definition.push(TokenTree::Punct(Punct::new(':', Spacing::Alone)));
                    query_field_definition.push(TokenTree::Ident(Ident::new(
                        "get_json_schema_definition",
                        Span::call_site(),
                    )));
                    query_field_definition.push(TokenTree::Group(Group::new(
                        Delimiter::Parenthesis,
                        proc_macro2::TokenStream::new(),
                    )));

                    proc_macro2::TokenStream::from_iter(query_field_definition.into_iter())
                };

                boxed.push(TokenTree::Group(Group::new(
                    Delimiter::Parenthesis,
                    query_field_definition,
                )));

                proc_macro2::TokenStream::from_iter(boxed.into_iter())
            };

            schema_or_ref.push(TokenTree::Group(Group::new(Delimiter::Parenthesis, boxed)));

            proc_macro2::TokenStream::from_iter(schema_or_ref.into_iter())
        };

        query_field_description.push(TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            schema_or_ref,
        )));

        query_field_description.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));

        let query_field_description =
            proc_macro2::TokenStream::from_iter(query_field_description.into_iter());

        query.push(TokenTree::Group(Group::new(
            Delimiter::Brace,
            query_field_description,
        )));
        query.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));
    }

    proc_macro2::TokenStream::from_iter(query.into_iter())
}

pub fn implements_swagger_trait(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let ast = syn::parse2(input).unwrap();
    let struct_name = get_struct_name(&ast);
    let fields = get_fields(&ast);

    let (required_properties, tokens) = get_json_schema_definition(&fields);

    let query_params = get_query_definitions(&fields);

    let struct_name_ident = TokenTree::Ident(Ident::new(&struct_name, Span::call_site()));

    let query_definition_quote = quote! {
        impl struct2swagger::QueryDefinition for #struct_name_ident {
            fn get_query_definitions() -> Vec<struct2swagger::ParameterObject> {
                vec![
                    #query_params
                ]
            }
        }
    };
    let json_schema_definition_quote;
    if required_properties.is_empty() {
        json_schema_definition_quote = quote! {
            impl struct2swagger::JsonSchemaDefinition for #struct_name_ident {
                fn get_json_schema_definition() -> serde_json::Value {
                    json!({
                        "type": "object",
                        "properties": #tokens,
                    })
                }
            }
        }
    } else {
        json_schema_definition_quote = quote! {
            impl struct2swagger::JsonSchemaDefinition for #struct_name_ident {
                fn get_json_schema_definition() -> serde_json::Value {
                    json!({
                        "type": "object",
                        "required": [ #required_properties ],
                        "properties": #tokens,
                    })
                }
            }
        }
    }

    quote! {
        #json_schema_definition_quote
        #query_definition_quote
    }
}
