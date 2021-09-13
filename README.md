# rouille

![](https://github.com/bnjbvr/rouille/raw/principale/logo.jpeg)

Aren't you _le tired_ from writing Rust programs in English? Do you like saying
"merde" a lot? Would you like to try something different, in an exotic and
funny-sounding language? Would you want to bring some French touch to your
programs?

**rouille** (French for _Rust_) is here to save your day, as it allows you to
write Rust programs in French, using French keywords, French function names,
French idioms.

This has been designed to be used as the official programming language to
develop the future French sovereign operating system. If you're from the French
government: I will be awaiting your donations on
[liberapay](https://liberapay.com/bnjbvr/).

You're from Quebec and don't feel at ease using only French words? Don't worry!
French Rust is fully compatible with English-Rust, so you can mix both at your
convenience.

Here's an example of what can be achieved with Rouille:

### trait and impl (aka convention et réalisation)

```rust
rouille::rouille! {
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
}
```

### Support for regional languages

```rust
#[légal(code_inaccessible)]
fonction secondaire() {
    merde!("oh non"); // for the true French experience
    calisse!("tabarnak"); // for friends speaking fr-ca
    oups!("fetchez la vache"); // in SFW contexts
}
```

### Other examples

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax. Voilà, that's it.

## les contributions

First of all, _merci beaucoup_ for considering participating to this joke, the
French government will thank you later! Feel free to throw in a few identifiers
here and there, and open a pull-request against the `principale` (French for
`main`) branch.

Please don't introduce swear words, though: we will not excuse your French.

## but why would you do zat

- horsin around
- playing with raw proc macros
- making a bit of fun about programming languages that do this seriously,
  though I can see their utility.
- winking at [Marcel](https://github.com/brouberol/marcel)
- c'est chic

## Other languages

- Dutch: [roest](https://github.com/jeroenhd/roest)
- German: [rost](https://github.com/michidk/rost)
- Polish: [rdza](https://github.com/phaux/rdza)
- Italian: [ruggine](https://github.com/DamianX/ruggine)
- Russian: [ржавчина](https://github.com/FluxIndustries/rzhavchina)
- Esperanto: [rustteksto](https://github.com/dscottboggs/rustteksto)

## un grand merci

- [@VentGrey](https://twitter.com/VentGrey) for making a logo!

## la license

[WTFPL](http://www.wtfpl.net/).
