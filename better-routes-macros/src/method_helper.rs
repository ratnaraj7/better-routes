use std::collections::HashSet;

use quote::{format_ident, quote, ToTokens};
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::{Attribute, FnArg, ImplItemFn, ItemImpl};

#[derive(PartialEq, Eq, Hash)]
enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

impl ToTokens for Method {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Method::Get => tokens.extend(quote! { ::axum::http::Method::GET }),
            Method::Post => tokens.extend(quote! { ::axum::http::Method::POST }),
            Method::Put => tokens.extend(quote! { ::axum::http::Method::PUT }),
            Method::Delete => tokens.extend(quote! { ::axum::http::Method::DELETE }),
            Method::Patch => tokens.extend(quote! { ::axum::http::Method::PATCH }),
        }
    }
}

pub struct MethodHelper {
    fns_: Vec<ImplItemFn>,
    methods: HashSet<Method>,
    item_impl: ItemImpl,
}

impl Parse for MethodHelper {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut item_impl: ItemImpl = input.parse()?;
        let mut methods = HashSet::new();
        let mut fns_ = Vec::new();
        for item in item_impl.items.iter_mut() {
            if let syn::ImplItem::Fn(fn_) = item {
                let mut to_remove = Vec::new();
                for (i, attrs) in fn_.attrs.iter().enumerate() {
                    if let Some(method) = attr_to_method(attrs) {
                        // check if method is async
                        if fn_.sig.asyncness.is_none() {
                            return Err(syn::Error::new(
                                fn_.sig.fn_token.span(),
                                "This method must be async",
                            ));
                        }
                        // check if first argument is self
                        let s: FnArg = syn::parse_quote! { self };
                        if fn_.sig.inputs.first() != Some(&s) {
                            return Err(syn::Error::new(
                                fn_.sig.fn_token.span(),
                                "First argument must be self",
                            ));
                        }
                        let mut fn_ = fn_.clone();
                        match method {
                            Method::Get => fn_.sig.ident = format_ident!("get"),
                            Method::Post => fn_.sig.ident = format_ident!("post"),
                            Method::Put => fn_.sig.ident = format_ident!("put"),
                            Method::Delete => fn_.sig.ident = format_ident!("delete"),
                            Method::Patch => fn_.sig.ident = format_ident!("patch"),
                        }
                        fn_.attrs = Vec::new();
                        fns_.push(fn_);
                        methods.insert(method);
                        to_remove.push(i);
                        break;
                    }
                }
                for i in to_remove.into_iter().rev() {
                    fn_.attrs.remove(i);
                }
            }
        }
        Ok(Self {
            fns_,
            methods,
            item_impl,
        })
    }
}

fn attr_to_method(attr: &Attribute) -> Option<Method> {
    let get: Attribute = syn::parse_quote! { #[get] };
    let post: Attribute = syn::parse_quote! { #[post] };
    let put: Attribute = syn::parse_quote! { #[put] };
    let patch: Attribute = syn::parse_quote! { #[patch] };
    let delete: Attribute = syn::parse_quote! { #[delete] };
    match attr {
        attr if attr.eq(&get) => Some(Method::Get),
        attr if attr.eq(&post) => Some(Method::Post),
        attr if attr.eq(&put) => Some(Method::Put),
        attr if attr.eq(&patch) => Some(Method::Patch),
        attr if attr.eq(&delete) => Some(Method::Delete),
        _ => None,
    }
}

impl ToTokens for MethodHelper {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            fns_,
            item_impl,
            methods,
        } = self;
        let self_ty = &item_impl.self_ty;
        let all_methods = [
            Method::Get,
            Method::Post,
            Method::Put,
            Method::Delete,
            Method::Patch,
        ]
        .into_iter()
        .collect::<HashSet<_>>();
        let diff = all_methods.difference(methods).map(|method| match method {
            Method::Get => quote! { async fn get(self) {} },
            Method::Post => quote! { async fn post(self) {} },
            Method::Put => quote! { async fn put(self) {} },
            Method::Delete => quote! { async fn delete(self) {} },
            Method::Patch => quote! { async fn patch(self) {} },
        });
        tokens.extend(quote! {
            #item_impl
        });
        tokens.extend(quote! {
            #[automatically_derived]
            impl #self_ty {
                #(#fns_)*
                #(#diff)*
            }
        });
        let methods = methods.iter();
        tokens.extend(quote! {
            #[automatically_derived]
            impl ::better_routes::MethodHandlers for #self_ty {
                const METHODS: &'static [::axum::http::Method] = &[#(#methods),*];
            }
        });
    }
}
