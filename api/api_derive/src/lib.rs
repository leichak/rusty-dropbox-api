use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Service)]
pub fn service_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_service_macro(&ast)
}
// I would liuke to pass response, and enum and headers vector
fn impl_service_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Service for #name {

            fn call_sync(&self) -> Result<Option<SetProfilePhotoResponse>> {
                let endpoint = get_endpoint_url(Endpoint::SetProfilePhotoPost).0;


                let response = SyncClient
                    .post(endpoint)
                    .bearer_auth(self.access_token)
                    .header(
                        Headers::ContentTypeAppJson.get_str().0,
                        Headers::ContentTypeAppJson.get_str().1,
                    )
                    .json(&self.parameters())
                    .send()
                    .map_err(|err| ApiError::RequestError(err.into()))?;

                match response.error_for_status() {
                    Ok(response) => {
                        let text = response
                            .text()
                            .map_err(|err| ApiError::ParsingError(err.into()))?;

                        if text.is_empty() {
                            return Ok(None);
                        }

                        let response: SetProfilePhotoResponse = serde_json::from_str(&text)
                            .map_err(|err| ApiError::ParsingError(err.into()))?;
                        Ok(Some(response))
                    }
                    Err(err) => Err(ApiError::DropBoxError(err.into()).into()),
                }
            }



        }
    };
    gen.into()
}
