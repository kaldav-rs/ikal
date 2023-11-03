#[derive(Clone, Default, Debug)]
pub(crate) struct Field {
    pub append: bool,
    pub ignore: bool,
}

impl Field {
    pub fn from_ast(field: &syn::Field) -> syn::Result<Self> {
        let mut param = Self::default();

        for item in flat_map(&field.attrs)? {
            if item == crate::symbol::APPEND {
                param.append = true;
            } else if item == crate::symbol::IGNORE {
                param.ignore = true;
            } else {
                continue;
            }
        }

        Ok(param)
    }
}

fn flat_map(attrs: &[syn::Attribute]) -> syn::Result<Vec<syn::Path>> {
    let mut items = Vec::new();

    for attr in attrs {
        items.append(&mut meta_items(attr)?);
    }

    Ok(items)
}

fn meta_items(attr: &syn::Attribute) -> syn::Result<Vec<syn::Path>> {
    let mut items = Vec::new();

    if attr.path() != crate::symbol::COMPONENT {
        return Ok(items);
    }

    attr.parse_nested_meta(|meta| {
        items.push(meta.path);
        Ok(())
    })?;

    Ok(items)
}
