mod params;
mod symbol;

#[proc_macro_derive(Component, attributes(component))]
pub fn component_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_macro(&ast)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn impl_macro(ast: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let fields = match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => {
            return Err(syn::Error::new_spanned(
                ast,
                "this derive macro only works on structs",
            ))
        }
    };

    let name = &ast.ident;
    let parser = quote::format_ident!("{}", name.to_string().to_lowercase());
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let mut from_body = Vec::new();

    for field in fields {
        let name = &field.ident;
        let ty = &field.ty;
        let field_params = crate::params::Field::from_ast(field)?;

        if field_params.ignore {
            continue;
        }

        let parser_fn = quote::quote! { crate::parser::#name(&value)? };
        let parser = if is_option(ty) {
            quote::quote! { component.#name = Some(#parser_fn) }
        } else if is_vec(ty) {
            if field_params.append {
                quote::quote! { component.#name.append(&mut #parser_fn) }
            } else {
                quote::quote! { component.#name.push(#parser_fn) }
            }
        } else {
            quote::quote! { component.#name = #parser_fn }
        };

        let field_name = name
            .as_ref()
            .unwrap()
            .to_string()
            .to_uppercase()
            .replace('_', "-");
        let from_part = quote::quote! {
            #field_name => #parser
        };

        from_body.push(from_part);
    }

    let traits = quote::quote! {
        #[automatically_derived]
        #[doc(hidden)]
        impl #impl_generics TryFrom<std::collections::BTreeMap<String, String>> for #name #ty_generics #where_clause {
            type Error = crate::Error;

            fn try_from(properties: std::collections::BTreeMap<String, String>) -> crate::Result<Self> {
                let mut component = Self::new();

                for (key, value) in properties {
                    match key.as_str() {
                        #(#from_body, )*
                        _ => {
                            if key.starts_with("X-") {
                                component.x_prop.insert(key, value);
                            } else {
                                component.iana_prop.insert(key, value);
                            }
                        }
                    }
                }

                Ok(component)
            }
        }

        #[automatically_derived]
        impl #impl_generics TryFrom<String> for #name #ty_generics #where_clause {
            type Error = crate::Error;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                value.parse()
            }
        }

        #[automatically_derived]
        impl #impl_generics TryFrom<&str> for #name #ty_generics #where_clause {
            type Error = crate::Error;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                value.parse()
            }
        }

        #[automatically_derived]
        impl #impl_generics std::str::FromStr for #name #ty_generics #where_clause {
            type Err = crate::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                crate::parser::#parser(s)
                    .map_err(crate::Error::from)
                    .map(|(_, x)| x)
            }
        }
    };

    Ok(traits)
}

fn is_option(ty: &syn::Type) -> bool {
    tyname(ty) == "Option"
}

fn is_vec(ty: &syn::Type) -> bool {
    tyname(ty) == "Vec"
}

fn tyname(ty: &syn::Type) -> String {
    let syn::Type::Path(typepath) = ty else {
        return String::new();
    };

    typepath
        .path
        .segments
        .iter()
        .map(|x| x.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}
