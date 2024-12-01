use quote::ToTokens as _;

fn ikal() -> proc_macro2::TokenStream {
    match (
        proc_macro_crate::crate_name("ikal"),
        std::env::var("CARGO_CRATE_NAME").as_deref(),
    ) {
        (Ok(proc_macro_crate::FoundCrate::Itself), Ok("ikal")) => quote::quote!(crate),
        (Ok(proc_macro_crate::FoundCrate::Name(name)), _) => {
            let ident = proc_macro2::Ident::new(&name, proc_macro2::Span::call_site());
            quote::quote!(::#ident)
        }
        _ => quote::quote!(::ikal),
    }
}

pub(crate) fn impl_macro(ast: &Map) -> syn::Result<proc_macro2::TokenStream> {
    let ikal = ikal();

    Ok(quote::quote! {
        (|| {
            #[allow(clippy::needless_update)]
            let v = #ast;

            Ok::<_, #ikal::Error>(v)
        })()
    })
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Map {
    pub ty: String,
    entries: Vec<Entry>,
}

impl syn::parse::Parse for Map {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut entries = Vec::new();

        while !input.is_empty() {
            let key = if let Ok(key) = input.parse::<syn::Ident>() {
                key
            } else {
                panic!("Key must be an identifier!");
            };

            if input.parse::<syn::Token![,]>().is_ok() {
                entries.push(Entry {
                    key: key.clone(),
                    value: Value::None,
                });
                continue;
            }

            input.parse::<syn::Token![:]>()?;

            let value = input.parse()?;

            entries.push(Entry { key, value });

            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(Self {
            ty: String::new(),
            entries,
        })
    }
}

impl quote::ToTokens for Map {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let entries = self
            .entries
            .iter()
            .map(|x| TypedEntry(self.ty.clone(), x.clone()));
        let ikal = ikal();
        let ty = self.ty.parse::<proc_macro2::TokenStream>().unwrap();

        let v = quote::quote! {
            #ikal::#ty {
                #( #entries, )*
                .. Default::default()
            }
        };
        tokens.extend(v);
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Entry {
    key: syn::Ident,
    value: Value,
}

struct TypedEntry(String, Entry);

impl quote::ToTokens for TypedEntry {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let key = &self.1.key;

        if matches!(self.1.value, Value::None) {
            tokens.extend(key.to_token_stream());
        } else {
            let value = TypedValue(self.0.clone(), key.to_string(), self.1.value.clone());

            tokens.extend(quote::quote! {
                #key: #value
            });
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Value {
    None,
    Single(proc_macro2::TokenStream),
    Object(Map),
    Collection(Vec<Self>),
}

impl syn::parse::Parse for Value {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let value = if input.peek(syn::token::Brace) {
            let content;
            syn::braced!(content in input);

            let mut object = content.parse::<Map>()?;
            object.ty = "".to_string();
            Value::Object(object)
        } else if input.peek(syn::token::Bracket) {
            let content;
            syn::bracketed!(content in input);

            Value::Collection(
                content
                    .parse_terminated(Value::parse, syn::Token![,])?
                    .iter()
                    .cloned()
                    .collect(),
            )
        } else {
            Value::Single(parse_value(input)?)
        };

        Ok(value)
    }
}

fn parse_value(input: syn::parse::ParseStream) -> syn::Result<proc_macro2::TokenStream> {
    let v = if let Ok(value) = input.parse::<syn::Expr>() {
        value.to_token_stream()
    } else if let Ok(value) = input.parse::<syn::Ident>() {
        value.to_token_stream()
    } else if let Ok(value) = input.parse::<syn::Lit>() {
        value.to_token_stream()
    } else {
        panic!("Value must be either a literal or an identifier!");
    };

    Ok(v)
}

struct TypedValue(String, String, Value);

impl quote::ToTokens for TypedValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self.2 {
            Value::None => (),
            Value::Single(value) => {
                let v = if let Some(ty) = Type::r#enum(&self.0, &self.1) {
                    quote::quote! { #ty::#value }
                } else {
                    value.to_token_stream()
                };

                let v = if Type::should_parsed(&self.0, &self.1) {
                    if self.1 == "duration" {
                        let ikal = ikal();
                        quote::quote! { #ikal::parse_duration(#v)? }
                    } else {
                        quote::quote! { #v.parse()? }
                    }
                } else if Type::should_convert(&self.0, &self.1) {
                    quote::quote! { #v.into() }
                } else {
                    v.to_token_stream()
                };

                let v = if Type::is_option(&self.0, &self.1) {
                    quote::quote! { Some(#v) }
                } else {
                    v
                };

                tokens.extend(v);
            }
            Value::Collection(values) => {
                let v = values
                    .iter()
                    .map(|x| TypedValue(self.0.clone(), self.1.clone(), x.clone()));
                tokens.extend(quote::quote! { vec![#(#v, )*] });
            }
            Value::Object(object) => {
                let mut object = object.clone();
                object.ty = match (self.0.as_str(), self.1.as_str()) {
                    (_, "alarms") => "VAlarm".to_string(),
                    (_, "daylight") => "vtimezone::Daylight".to_string(),
                    (_, "events") => "VEvent".to_string(),
                    (_, "geo") => "Geo".to_string(),
                    (_, "rrule") => "Recur".to_string(),
                    (_, "rstatus") => "RequestStatus".to_string(),
                    (_, "standard") => "vtimezone::Standard".to_string(),
                    (ty, field) => todo!("{ty}.{field}"),
                };

                if Type::is_option(&self.0, &self.1) {
                    tokens.extend(quote::quote! { Some( #object ) });
                } else {
                    tokens.extend(object.to_token_stream());
                }
            }
        }
    }
}

struct Type;

impl Type {
    fn is_option(ty: &str, field: &str) -> bool {
        (ty == "VEvent" && field == "description")
            || (ty == "VFreebusy" && field == "dtstart")
            || (ty != "valarm::Email" && field == "summary")
            || (ty == "VFreebusy" && field == "contact")
            || (ty == "VTodo" && field == "dtstart")
            || matches!(
                field,
                "class"
                    | "completed"
                    | "created"
                    | "dtend"
                    | "due"
                    | "duration"
                    | "extdata"
                    | "geo"
                    | "last_modified"
                    | "location"
                    | "organizer"
                    | "priority"
                    | "recurid"
                    | "rrule"
                    | "sequence"
                    | "status"
                    | "transp"
                    | "tzurl"
                    | "url"
            )
    }

    fn should_convert(_ty: &str, field: &str) -> bool {
        !matches!(field, "lat" | "lon" | "sequence")
    }

    fn should_parsed(_ty: &str, field: &str) -> bool {
        matches!(
            field,
            "completed"
                | "dtstamp"
                | "dtstart"
                | "duration"
                | "created"
                | "due"
                | "last_modified"
                | "dtend"
                | "tzoffsetfrom"
                | "tzoffsetto"
                | "trigger"
                | "geo"
                | "rrule"
                | "rstatus"
                | "rdate"
                | "exdate"
                | "recurid"
        )
    }

    fn r#enum(_ty: &str, field: &str) -> Option<proc_macro2::TokenStream> {
        let ikal = ikal();
        let ty = match field {
            "class" => "Class",
            "freq" => "Freq",
            "status" => "Status",
            "transp" => "TimeTransparency",
            _ => return None,
        }
        .parse::<proc_macro2::TokenStream>()
        .unwrap();

        Some(quote::quote! { #ikal::#ty })
    }
}
