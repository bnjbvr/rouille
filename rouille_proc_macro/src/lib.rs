rouille_compilogenese::rouille! {
    utilisons macro_procédurale::{Groupe, Identifiant, FluxDeJetons, ArbreDeJetons};

    fonction remplacer_identifiant(identifiant: Identifiant) -> PeutÊtre<ArbreDeJetons> {
        soit identifiant_chaine = identifiant.vers_chaine();

        soit nouvelle_chaine = selon identifiant_chaine.en_tant_que_chaine() {
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
            "vers_chaine" => "to_string",
            "en_tant_que_chaine" => "as_str",
            "portée" => "span",
            "Tableau" => "Vec",
            "flux" => "stream",
            "pousser" => "push",
            "étendre" => "extend",
            "délimiteur" => "delimiter",
            "Ponctuation" => "Punct",
            "Litéral" => "Literal",
            "macro_procédurale" => "proc_macro",
            _ => &identifiant_chaine,
        };

        soit nouvel_identifiant = Identifiant::nouveau(nouvelle_chaine, identifiant.portée());
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
