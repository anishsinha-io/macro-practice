use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

mod util;

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let builder_name = format!("{name}Builder");

    let builder_ident = syn::Ident::new(&builder_name, name.span());

    let fields = match ast.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!(),
    };

    let optionized = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        if util::is_option(&f.ty) {
            quote! {
               #name: #ty
            }
        } else {
            quote! {
               #name: ::std::option::Option<#ty>
            }
        }
    });

    let methods = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        if util::is_option(&f.ty) {
            let unwrapped_ty = util::unwrap_option(&f.ty).unwrap();
            quote! {
                pub fn #name(&mut self, #name: #unwrapped_ty) -> &mut Self {
                    self.#name = Some(#name);
                    self
                }
            }
        } else {
            quote! {
                pub fn #name(&mut self, #name: #ty) -> &mut Self {
                    self.#name = Some(#name);
                    self
                }
            }
        }
    });

    let extend_methods = fields.iter().flat_map(|f| {
        use proc_macro2::TokenTree;
        let ident = &f.ident;
        f.attrs.iter().filter_map(move |attr| {
            use syn::Meta;
            match attr.meta {
                Meta::Path(_) => None,
                Meta::List(ref list) => {
                    let mut tokens = list.tokens.clone().into_iter().take(3);
                    // dbg!(&tokens);
                    if let (
                        Some(TokenTree::Ident(directive)),
                        Some(TokenTree::Punct(_)),
                        Some(TokenTree::Literal(func_name_literal)),
                    ) = (tokens.next(), tokens.next(), tokens.next())
                    {
                        match directive.to_string().as_str() {
                            "each" => match syn::Lit::new(func_name_literal) {
                                syn::Lit::Str(s) => {
                                    let func_ident = syn::Ident::new(&s.value(), s.span());
                                    let _ = match &f.ty {
                                        syn::Type::Path(p) => {
                                            let segments = &mut p.path.segments.iter();
                                            if let (Some(s), None) =
                                                (segments.next(), segments.next())
                                            {
                                                dbg!(&s.ident);
                                                // match s.ident.to_string().as_str() {
                                                //     "Vec" => (),
                                                //     _ => return None
                                                // };
                                            };

                                            Some(())
                                        }
                                        _ => None,
                                    };
                                    Some(quote! {
                                        pub fn #func_ident(&mut self) {
                                            if let Some(ref mut v) = self.#ident {}
                                            else {}
                                        }
                                    })
                                }
                                _ => None,
                            },
                            _ => None,
                        }
                    } else {
                        None
                    }
                }
                Meta::NameValue(_) => None,
            }
        })
    });

    // dbg!(extend_methods.collect::<Vec<_>>());

    let defaults = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            #name: None
        }
    });

    let built_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let wrapped = util::is_option(&f.ty);
        if wrapped {
            quote! {
                 #name: self.#name.clone()
            }
        } else {
            quote! {
                 #name: self.#name.clone().ok_or(concat!(stringify!(#name), " is not set!"))?
            }
        }
    });

    quote! {
        struct #builder_ident {
            #(#optionized,)*
        }

        impl #builder_ident {
            pub fn new() -> #builder_ident {
                #builder_ident {
                    #(#defaults,)*
                }
            }

            pub fn build(&self) -> Result<#name, Box<dyn std::error::Error>> {
                Ok(#name {
                    #(#built_fields,)*
                })
            }

            #(#methods)*

            #(#extend_methods)*

        }

        impl #name {
            pub fn builder() -> #builder_ident {
                #builder_ident::new()
            }
        }
    }
    .into()
}
