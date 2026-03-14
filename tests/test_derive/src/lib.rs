use proc_macro::{
    Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree,
};

/// A derive macro that generates a `value()` method returning `self.0`.
///
/// This exercises hygiene because the generated code references `self`
/// via real token spans (not format! + parse, which discards spans).
#[proc_macro_derive(GetValue)]
pub fn get_value(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();

    // Skip to the ident after `struct`/`enum`
    let name = loop {
        match iter.next() {
            Some(TokenTree::Ident(ident))
                if ident.to_string() == "struct" || ident.to_string() == "enum" =>
            {
                break match iter.next() {
                    Some(TokenTree::Ident(name)) => name,
                    _ => panic!("expected type name after keyword"),
                };
            }
            None => panic!("expected struct or enum"),
            _ => {}
        }
    };

    let span = Span::call_site();
    let self_ident = Ident::new("self", span);

    // impl $name { fn value(&self) -> u32 { self.0 } }
    TokenStream::from_iter([
        TokenTree::Ident(Ident::new("impl", span)),
        TokenTree::Ident(name),
        TokenTree::Group(Group::new(
            Delimiter::Brace,
            TokenStream::from_iter([
                // fn value(&self) -> u32
                TokenTree::Ident(Ident::new("fn", span)),
                TokenTree::Ident(Ident::new("value", span)),
                TokenTree::Group(Group::new(
                    Delimiter::Parenthesis,
                    TokenStream::from_iter([
                        TokenTree::Punct(Punct::new('&', Spacing::Alone)),
                        TokenTree::Ident(self_ident.clone()),
                    ]),
                )),
                TokenTree::Punct(Punct::new('-', Spacing::Joint)),
                TokenTree::Punct(Punct::new('>', Spacing::Alone)),
                TokenTree::Ident(Ident::new("u32", span)),
                // { self.0 }
                TokenTree::Group(Group::new(
                    Delimiter::Brace,
                    TokenStream::from_iter([
                        TokenTree::Ident(self_ident),
                        TokenTree::Punct(Punct::new('.', Spacing::Alone)),
                        TokenTree::Literal(Literal::u32_unsuffixed(0)),
                    ]),
                )),
            ]),
        )),
    ])
}
