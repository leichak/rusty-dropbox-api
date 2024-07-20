use api::Service;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Service)]
pub fn service_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_service_macro(&ast)
}
