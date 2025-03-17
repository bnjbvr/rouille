# ferrugem

![](https://github.com/gabrielpacheco23/ferrugem/raw/main/logo.png)


Aren't you _cansado_ from writing Rust programs in English? Do you like saying
"putz" a lot? Would you like to try something different, in an exotic and
funny-sounding language? Would you want to bring some Brazilian/Portuguese touch to your
programs?

**ferrugem** (Portuguese for _Rust_) is here to save your day, as it allows you to
write Rust programs in Portuguese, using Portuguese keywords, Portuguese function names,
Brazilian Portuguese idioms.

This has been designed to be used as the official programming language to
develop the future Brazilian sovereign operating system. 

Here's an example of what can be achieved with Ferrugem:


```rust
ferrugem! {
    use std::collections::Dicionario como Dic;
    característica ChaveValor {
        função escrever(&próprio, chave: Texto, valor: Texto);
        função obter(&próprio, chave: Texto) -> Talvez<&Texto>;
    }

    estático mutável DICIONARIO: Talvez<Dic<Texto, Texto>> = Nenhum;
    estrutura Concreta;

    implementação ChaveValor para Concreta {
        função escrever(&próprio, chave: Texto, valor: Texto) {
            seja dic = inseguro {
                DICIONARIO.obter_ou_inserir_com(Padrão::padrão)
            };
            dic.inserir(chave, valor);
        }

        função obter(&próprio, chave: Texto) -> Resultado<Talvez<&Texto>, Texto> {
            se seja Algum(dic) = inseguro { DICIONARIO.como_ref() } {
                Ok(dic.obter(&chave))
            } senão {
                Falha("busque o dicionário".converter())
            }
        }
    }
}
```

### Support for regional languages

```rust
#[permitir(código_inalcançável)]
função secundaria() {
    putz!("deu ruim");     // para a verdadeira experiência brasileira
    oxe!("vixi mainha");   // para amigos falando pt-br nordestino
    bah!("que isso tchê"); // para amigos falando pt-br sulista 
}
```

### Other examples

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax.

## Contributions

Feel free to throw in a few identifiers here and there, and open a pull-request against the `main` branch.

Please don't introduce swear words.

## Other languages

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
- Latin: [ferrugo](https://github.com/pianoman911/ferrugo)
- Norwegian: [korrosjon](https://github.com/datagutt/korrosjon)
- All of the above: [unirust](https://github.com/charyan/unirust)

## Obrigado
Thanks [@bnjbvr](https://github.com/bnjbvr) for the original idea, as well as the base repository.

## Licença (license)
[WTFPL](http://www.wtfpl.net/)
