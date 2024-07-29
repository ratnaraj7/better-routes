extern crate proc_macro;

mod method_helper;
mod routes;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::routes::Routes;

use self::method_helper::{MethodHelper, State};

#[proc_macro_attribute]
pub fn method_helper(attr: TokenStream, input: TokenStream) -> TokenStream {
    let state = parse_macro_input!(attr as State);
    let mut method_helper = parse_macro_input!(input as MethodHelper);
    method_helper.state = state;

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
