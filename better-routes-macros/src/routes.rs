use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{Ident, ItemStruct, LitStr, Token};

struct Route {
    path: LitStr,
    item_struct: ItemStruct,
    rejection: Option<Ident>,
}

pub struct Routes {
    state: Option<Ident>,
    rejection: Option<Ident>,
    routes: Vec<Route>,
}

impl Parse for Routes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut state = None;
        let mut rejection = None;
        let mut routes = Vec::new();
        let mut count = 0;

        while !input.is_empty() {
            if count > 0 {
                input.parse::<Token![,]>()?;
                if input.is_empty() {
                    break;
                }
            }

            if input.peek(Ident) {
                let ident: Ident = input.parse()?;
                if ident == "State" {
                    if state.is_some() {
                        return Err(syn::Error::new(ident.span(), "Duplicate State"));
                    }
                    input.parse::<Token![=>]>()?;
                    state = Some(input.parse()?);
                } else if ident == "Rejection" {
                    if rejection.is_some() {
                        return Err(syn::Error::new(ident.span(), "Duplicate Rejection"));
                    }
                    input.parse::<Token![=>]>()?;
                    rejection = Some(input.parse()?);
                }
            } else if input.peek(LitStr) {
                let path: LitStr = input.parse()?;
                input.parse::<Token![=>]>()?;
                let item_struct: ItemStruct = input.parse()?;
                let rejection: Option<Ident> = if input.peek(Token![=>]) {
                    input.parse::<Token![=>]>()?;
                    Some(input.parse()?)
                } else {
                    None
                };
                routes.push(Route {
                    path,
                    item_struct,
                    rejection,
                });
            } else {
                return Err(syn::Error::new(input.span(), "Unexpected Token"));
            }

            count += 1;
        }

        if (state.is_some() || rejection.is_some()) && routes.is_empty() {
            return Err(syn::Error::new(input.span(), "Missing routes"));
        }

        Ok(Routes {
            state,
            rejection,
            routes,
        })
    }
}

impl ToTokens for Routes {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Routes {
            state,
            rejection: global_rejection,
            routes,
        } = self;

        let typed_paths = routes.iter().map(
            |Route {
                 path,
                 item_struct,
                 rejection,
             }| {
                if let Some(rejection) = rejection.as_ref() {
                    return quote! {
                        #[derive(::axum_extra::routing::TypedPath, ::serde::Deserialize)]
                        #[typed_path(#path, rejection(#rejection))]
                        #item_struct
                    };
                }

                if let Some(rejection) = global_rejection.as_ref() {
                    return quote! {
                        #[derive(::axum_extra::routing::TypedPath, ::serde::Deserialize)]
                        #[typed_path(#path, rejection(#rejection))]
                        #item_struct
                    };
                }

                quote! {
                    #[derive(::axum_extra::routing::TypedPath, ::serde::Deserialize)]
                    #[typed_path(#path)]
                    #item_struct
                }
            },
        );

        let get_all_routes_fn = if let Some(state) = state {
            quote! {
                    fn get_all_routes<T: ::better_routes::MethodHandler<#state>>() -> ::axum::Router<#state> {
                        T::router()
                    }
            }
        } else {
            quote! {
                    fn get_all_routes<T: ::better_routes::MethodHandler>() -> ::axum::Router {
                        T::router()
                    }
            }
        };

        let mergers = routes.iter().map(|Route { item_struct, .. }| {
            let ident = &item_struct.ident;
            quote! {
                app = app.merge(get_all_routes::<#ident>());
            }
        });

        let router_fn = if let Some(state) = state {
            quote! {
                pub fn router() -> ::axum::Router<#state> {
                    let mut app = ::axum::Router::new();
                    #(#mergers)*
                    app
                }
            }
        } else {
            quote! {
                pub fn router() -> ::axum::Router {
                    let mut app = ::axum::Router::new();
                    #(#mergers)*
                    app
                }
            }
        };

        tokens.extend(typed_paths);
        tokens.extend(get_all_routes_fn);
        tokens.extend(router_fn);
    }
}
