use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();
    let new_str = match ident_str.as_str() {
        "Falha" => "Err",
        "Ok" => "Ok",
        "Texto" => "String",
        "Dicionario" => "HashMap",
        "Padrão" => "Default",
        "Erro" => "Error",
        "Talvez" => "Option",
        "Algum" => "Some",
        "Nenhum" => "None",
        "Resultado" => "Result",
        "Próprio" => "Self",
        "exiba" => "println",
        "pare" => "break",
        "assíncrono" => "async",
        "aguarde" => "await",
        "laço" => "loop",
        "mova" => "move",
        "caixa" => "crate",
        "código_inalcançável" => "unreachable_code",
        "como" => "as",
        "constante" => "const",
        "característica" => "trait",
        "inseguro" => "unsafe",
        "em" => "in",
        "de" => "from",
        "dinâmico" => "dyn",
        "desembrulhar" => "unwrap",
        "padrão" => "default",
        "como_ref" => "as_ref",
        "es" => "io",
        "externo" => "extern",
        "falso" => "false",
        "função" => "fn",
        "super" => "super",
        "inserir" => "insert",
        "obter" => "get",
        "permitir" => "allow",
        "putz" | "eita" | "oxe" | "bah" => "panic",
        "módulo" => "mod",
        "mutável" => "mut",
        "novo" => "new",
        "onde" => "where",
        "para" => "for",
        "obter_ou_inserir_com" => "get_or_insert_with",
        "principal" => "main",
        "público" => "pub",
        "que" => None?,
        "retorne" => "return",
        "implementação" => "impl",
        "ref" => "ref",
        "escolha" => "match",
        "se" => "if",
        "senão" => "else",
        "próprio" => "self",
        "seja" => "let",
        "estático" => "static",
        "estrutura" => "struct",
        "esperar" => "expect",
        "enquanto" => "while",
        "usar" => "use",
        "converter" => "into",
        "verdadeiro" => "true",
        "enumeração" => "enum",
        "Grupo" => "Group",
        "Identificador" => "Ident",
        "FluxoDeTokens" => "TokenStream",
        "ArvoreDeTokens" => "TokenTree",
        "para_texto" => "to_string",
        "como_texto" => "as_str",
        "alcançar" => "span",
        "Vetor" => "Vec",
        "fluxo" => "stream",
        "empurrar" => "push",
        "estender" => "extend",
        "delimitador" => "delimiter",
        "Pontuação" => "Punct",
        "Literal" => "Literal",
        "macro_procedural" => "proc_macro",
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
pub fn ferrugem(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
