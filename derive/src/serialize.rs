use darling::FromField;

#[derive(Clone, Default, Debug, FromField)]
#[darling(attributes(serialize))]
pub(crate) struct Field {
    pub rename: Option<String>,
}

impl Field {
    fn name(&self, name: &Option<syn::Ident>) -> String {
        if let Some(name) = &self.rename {
            return name.clone();
        }

        name.as_ref()
            .unwrap()
            .to_string()
            .to_uppercase()
            .replace('_', "-")
    }
}

pub(crate) fn impl_macro(ast: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let fields = match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => {
            return Err(syn::Error::new_spanned(
                ast,
                "this derive macro only works on structs",
            ));
        }
    };

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let mut body = Vec::new();

    for field in fields {
        let name = &field.ident;
        let field_params = Field::from_field(field)?;

        let field_name = field_params.name(name);

        let ser_part = quote::quote! {
            let ical = self.#name.ical()?;
            if !ical.is_empty() {
                s.push_str(&format!("{}={ical};", #field_name));
            }
        };

        body.push(ser_part);
    }

    let serialize = quote::quote! {
        #[automatically_derived]
        impl #impl_generics crate::ser::Serialize for #name #ty_generics #where_clause {
            fn ical(&self) -> crate::Result<::std::string::String> {
                let mut s = String::new();

                #(#body)*
                s.pop();

                Ok(s)
            }
        }
    };

    Ok(serialize)
}
