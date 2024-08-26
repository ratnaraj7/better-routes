use proc_macro2::Span;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{braced, Ident, LitStr, Path, Token, Visibility};

mod kw {
    syn::custom_keyword!(name);
    syn::custom_keyword!(state);
    syn::custom_keyword!(rejection);
    syn::custom_keyword!(get);
    syn::custom_keyword!(post);
    syn::custom_keyword!(put);
    syn::custom_keyword!(patch);
    syn::custom_keyword!(delete);
}

struct Route {
    path: LitStr,
    segments: Vec<Segment>,
    path_struct: Path,
    rejection: Option<Path>,
    get_handler: Option<Path>,
    post_handler: Option<Path>,
    put_handler: Option<Path>,
    patch_handler: Option<Path>,
    delete_handler: Option<Path>,
}

pub struct Routes {
    state: Option<Path>,
    rejection: Option<Path>,
    routes: Vec<Route>,
    name: Ident,
    vis: Visibility,
}

impl Parse for Routes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let (vis, name) = {
            input.parse::<kw::name>()?;
            input.parse::<Token![=>]>()?;
            let vis = input.parse::<Visibility>()?;
            let name = input.parse()?;
            input.parse::<Token![,]>()?;
            (vis, name)
        };
        let state = if input.peek(kw::state) {
            input.parse::<kw::state>()?;
            input.parse::<Token![=>]>()?;
            let state = Some(input.parse()?);
            input.parse::<Token![,]>()?;
            state
        } else {
            None
        };
        let rejection = if input.peek(kw::rejection) {
            input.parse::<kw::rejection>()?;
            input.parse::<Token![=>]>()?;
            let rejection = Some(input.parse()?);
            input.parse::<Token![,]>()?;
            rejection
        } else {
            None
        };
        let mut routes = Vec::new();
        let mut count = 0;
        while !input.is_empty() {
            if count > 0 {
                input.parse::<Token![,]>()?;
                if input.is_empty() {
                    break;
                }
            }
            let path: LitStr = input.parse()?;
            input.parse::<Token![=>]>()?;
            let rejection: Option<Path> = if input.peek(kw::rejection) {
                input.parse::<kw::rejection>()?;
                let rejection = Some(input.parse()?);
                input.parse::<Token![=>]>()?;
                rejection
            } else {
                None
            };
            let path_struct: Path = input.parse()?;
            let mut get_handler = None;
            let mut post_handler = None;
            let mut put_handler = None;
            let mut patch_handler = None;
            let mut delete_handler = None;
            let mut count2 = 0;
            let content;
            let _ = braced!(content in input);
            if content.is_empty() {
                return Err(syn::Error::new(
                    path_struct.span(),
                    "expected at least one handler",
                ));
            }
            while !content.is_empty() {
                if count2 > 0 {
                    content.parse::<Token![,]>()?;
                    if content.is_empty() {
                        break;
                    }
                }
                if content.peek(kw::get) {
                    content.parse::<kw::get>()?;
                    content.parse::<Token![=>]>()?;
                    if get_handler.is_some() {
                        return Err(syn::Error::new(content.span(), "duplicate get handler"));
                    }
                    get_handler = Some(content.parse()?);
                }
                if content.peek(kw::post) {
                    content.parse::<kw::post>()?;
                    content.parse::<Token![=>]>()?;
                    if post_handler.is_some() {
                        return Err(syn::Error::new(content.span(), "duplicate post handler"));
                    }
                    post_handler = Some(content.parse()?);
                }
                if content.peek(kw::put) {
                    content.parse::<kw::put>()?;
                    content.parse::<Token![=>]>()?;
                    if put_handler.is_some() {
                        return Err(syn::Error::new(content.span(), "duplicate put handler"));
                    }
                    put_handler = Some(content.parse()?);
                }
                if content.peek(kw::patch) {
                    content.parse::<kw::patch>()?;
                    content.parse::<Token![=>]>()?;
                    if patch_handler.is_some() {
                        return Err(syn::Error::new(content.span(), "duplicate patch handler"));
                    }
                    patch_handler = Some(content.parse()?);
                }
                if content.peek(kw::delete) {
                    content.parse::<kw::delete>()?;
                    content.parse::<Token![=>]>()?;
                    if delete_handler.is_some() {
                        return Err(syn::Error::new(content.span(), "duplicate delete handler"));
                    }
                    delete_handler = Some(content.parse()?);
                }

                count2 += 1;
            }
            let segments = parse_path(&path)?;
            routes.push(Route {
                get_handler,
                post_handler,
                put_handler,
                patch_handler,
                delete_handler,
                path,
                segments,
                path_struct,
                rejection,
            });
            count += 1;
        }
        Ok(Routes {
            name,
            vis,
            state,
            rejection,
            routes,
        })
    }
}

fn parse_path(path: &LitStr) -> syn::Result<Vec<Segment>> {
    let value = path.value();
    if value.is_empty() {
        return Err(syn::Error::new_spanned(
            path,
            "paths must start with a `/`. Use \"/\" for root routes",
        ));
    } else if !path.value().starts_with('/') {
        return Err(syn::Error::new_spanned(path, "paths must start with a `/`"));
    }

    path.value()
        .split('/')
        .map(|segment| {
            if let Some(capture) = segment
                .strip_prefix(':')
                .or_else(|| segment.strip_prefix('*'))
            {
                Ok(Segment::Capture(capture.to_owned(), path.span()))
            } else {
                Ok(Segment::Static(segment.to_owned()))
            }
        })
        .collect()
}

#[derive(Debug)]
enum Segment {
    Capture(String, Span),
    Static(String),
}

impl ToTokens for Routes {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Routes {
            name,
            state,
            rejection: global_rejection,
            routes,
            vis,
        } = self;

        let mut routes_fn = Vec::new();
        routes.iter().for_each(
            |Route {
                 path,
                 segments,
                 path_struct,
                 rejection,
                 get_handler,
                 post_handler,
                 put_handler,
                 patch_handler,
                 delete_handler,
             }| {
                let format_str = format_str_from_path(segments);
                let captures = captures_from_path(segments);
                tokens.extend(quote_spanned! {
                    path.span() =>
                    #[automatically_derived]
                    impl ::axum_extra::routing::TypedPath for #path_struct {
                        const PATH: &'static str = #path;
                    }
                });
                tokens.extend(quote_spanned! {
                    path.span()=>
                    #[automatically_derived]
                    impl ::std::fmt::Display for #path_struct {
                        #[allow(clippy::unnecessary_to_owned)]
                        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            let Self { #(#captures,)* } = self;
                            write!(
                                f,
                                #format_str,
                                #(
                                    #captures = ::axum_extra::__private::utf8_percent_encode(
                                        &#captures.to_string(),
                                        ::axum_extra::__private::PATH_SEGMENT,
                                    )
                                ),*
                            )
                        }
                    }
                });
                let (rejection_assoc_type, map_err_rejection) = if rejection.is_some() {
                    (
                        rejection_assoc_type(rejection),
                        map_err_rejection(rejection),
                    )
                } else if global_rejection.is_some() {
                    (
                        rejection_assoc_type(global_rejection),
                        map_err_rejection(global_rejection),
                    )
                } else {
                    (
                        rejection_assoc_type(rejection),
                        map_err_rejection(rejection),
                    )
                };
                tokens.extend(quote_spanned! {
                    path_struct.span() =>
                    #[::axum::async_trait]
                    #[automatically_derived]
                    impl<S> ::axum::extract::FromRequestParts<S> for #path_struct
                    where
                        S: Send + Sync,
                    {
                        type Rejection = #rejection_assoc_type;

                        async fn from_request_parts(
                            parts: &mut ::axum::http::request::Parts,
                            state: &S,
                        ) -> ::std::result::Result<Self, Self::Rejection> {
                            ::axum::extract::Path::from_request_parts(parts, state)
                                .await
                                .map(|path| path.0)
                                #map_err_rejection
                        }
                    }
                });

                if let Some(get_handler) = get_handler {
                    routes_fn.push(quote_spanned! {
                        get_handler.span() =>
                        r = r.typed_get::<_,_,#path_struct>(#get_handler);
                    })
                }
                if let Some(post_handler) = post_handler {
                    routes_fn.push(quote_spanned! {
                        post_handler.span() =>
                        r = r.typed_post::<_,_,#path_struct>(#post_handler);
                    })
                }
                if let Some(put_handler) = put_handler {
                    routes_fn.push(quote_spanned! {
                        put_handler.span() =>
                        r = r.typed_put::<_,_,#path_struct>(#put_handler);
                    })
                }
                if let Some(patch_handler) = patch_handler {
                    routes_fn.push(quote_spanned! {
                        patch_handler.span() =>
                        r = r.typed_patch::<_,_,#path_struct>(#patch_handler);
                    })
                }
                if let Some(delete_handler) = delete_handler {
                    routes_fn.push(quote_spanned! {
                        delete_handler.span() =>
                        r = r.typed_delete::<_,_,#path_struct>(#delete_handler);
                    })
                }
            },
        );
        if let Some(state) = state {
            tokens.extend(quote_spanned! {
                name.span() =>
                #vis struct #name;
                #[automatically_derived]
                #[allow(unused_mut)]
                #[allow(clippy::let_and_return)]
                impl #name {
                    #vis fn routes() -> ::axum::Router<#state> {
                        let mut r = ::axum::Router::new();
                        #(#routes_fn)*
                        r
                    }
                }
            })
        } else {
            tokens.extend(quote_spanned! {
                name.span() =>
                #vis struct #name;
                #[automatically_derived]
                #[allow(unused_mut)]
                #[allow(clippy::let_and_return)]
                impl #name {
                    #vis fn routes() -> ::axum::Router {
                        let mut r = ::axum::Router::new();
                        #(#routes_fn)*
                        r
                    }
                }
            })
        }
    }
}

fn format_str_from_path(segments: &[Segment]) -> String {
    segments
        .iter()
        .map(|segment| match segment {
            Segment::Capture(capture, _) => format!("{{{capture}}}"),
            Segment::Static(segment) => segment.to_owned(),
        })
        .collect::<Vec<_>>()
        .join("/")
}

fn captures_from_path(segments: &[Segment]) -> Vec<syn::Ident> {
    segments
        .iter()
        .filter_map(|segment| match segment {
            Segment::Capture(capture, span) => Some(format_ident!("{}", capture, span = *span)),
            Segment::Static(_) => None,
        })
        .collect::<Vec<_>>()
}

fn path_rejection() -> proc_macro2::TokenStream {
    quote! {
        <::axum::extract::Path<Self> as ::axum::extract::FromRequestParts<S>>::Rejection
    }
}

fn rejection_assoc_type(rejection: &Option<syn::Path>) -> proc_macro2::TokenStream {
    match rejection {
        Some(rejection) => quote! { #rejection },
        None => path_rejection(),
    }
}

fn map_err_rejection(rejection: &Option<syn::Path>) -> proc_macro2::TokenStream {
    rejection
        .as_ref()
        .map(|rejection| {
            let path_rejection = path_rejection();
            quote! {
                .map_err(|rejection| {
                    <#rejection as ::std::convert::From<#path_rejection>>::from(rejection)
                })
            }
        })
        .unwrap_or_default()
}
