# ferrugo

<p align="center"><img src="logo.jpg" alt="Rust Latin Logo"></p>

Aren't you _defessus_ from writing Rust programs in English? Do you like saying
"Lorem ipsum" a lot? Would you like to try something different, in an exotic and
funny-sounding language? Would you want to bring some Latin touch to your
programs?

**ferrugo** (Latin for _Rust_) is here to save your day, as it allows you to
write Rust programs in Latin, using Latin keywords, Latin function names,
Latin idioms.

You don't feel at ease using only Latin words? Don't worry!
Latin Rust is fully compatible with English-Rust, so you can mix both at your
convenience.

Here's an example of what can be achieved with ferrugo:

## struct and impl (aka _lex et transplantatis_)

```rust
ferrugo::ferrugo! {
    uti std::thesaurus::glossarium cum Glos;

    proprietas Res {
        functionaliter scribe(&ipse, clavis: filum, wert: filum);
        functionaliter lege(&ipse, clavis: filum) -> eventum<facultas<&filum>, filum>;
    }

    staticus mut GLOSSARIUM: facultas<Glos<filum, filum>> = nihil;

    structura Rectus;

    impl Res pro Rectus {

        functionaliter scribe(&ipse, clavis: filum, pretium: filum) {
            sinere glos = periculosus {
                GLOSSARIUM.adepto_vel_adde_cum(criterium::adsuetus)
            };
            glos.inserere(clavis, pretium);
        }

        functionaliter lege(&ipse, clavis: filum) -> eventum<facultas<&filum>, filum> {
            si sinere quicquam(glos) = periculosus { GLOSSARIUM.quam_incidens() } {
                bene(glos.accere(&clavis))
            } alioquin {
                vitium("arcessite glossarium!".intro())
            }
        }
    }
}
```

## Other examples

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax. Bene!

## but why would you do id?

* they below can do it, so we can as well!

- French: [rouille ](https://github.com/bnjbvr/rouille)
- Dutch: [roest](https://github.com/jeroenhd/roest)
- German: [rost](https://github.com/michidk/rost)
- Polish: [rdza](https://github.com/phaux/rdza)
- Italian: [ruggine](https://github.com/DamianX/ruggine)
- Russian: [Ржавый](https://github.com/Sanceilaks/rzhavchina)
- Esperanto: [rustteksto](https://github.com/dscottboggs/rustteksto)
- Hindi: [zung](https://github.com/rishit-khandelwal/zung)
- Hungarian: [rozsda](https://github.com/jozsefsallai/rozsda)
- Chinese: [xiu (锈)](https://github.com/lucifer1004/xiu)
- Spanish: [rustico](https://github.com/UltiRequiem/rustico)
- Korean: [Nok (녹)](https://github.com/Alfex4936/nok)
- Finnish: [ruoste](https://github.com/vkoskiv/ruoste)
- Arabic: [sada](https://github.com/LAYGATOR/sada)
- Turkish: [pas](https://github.com/ekimb/pas)
- Vietnamese: [gỉ](https://github.com/Huy-Ngo/gir)
- Japanese: [sabi (錆)](https://github.com/yuk1ty/sabi)
- Danish: [rust?](https://github.com/LunaTheFoxgirl/rust-dk)
- Marathi: [gan̄ja](https://github.com/pranavgade20/ganja)
- Romanian: [rugină](https://github.com/aionescu/rugina)
- Czech: [rez](https://github.com/radekvit/rez)
- Ukrainian: [irzha](https://github.com/brokeyourbike/irzha)
- Bulgarian: [ryzhda](https://github.com/gavadinov/ryzhda)
- Slovak: [hrdza](https://github.com/TheMessik/hrdza)
- Catalan: [rovell](https://github.com/gborobio73/rovell)
- Corsican: [rughjina](https://github.com/aldebaranzbradaradjan/rughjina)
- Indonesian: [karat](https://github.com/annurdien/karat)
- Lithuanian: [rūdys](https://github.com/TruncatedDinosour/rudys)
- Greek: [skouriasmeno](https://github.com/devlocalhost/skouriasmeno)
- Thai: [sanim (สนิม)](https://github.com/korewaChino/sanim)
- Swiss: [roeschti](https://github.com/Georg-code/roeschti)
- Swedish: [rost](https://github.com/vojd/rost/)
- Croatian: [hrđa](https://github.com/njelich/hrdja)
- Persian: [zangar (زنگار)](https://github.com/ui-ce/zangar)
- Malagasy: [arafesina](https://github.com/luckasRanarison/arafesina)
- All of the above: [unirust](https://github.com/charyan/unirust)

## Cooperare

Feel free to throw in a few identifiers
here and there, and open a pull-request against the `authenta` (Latin for
`main`). The initial translation was made by [pianoman911](https://github.com/pianoman911/).

## Fructus lex

[WTFPL](http://www.wtfpl.net/), see [LICENTIA](./LICENTIA) as translated version.
