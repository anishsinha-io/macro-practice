use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

mod util;

#[proc_macro_derive(Builder)]
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

            #(#methods)*

            pub fn build(&self) -> Result<#name, Box<dyn std::error::Error>> {
                Ok(#name {
                    #(#built_fields,)*
                })
            }
        }

        impl #name {
            pub fn builder() -> #builder_ident {
                #builder_ident::new()
            }
        }
    }
    .into()
}
