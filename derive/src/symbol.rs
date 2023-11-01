#[derive(Clone, Copy)]
pub(crate) struct Symbol(&'static str);

pub(crate) const APPEND: Symbol = Symbol("append");
pub(crate) const COMPONENT: Symbol = Symbol("component");
pub(crate) const IGNORE: Symbol = Symbol("ignore");

impl PartialEq<Symbol> for syn::Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl<'a> PartialEq<Symbol> for &'a syn::Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}
