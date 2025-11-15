mod component;
mod macros;
mod serialize;

#[proc_macro_derive(Component, attributes(component))]
pub fn component_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    component::impl_macro(&ast)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Serialize, attributes(serialize))]
pub fn serialize_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    serialize::impl_macro(&ast)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

macro_rules! component {
    ($name:ident, $ty:ty) => {
        #[proc_macro]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            let mut ast: macros::Map = syn::parse(input).unwrap();
            ast.ty = stringify!($ty).to_string();

            macros::impl_macro(&ast)
                .unwrap_or_else(syn::Error::into_compile_error)
                .into()
        }
    };
}

component!(audio, valarm::Audio);
component!(display, valarm::Display);
component!(email, valarm::Email);
component!(tz_daylight, vtimezone::Daylight);
component!(tz_standard, vtimezone::Standard);
component!(vcalendar, VCalendar);
component!(vevent, VEvent);
component!(vfreebusy, VFreebusy);
component!(vjournal, VJournal);
component!(vtimezone, VTimezone);
component!(vtodo, VTodo);

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
