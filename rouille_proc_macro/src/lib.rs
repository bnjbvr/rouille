rouille_compilogenese::rouille! {
    utilisons macro_procédurale::{Groupe, Identifiant, FluxDeJetons, ArbreDeJetons};

    fonction remplacer_identifiant(identifiant: Identifiant) -> PeutÊtre<ArbreDeJetons> {
        soit identifiant_chaîne = identifiant.vers_chaîne();

        soit nouvelle_chaîne = selon identifiant_chaîne.en_tant_que_chaîne() {
            "Arf" => "Err",
            "Bien" => "Ok",
            "Chaîne" => "String",
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
            "code_inaccessible" => "unreachable_code",
            "comme" => "as",
            "constant" => "const",
            "convention" => "trait",
            "dangereux" => "unsafe",
            "de" => "in",
            "depuis" => "from",
            "dynamique" => "dyn",
            "déballer" => "unwrap",
            "défaut" => "default",
            "en_réf" => "as_ref",
            "es" => "io",
            "externe" => "extern",
            "faux" => "false",
            "fonction" => "fn",
            "génial" => "super",
            "insérer" => "insert",
            "lire" => "get",
            "légal" => "allow",
            "merde" | "calisse" | "oups" => "panic",
            "module" => "mod",
            "mutable" => "mut",
            "nouveau" => "new",
            "où" => "where",
            "pour" => "for",
            "prendre_ou_insérer_avec" => "get_or_insert_with",
            "principale" => "main",
            "public" => "pub",
            "que" => Rien?,
            "renvoie" => "return",
            "réalisation" => "impl",
            "réf" => "ref",
            "selon" => "match",
            "si" => "if",
            "sinon" => "else",
            "soi" => "self",
            "soit" => "let",
            "statique" => "static",
            "structure" => "struct",
            "suppose" => "expect",
            "tant" => "while",
            "utilisons" => "use",
            "vers" => "into",
            "vrai" => "true",
            "énumération" => "enum",
            "Groupe" => "Group",
            "Identifiant" => "Ident",
            "FluxDeJetons" => "TokenStream",
            "ArbreDeJetons" => "TokenTree",
            "vers_chaîne" => "to_string",
            "en_tant_que_chaîne" => "as_str",
            "portée" => "span",
            "Tableau" => "Vec",
            "tab" => "vec",
            "flux" => "stream",
            "pousser" => "push",
            "étendre" => "extend",
            "délimiteur" => "delimiter",
            "Ponctuation" => "Punct",
            "Litéral" => "Literal",
            "macro_procédurale" => "proc_macro",
            "LectureBufferisée" => "BufRead",
            "Lecture" => "Read",
            "Écriture" => "Write",
            "entrée_sortie" => "io",
            "entrée" => "stdin",
            "sortie" => "stdout",
            "lire_ligne" => "read_line",
            "écrire_ligne" => "write_line",
            "écrire" => "write",
            "est_vide" => "is_empty",
            "est" => "is",
            "verrouiller" => "lock",
            "octets" => "bytes",
            "chaîne" => "str",
            "lire_exactement" => "read_exact",
            "sépare" => "split",
            "sépare_avec_les_espaces" => "split_whitespace",
            "en_tant_que" => "as",
            "ensuite" => "then",
            "ensuite_quelque" => "then_some",
            "déballer_ou" => "unwrap_or",
            "déballer_ou_avec" => "unwrap_or_else",
            "depuis_chaîne" => "from_str",
            "Depuis" => "From",
            "bien" => "ok",
            "bien_ou" => "ok_or",
            "bien_ou_avec" => "ok_or_else",
            "en_tant_que_déréférencé" => "as_deref",
            "en_tant_que_déréférencé_mutable" => "as_mut_deref",
            "itérable" => "iter",
            "itérable_mut" => "iter_mut",
            "énumérer" => "enumerate",
            "filtre" => "filter",
            "filtre_applique" => "filter_map",
            "applique" => "map",
            "applique_ou" => "map_or",
            "applique_ou_avec" => "map_or_else",
            "applique_arf" => "map_err",
            "pour_chaque" => "for_each",
            "essayer_pour_chaque" => "try_for_each",
            "applatir" => "flatten",
            "applatir_en_appliquant" => "flat_map",
            "compter" => "count",
            "trouve" => "find",
            "trouve_en_appliquant" => "find_map",
            "tous" | "toutes" => "all",
            "certains" | "certaines" => "any",
            "récupérer" => "collect",
            "copié" => "copied",
            "cloné" => "cloned",
            "Copiable" => "Copy",
            "Clonable" => "Clone",
            "processus" => "process",
            "interpète" => "parse",
            "taille_non_signée" => "usize",
            "taille_signée" => "isize",
            "entier_non_signé_8" => "u8",
            "entier_non_signé_32" => "u32",
            "entier_non_signé_64" => "u64",
            "entier_signé_8" => "i8",
            "entier_signé_32" => "i32",
            "entier_signé_64" => "i64",
            "flottant_32" => "f32",
            "flottant_64" => "f64",
            "caractère" => "char",
            "bibliothèque_standard" => "std",
            "arf_affiche" => "eprintln",
            "dégage" => "exit",
            "Eg" => "Eq",
            "EgPartiel" => "PartialEq",
            "longueur" => "len",
            _ => &identifiant_chaîne,
        };

        soit nouvel_identifiant = Identifiant::nouveau(nouvelle_chaîne, identifiant.portée());
        Quelque(ArbreDeJetons::Identifiant(nouvel_identifiant))
    }

    fonction remplacer_arbre(jeton: ArbreDeJetons, sortie: &mutable Tableau<ArbreDeJetons>) {
        selon jeton {
            ArbreDeJetons::Groupe(groupe) => {
                soit mutable groupe_elements = Tableau::nouveau();
                remplacer_le_flux(groupe.flux(), &mutable groupe_elements);
                soit mutable nouveau_flux = FluxDeJetons::nouveau();
                nouveau_flux.étendre(groupe_elements);
                sortie.pousser(ArbreDeJetons::Groupe(Groupe::nouveau(groupe.délimiteur(), nouveau_flux)));
            }
            ArbreDeJetons::Identifiant(identifiant) => {
                si soit Quelque(identifiant) = remplacer_identifiant(identifiant) {
                    sortie.pousser(identifiant);
                }
            }
            ArbreDeJetons::Ponctuation(..) | ArbreDeJetons::Litéral(..) => {
                sortie.pousser(jeton);
            }
        }
    }

    fonction remplacer_le_flux(arbre_de_jetons: FluxDeJetons, sortie: &mutable Tableau<ArbreDeJetons>) {
        pour jeton de arbre_de_jetons {
            remplacer_arbre(jeton, sortie)
        }
    }

    #[macro_procédurale]
    public fonction rouille(élément: FluxDeJetons) -> FluxDeJetons {
        soit mutable retourné = Tableau::nouveau();
        remplacer_le_flux(élément, &mutable retourné);
        soit mutable sortie = FluxDeJetons::nouveau();
        sortie.étendre(retourné);
        sortie
    }
}
