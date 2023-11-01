#[derive(Clone, Default, Debug)]
pub(crate) struct Field {
    pub append: bool,
    pub ignore: bool,
}

impl Field {
    pub fn from_ast(field: &syn::Field) -> syn::Result<Self> {
        let mut param = Self::default();

        for item in flat_map(&field.attrs)? {
            match &item {
                // Parse #[component(append)]
                syn::NestedMeta::Meta(syn::Meta::Path(w)) if w == crate::symbol::APPEND => {
                    param.append = true;
                }
                // Parse #[component(ignore)]
                syn::NestedMeta::Meta(syn::Meta::Path(w)) if w == crate::symbol::IGNORE => {
                    param.ignore = true;
                }
                _ => continue,
            }
        }

        Ok(param)
    }
}

fn flat_map(attrs: &[syn::Attribute]) -> syn::Result<Vec<syn::NestedMeta>> {
    let mut items = Vec::new();

    for attr in attrs {
        items.append(&mut meta_items(attr)?);
    }

    Ok(items)
}

fn meta_items(attr: &syn::Attribute) -> syn::Result<Vec<syn::NestedMeta>> {
    if attr.path != crate::symbol::COMPONENT {
        return Ok(Vec::new());
    }

    match attr.parse_meta()? {
        syn::Meta::List(meta) => Ok(meta.nested.into_iter().collect()),
        _ => Err(syn::Error::new_spanned(attr, "expected #[component(...)]")),
    }
}
