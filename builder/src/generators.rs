use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{self, Ident, Type};

use crate::field::{Field, FieldType};

pub(crate) fn map<T, F>(fields: &[Field], f: F) -> Vec<T>
where
    F: Fn(&Field) -> T,
{
    fields.iter().map(|field| f(field)).collect()
}

fn builder_field_ident(field: &Field) -> Ident {
    format_ident!("__{}", field.ident.clone())
}

pub(crate) fn generate_builder_field(field: &Field) -> syn::Field {
    let mut f = field.field.clone();
    f.ident = Some(builder_field_ident(&field));
    match field.field_type {
        FieldType::Regular => {
            let ty = f.ty.clone();
            let ty: Type = syn::parse2(quote!(Option<#ty>)).unwrap();
            f.ty = ty;
            f
        }
        FieldType::Option(_) => f,
    }
}

pub(crate) fn generate_builder_field_init(field: &Field) -> TokenStream {
    let ident = builder_field_ident(field);
    quote!(#ident: None)
}

pub(crate) fn generate_setter(field: &Field) -> TokenStream {
    let ident = field.ident.clone();
    let builder_field_ident = builder_field_ident(field);
    let field = match field.field_type {
        FieldType::Regular => field.field.clone(),
        FieldType::Option(ref ty) => {
            let mut field = field.field.clone();
            field.ty = ty.clone();
            field
        }
    };
    quote! {
        fn #ident(&mut self, #field) -> &mut Self {
            self.#builder_field_ident = Some(#ident);
            self
        }
    }
}

pub(crate) fn validate_field(field: &Field) -> TokenStream {
    let ident = field.ident.clone();
    let builder_field_ident = builder_field_ident(field);
    match field.field_type {
        FieldType::Regular => {
            let err = format!("{} not set", ident);
            quote! {
                let #ident = self.#builder_field_ident.as_ref().ok_or(#err)?.clone();
            }
        }
        FieldType::Option(_) => quote! {
            let #ident = self.#builder_field_ident.clone();
        },
    }
}

pub(crate) fn build_field(field: &Field) -> TokenStream {
    let ident = field.ident.clone();
    quote!(#ident)
}
