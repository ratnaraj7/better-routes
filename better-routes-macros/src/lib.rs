mod routes;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

use crate::routes::Routes;

#[proc_macro]
pub fn routes(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as Routes)
        .into_token_stream()
        .into()
}
