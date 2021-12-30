hrdza::hrdza! {
    extern prepravka hrdza;

    pouzi std::collections::Slovnik ako Slk;

    vlastnost KlucHodnota {
        funkcia pis(&ja, kluc: Retaz, hodnota: Retaz);
        funckia citaj(&ja, kluc: Retaz) -> Vysledok<Moznost<&Retaz>, Retaz>;
    }

    pevny menny SLOVNIK: Moznost<Slk<Retaz, Retaz>> = Ziaden;

    struktura Konkretny;

    realizacia KlucHodnota pre Konkretny {
        funkcia pis(&ja, kluc: Retaz, hodnota: Retaz) {
            nech slvk = nebezpecny {
                SLOVNIK.daj_alebo_vloz_s(Zaklad::zaklad)
            };
            slvk.vloz(kluc, hodnota);
        }
        funkcia citaj(&ja, kluc: Retaz) -> Vysledok<Moznost<&Retaz>, Retaz> {
            ak nech Nejaky(slvk) = nebezpecny { SLOVNIK.ako_odkaz() } {
                Dobre(slvk.dostan(&kluc))
            } inak {
                Chyba("ziskat zo slovnika".do())
            }
        }
    }

    verejny(prepravka) funkcia mozno(i: u32) -> Moznost<Vysledok<u32, Retaz>> {
        ak i % 2 == 1 {
            ak i == 42 {
                Nejaky(Chyba(Retaz::od("blaze it")))
            } inak {
                Nejaky(Dobre(33))
            }
        } inak {
            Ziaden
        }
    }

    zaroven funkcia priklad() {
    }

    zaroven funkcia priklad2() {
        priklad().pockaj;
    }

    funkcia hlavny() {
        nech menny x = 31;

        podobny x {
            42 => {
                vytlac_rdk!("chobotnica")
            }
            _ => vytlac_rdk!("oktopus")
        }

        pre i dnu 0..10 {
            nech val = cyklus {
                prelom i;
            };

            dokym x < val {
                x += 1;
            }

            x = ak nech Nejaky(vysledok) = mozno(i) {
                vysledok.rozbal()
            } inak {
                12
            };
        }
    }
}