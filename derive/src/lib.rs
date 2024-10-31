mod component;

#[proc_macro_derive(Component, attributes(component))]
pub fn component_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    component::impl_macro(&ast)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
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
