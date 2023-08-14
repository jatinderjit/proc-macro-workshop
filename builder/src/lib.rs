use proc_macro;
use quote::{format_ident, quote};
use syn::{self, parse_macro_input, Data, DeriveInput, Fields};

mod field;
mod generators;
use field::Field;
use generators::*;

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);

    let builder_name = format_ident!("{ident}Builder");

    let fields = match data {
        Data::Struct(d) => match d.fields {
            Fields::Named(fields) => fields
                .named
                .iter()
                .map(|f| Field::new(f.clone()))
                .collect::<Vec<_>>(),
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
