mod method_helper;
mod routes;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::routes::Routes;

use self::method_helper::MethodHelper;

#[proc_macro_attribute]
pub fn method_helper(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let method_helper = parse_macro_input!(input as MethodHelper);
    quote! {
        #method_helper
    }
    .into()
}

#[proc_macro]
pub fn routes(input: TokenStream) -> TokenStream {
    let routes = parse_macro_input!(input as Routes);
    quote! {
        #routes
    }
    .into()
}
