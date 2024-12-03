use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "vitium" => "Err",
        "bene" => "Ok",
        "filum" => "String",
        "glossarium" => "HashMap",
        "criterium" => "Default",
        "peccare" => "Error",
        "facultas" => "Option",
        "quicquam" => "Some",
        "nihil" => "None",
        "eventum" => "Result",
        "Ipse" => "Self",
        "thesaurus" => "collections",
        "erogatio" => "println",
        "inerruptio" => "break",
        "iuxta_ire" => "async",
        "exspectare" => "await",
        "cyclus" => "loop",
        "proferre" => "move",
        "cista" => "crate",
        "capsula" => "Box",
        "incomparabilis_codex" => "unreachable_code",
        "cum" => "as",
        "uniformiter" => "const",
        "proprietas" => "trait",
        "typus" => "type",
        "periculosus" => "unsafe",
        "in" => "in",
        "ex" => "from",
        "dynamicus" => "dyn",
        "vacuefacere" => "unwrap",
        "adsuetus" => "default",
        "quam_incidens" => "as_ref",
        "ld" => "io",
        "extra" => "extern",
        "falsus" => "false",
        "functionaliter" => "fn",
        "trans_descriptus" => "super",
        "inserere" => "insert",

        // iterator functions
        "rursum" => "iter",
        "ad_rursum" => "into_iter",
        "comparturire" => "map",
        "planus_comparturire" => "flat_map",
        "plicare" => "fold",
        "ebibere" => "drain",
        "colligere" => "collect",
        "invenire" => "find",
        "capere" => "take",
        "merx" => "product",

        // ordering
        "clo" => "cmp",
        "Ordo" => "Ordering",
        "plus" => "Greater",
        "minus" => "Less",
        "parilis" => "Equal",
        "accere" => "get",
        "permittere" => "allow",
        "trepidatio" => "panic",
        "pars" => "mod",
        "mut" => "mut",
        "novus" => "new",
        "ubi" => "where",
        "pro" => "for",
        "adepto_vel_adde_cum" => "get_or_insert_with",
        "initium" => "main",
        "civilis" => "pub",
        "nullus" => None?,
        "reddere" => "return",
        "impl" => "impl",
        "ids" => "ref",
        "ammodulari" => "match",
        "si" => "if",
        "alioquin" => "else",
        "ipse" => "self",
        "sinere" => "let",
        "staticus" => "static",
        "structura" => "struct",
        "expectare" => "expect",
        "dum" => "while",
        "uti" => "use",
        "intro" => "into",
        "verus" => "true",
        "enumeratio" => "enum",

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
pub fn ferrugo(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
