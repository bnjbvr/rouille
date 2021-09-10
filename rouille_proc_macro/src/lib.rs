use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Arf" => "Err",
        "Bien" => "Ok",
        "Chaine" => "String",
        "Dictionnaire" => "HashMap",
        "Défaut" => "Default",
        "Erreur" => "Error",
        "PeutÊtre" => "Option",
        "Quelque" => "Some",
        "Rien" => "None",
        "Résultat" => "Result",
        "Soi" => "Self",
        "affiche" => "println",
        "arrête" => "break",
        "asynchrone" => "async",
        "attend" => "await",
        "boucle" => "loop",
        "bouge" => "move",
        "cagette" => "crate",
        "comme" => "as",
        "constant" => "const",
        "correspond" => "match",
        "dangereux" => "unsafe",
        "de" => "in",
        "depuis" => "from",
        "dynamique" => "dyn",
        "déballer" => "unwrap",
        "défaut" => "default",
        "es" => "io",
        "externe" => "extern",
        "faux" => "false",
        "fonction" => "fn",
        "génial" => "super",
        "réalisation" => "impl",
        "insérer" => "insert",
        "convention" => "trait",
        "lire" => "get",
        "module" => "mod",
        "mutable" => "mut",
        "nouveau" => "new",
        "où" => "where",
        "pour" => "for",
        "prendre_ou_insérer_avec" => "get_or_insert_with",
        "principale" => "main",
        "public" => "pub",
        "que" => None?,
        "renvoie" => "return",
        "si" => "if",
        "sinon" => "else",
        "soi" => "self",
        "soit" => "let",
        "statique" => "static",
        "structure" => "struct",
        "suppose" => "expect",
        "tant" => "while",
        "utilisons" => "use",
        "vrai" => "true",
        "énumération" => "enum",

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
pub fn rouille(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
