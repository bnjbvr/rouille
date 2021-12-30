use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Chyba" => "Err",
        "Dobre" => "Ok",
        "Retaz" => "String",
        "rtz" => "str",
        "Slovnik" => "HashMap",
        "Zaklad" => "Default",
        "Chyba" => "Error",
        "Moznost" => "Option",
        "Nejaky" => "Some",
        "Ziaden" => "None",
        "Vysledok" => "Result",
        "Ja" => "Self",
        "vytlac_rdk" => "println",
        "prelom" => "break",
        "zaroven" => "async",
        "pockaj" => "await",
        "cyklus" => "loop",
        "posun" => "move",
        "prepravka" => "crate",
        "ako" => "as",
        "nemenny" => "const",
        "podobny" => "match",
        "nebezpecny" => "unsafe",
        "dnu" => "in",
        "od" => "from",
        "dynamicky" => "dyn",
        "rozbal" => "unwrap",
        "zaklad" => "default",
        "vstup_vystup" => "io",
        "vonkajsy" => "extern",
        "nepravda" => "false",
        "funkcia" => "fn",
        "vrchny" => "super",
        "realizacia" => "impl",
        "vloz" => "insert",
        "vlastnost" => "trait",
        "dostan" => "get",
        "modul" => "mod",
        "menny" => "mut",
        "novy" => "new",
        "kde" => "where",
        "pre" => "for",
        "daj_alebo_vloz_s" => "get_or_insert_with",
        "hlavny" => "main",
        "verejny" => "pub",
        "vrat" => "return",
        "ak" => "if",
        "inak" => "else",
        "ja" => "self",
        "nech" => "let",
        "pevny" => "static",
        "struktura" => "struct",
        "ocakavaj" => "expect",
        "dokym" => "while",
        "pouzi" => "use",
        "pravda" => "true",
        "vymenovanie" => "enum",
        "do" => "into",
        "odkaz" => "ref",
        "ako_odkaz" => "as_ref",
        "nedostupny_kod" => "unreachable_code",
        "dovol" => "allow",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn hrdza(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}