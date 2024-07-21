use api::Service;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Service)]
pub fn service_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_service_macro(&ast)
}

fn impl_service_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Service for #name {
            fn sync() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }

            fn async() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
