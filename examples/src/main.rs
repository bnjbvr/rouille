rouille::rouille! {
    externe cagette rouille;

    utilisons std::collections::Dictionnaire comme Dico;

    convention CléValeur {
        fonction écrire(&soi, clé: Chaine, valeur: Chaine);
        fonction lire(&soi, clé: Chaine) -> Résultat<PeutÊtre<&Chaine>, Chaine>;
    }

    statique mutable DICTIONNAIRE: PeutÊtre<Dico<Chaine, Chaine>> = Rien;

    structure Concrète;

    réalisation CléValeur pour Concrète {
        fonction écrire(&soi, clé: Chaine, valeur: Chaine) {
            soit dico = dangereux {
                DICTIONNAIRE.prendre_ou_insérer_avec(Défaut::défaut)
            };
            dico.insérer(clé, valeur);
        }
        fonction lire(&soi, clé: Chaine) -> Résultat<PeutÊtre<&Chaine>, Chaine> {
            si soit Quelque(dico) = dangereux { DICTIONNAIRE.en_réf() } {
                Bien(dico.lire(&clé))
            } sinon {
                Arf("fetchez le dico".vers())
            }
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

        selon x {
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
            } sinon {
                12
            };
        }

        //secondaire();
    }

    #[légal(code_inaccessible)]
    fonction secondaire() {
        merde!("oh non"); // for the true French experience
        calisse!("tabernacle"); // for friends speaking fr-ca
        oups!("fetchez la vache"); // in SFW contexts
    }
}
