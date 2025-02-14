use darling::FromField;
use quote::ToTokens as _;

#[derive(Clone, Default, Debug, FromField)]
#[darling(attributes(component))]
pub(crate) struct Field {
    #[darling(default)]
    pub append: bool,
    #[darling(default)]
    pub ignore: bool,
}

pub(crate) fn impl_macro(ast: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
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
    let name_str = name.to_token_stream().to_string();
    let parser = quote::format_ident!("{}", name.to_string().to_lowercase());
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let mut new_body = Vec::new();
    let mut from_body = Vec::new();
    let mut ser_body = Vec::new();

    for field in fields {
        let name = &field.ident;
        let ty = &field.ty;
        let field_params = Field::from_field(field)?;

        let field_name = name
            .as_ref()
            .unwrap()
            .to_string()
            .to_uppercase()
            .replace('_', "-");

        let ser_part = if crate::is_vec(ty) {
            quote::quote! {
                if self.#name.len() == 1 {
                    s.push_str(&crate::ser::field(#field_name, &self.#name[0])?);
                }
                else if self.#name.attr().is_none() {
                    s.push_str(&crate::ser::field(#field_name, &self.#name)?);
                } else {
                    for v in &self.#name {
                        s.push_str(&crate::ser::field(#field_name, v)?);
                    }
                }
            }
        } else {
            quote::quote! {
                s.push_str(&crate::ser::field(#field_name, &self.#name)?);
            }
        };

        ser_body.push(ser_part);

        if field_params.ignore {
            continue;
        }

        let parser_fn = quote::quote! { crate::parser::#name(content_line)? };
        let parser = if crate::is_option(ty) {
            quote::quote! { component.#name = Some(#parser_fn) }
        } else if crate::is_vec(ty) {
            if field_params.append {
                quote::quote! { component.#name.append(&mut #parser_fn) }
            } else {
                quote::quote! { component.#name.push(#parser_fn) }
            }
        } else {
            let new_part = quote::quote! {
                #name: crate::parser::#name(
                    properties.iter().filter(|x| x.key == #field_name).last()
                    .ok_or_else(|| crate::Error::Parser(concat!("Missing field ", #field_name).to_string()))?
                    .clone()
                )?
            };

            new_body.push(new_part);
            quote::quote! { () }
        };

        let from_part = quote::quote! {
            #field_name => #parser
        };

        from_body.push(from_part);
    }

    let traits = quote::quote! {
        #[automatically_derived]
        #[doc(hidden)]
        impl #impl_generics TryFrom<Vec<crate::ContentLine>> for #name #ty_generics #where_clause {
            type Error = crate::Error;

            fn try_from(properties: Vec<crate::ContentLine>) -> crate::Result<Self> {
                let mut component = Self {
                    #(#new_body, )*
                    .. Default::default()
                };

                for content_line in properties {
                    match content_line.key.as_str() {
                        #(#from_body, )*
                        key => {
                            if key.starts_with("X-") {
                                component.x_prop.insert(key.to_string(), content_line);
                            } else {
                                component.iana_prop.insert(key.to_string(), content_line);
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
                crate::parser::#parser(&s.replace("\r\n ", ""))
                    .map_err(crate::Error::from)
                    .map(|(_, x)| x)
            }
        }

        #[automatically_derived]
        impl #impl_generics crate::ser::Serialize for #name #ty_generics #where_clause {
            fn component() -> Option<String> {
                #name_str.to_uppercase().into()
            }

            fn ical(&self) -> crate::Result<::std::string::String> {
                let mut s = String::new();
                let name = Self::component().unwrap();

                s.push_str("BEGIN:");
                s.push_str(&name);
                s.push_str("\r\n");
                #(#ser_body)*
                s.push_str("END:");
                s.push_str(&name);
                s.push_str("\r\n");

                Ok(s)
            }
        }
    };

    Ok(traits)
}
