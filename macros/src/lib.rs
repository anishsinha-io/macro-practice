use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

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

    // let option_unwrapped = |f|

    let option_wrapped = |ty: &syn::Type| {
        if let syn::Type::Path(ref p) = ty {
            let leading_path = &p
                .path
                .segments
                .iter()
                .take(3)
                .into_iter()
                .fold("".to_owned(), |acc, el| acc + "::" + &el.ident.to_string())[2..];

            return match leading_path {
                "Option" | "option::Option" | "std::option::Option" => true,
                _ => false,
            };
        };
        false
    };

    let flattened_option = |ty: &syn::Type| {};

    let unwrap_option = |ty: &syn::Type| {
        if let syn::Type::Path(ref p) = ty {
            if let syn::PathArguments::AngleBracketed(ref inner_ty) = p.path.segments[0].arguments {
            } else {
                panic!("Option type was not Option<T>")
            }
        }
    };

    let optionized = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        if option_wrapped(&f.ty) {
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

        if option_wrapped(&f.ty) {
            quote! {
                pub fn #name(&mut self, #name: #ty) -> &mut Self {
                    self.#name = #name;
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
        let wrapped = option_wrapped(&f.ty);
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
