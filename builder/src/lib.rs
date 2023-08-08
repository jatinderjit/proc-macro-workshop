use proc_macro;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{self, parse_macro_input, Data, DeriveInput, Field, Fields, Type};

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);

    let builder_name = format_ident!("{ident}Builder");

    let fields = match data {
        Data::Struct(d) => match d.fields {
            Fields::Named(fields) => fields.named.iter().map(|f| f.clone()).collect::<Vec<_>>(),
            _ => panic!("Builder can only be derived for structs with named fields."),
        },
        _ => panic!("Builder can only be derived for structs."),
    };
    let fields = fields.as_ref();

    let builder_fields = map(fields, generate_builder_field);
    let builder_fields_init = map(fields, generate_builder_field_init);
    let setters = map(fields, generate_setter);
    let validators = map(fields, validate_field);
    let build_fields = map(fields, build_field);

    let expanded = quote! {
        impl #ident {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#builder_fields_init),*
                }
            }
        }

        pub struct #builder_name {
            #(#builder_fields),*
        }


        impl #builder_name {
            #(#setters)*

            pub fn build(&mut self) -> Result<#ident, Box<dyn ::std::error::Error>> {
                #(#validators)*
                Ok(#ident {
                    #(#build_fields),*
                })
            }
        }
    };
    expanded.into()
}

fn map<T, F>(fields: &[Field], f: F) -> Vec<T>
where
    F: Fn(&Field) -> T,
{
    fields.iter().map(|field| f(field)).collect()
}

fn builder_field_ident(field: &Field) -> Ident {
    format_ident!("__{}", field.ident.clone().unwrap())
}

fn generate_builder_field(field: &Field) -> Field {
    let mut field = field.clone();
    field.ident = Some(builder_field_ident(&field));
    let ty = field.ty.clone();
    let ty: Type = syn::parse2(quote!(Option<#ty>)).unwrap();
    field.ty = ty;
    field
}

fn generate_builder_field_init(field: &Field) -> TokenStream {
    let ident = builder_field_ident(field);
    quote!(#ident: None)
}

fn generate_setter(field: &Field) -> TokenStream {
    let ident = field.ident.clone().unwrap();
    let builder_field_ident = builder_field_ident(field);
    quote! {
        fn #ident(&mut self, #field) -> &mut Self {
            self.#builder_field_ident = Some(#ident);
            self
        }
    }
}

fn validate_field(field: &Field) -> TokenStream {
    let ident = field.ident.clone().unwrap();
    let builder_field_ident = builder_field_ident(field);
    quote! {
        let #ident = self.#builder_field_ident.as_ref().ok_or("#ident not set")?.clone();
    }
}

fn build_field(field: &Field) -> TokenStream {
    let ident = field.ident.clone().unwrap();
    quote!(#ident)
}
