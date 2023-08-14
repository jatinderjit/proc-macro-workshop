use syn::{GenericArgument, Ident, PathArguments, Type};

pub(crate) struct Field {
    pub field: syn::Field,
    pub ident: Ident,
    pub field_type: FieldType,
}

impl Field {
    pub(crate) fn new(field: syn::Field) -> Self {
        let field_type = match field.ty {
            Type::Path(ref path) => path
                .path
                .segments
                .first()
                .map(|f| {
                    if f.ident.eq("Option") {
                        match f.arguments {
                            PathArguments::AngleBracketed(ref args) => match args.args.first() {
                                Some(GenericArgument::Type(t)) => FieldType::Option(t.clone()),
                                _ => FieldType::Regular,
                            },
                            _ => FieldType::Regular,
                        }
                    } else {
                        FieldType::Regular
                    }
                })
                .unwrap_or(FieldType::Regular),
            _ => FieldType::Regular,
        };
        Self {
            ident: field.ident.clone().unwrap(),
            field,
            field_type,
        }
    }
}

pub(crate) enum FieldType {
    Regular,
    // Only the values specified as `Option<T>` are identified.
    // Options fields defined in any other way (example: `std::option::Option`)
    // aren't yet supported.
    Option(Type),
}
