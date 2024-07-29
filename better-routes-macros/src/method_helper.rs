use quote::{quote, ToTokens};
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::{Attribute, ExprCall, FnArg, Ident, ItemImpl, Path};

pub struct State(Option<Path>);

impl Parse for State {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self(None));
        }
        Ok(Self(Some(input.parse()?)))
    }
}

pub struct MethodHelper {
    pub state: State,
    pub item_impl: ItemImpl,
    pub methods: Vec<ExprCall>,
}

impl Parse for MethodHelper {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let item_impl = input.parse()?;
        let (methods, item_impl) = Self::methods(item_impl)?;
        Ok(Self {
            state: State(None),
            item_impl,
            methods,
        })
    }
}

impl MethodHelper {
    fn methods(mut item_impl: ItemImpl) -> syn::Result<(Vec<ExprCall>, ItemImpl)> {
        let mut methods = Vec::new();
        for item in item_impl.items.iter_mut() {
            if let syn::ImplItem::Fn(method) = item {
                let ident = &method.sig.ident;
                let mut to_remove_indexes = Vec::new();
                for (i, attrs) in method.attrs.iter().enumerate() {
                    if let Some(call) = Self::to_call(attrs, ident) {
                        // check if method is async
                        if method.sig.asyncness.is_none() {
                            return Err(syn::Error::new(
                                method.sig.fn_token.span(),
                                "This method must be async",
                            ));
                        }

                        // check if first argument is self
                        let s: FnArg = syn::parse_quote! { self };
                        if method.sig.inputs.first() != Some(&s) {
                            return Err(syn::Error::new(
                                method.sig.fn_token.span(),
                                "First argument must be self",
                            ));
                        }
                        methods.push(call);
                        to_remove_indexes.push(i);
                    }
                }
                for i in to_remove_indexes.into_iter() {
                    method.attrs.remove(i);
                }
            }
        }

        if methods.is_empty() {
            return Err(syn::Error::new(item_impl.span(), "No methods found"));
        }

        Ok((methods, item_impl))
    }

    // returns Some(ExprCall) if attribute matches a method attribute else None
    fn to_call(attr: &Attribute, ident: &Ident) -> Option<ExprCall> {
        let get: Attribute = syn::parse_quote! { #[get] };
        let post: Attribute = syn::parse_quote! { #[post] };
        let put: Attribute = syn::parse_quote! { #[put] };
        let patch: Attribute = syn::parse_quote! { #[patch] };
        let delete: Attribute = syn::parse_quote! { #[delete] };

        if attr == &get {
            return Some(syn::parse_quote! {
                typed_get(Self::#ident)
            });
        }

        if attr == &post {
            return Some(syn::parse_quote! {
                typed_post(Self::#ident)
            });
        }
        if attr == &put {
            return Some(syn::parse_quote! {
                typed_put(Self::#ident)
            });
        }

        if attr == &delete {
            return Some(syn::parse_quote! {
                typed_delete(Self::#ident)
            });
        }

        if attr == &patch {
            return Some(syn::parse_quote! {
                typed_patch(Self::#ident)
            });
        }

        None
    }
}

impl ToTokens for MethodHelper {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let item_impl = &self.item_impl;
        let self_ty = &item_impl.self_ty;
        let methods = &self.methods;

        tokens.extend(if let Some(state) = &self.state.0 {
            quote! {
                impl ::better_routes::MethodHandler<#state> for #self_ty {
                    fn router() -> ::axum::Router<#state> {
                        ::axum::Router::new()
                            #(.#methods)*
                    }
                }
            }
        } else {
            quote! {
                impl ::better_routes::MethodHandler for #self_ty {
                    fn router() -> ::axum::Router {
                        ::axum::Router::new()
                        #(.#methods)*
                    }
                }
            }
        });

        tokens.extend(quote! {
              #item_impl
        });
    }
}
