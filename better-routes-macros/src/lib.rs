mod routes;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::routes::Routes;

#[proc_macro]
pub fn routes(input: TokenStream) -> TokenStream {
    let routes = parse_macro_input!(input as Routes);
    quote! {
        #routes
    }
    .into()
}
