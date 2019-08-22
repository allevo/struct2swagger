extern crate proc_macro;
extern crate proc_macro2;

extern crate swagger;

use swagger::implements_swagger_trait;

#[proc_macro_derive(Swagger, attributes(swagger))]
pub fn swagger_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    implements_swagger_trait(proc_macro2::TokenStream::from(input)).into()
}
