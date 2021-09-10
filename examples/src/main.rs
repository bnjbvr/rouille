rouille::rouille! {
    externe cagette rouille;

    utilisons std::collections::Dictionnaire comme Dico;

    convention CléValeur {
        fonction écrire(&soi, clé: Chaine, valeur: Chaine);
        fonction lire(&soi, clé: Chaine) -> PeutÊtre<&Chaine>;
    }

    statique mutable DICTIONNAIRE: PeutÊtre<Dico<Chaine, Chaine>> = Rien;

    structure Concrète;

    réalisation CléValeur pour Concrète {
        fonction écrire(&soi, clé: Chaine, valeur: Chaine) {
            soit dico = dangereux {
                DICTIONNAIRE.prendre_ou_insérer_avec(Défault::défault)
            };
            dico.insérer(clé, valeur);
        }
        fonction lire(&soi, clé: Chaine) -> PeutÊtre<&Chaine> {
            soit dico = dangereux {
                DICTIONNAIRE.prendre_ou_insérer_avec(Défault::défault)
            };
            dico.lire(&clé)
        }
    }

    public(cagette) fonction peut_etre(i: u32) -> PeutÊtre<Résultat<u32, Chaine>> {
        si i % 2 == 1 {
            si i == 42 {
                Quelque(Arf(Chaine::depuis("merde")))
            } sinon {
                Quelque(Bien(33))
            }
        } sinon {
            Rien
        }
    }

    asynchrone fonction exemple() {
    }

    asynchrone fonction exemple2() {
        exemple().attend;
    }

    fonction principale() {
        soit mutable x = 31;

        correspond x {
            42 => {
                affiche!("omelette du fromage")
            }
            _ => affiche!("voila")
        }

        pour i de 0..10 {
            soit val = boucle {
                arrête i;
            };
            tant que x < val {
                x += 1;
            }
            x = si soit Quelque(resultat) = peut_etre(i) {
                resultat.déballer()
            } else { 12 };
        }
    }
}
