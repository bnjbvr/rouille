ferrugo::ferrugo! {
    uti std::thesaurus::glossarium cum Glos;

    proprietas Res {
        functionaliter scribe(&ipse, clavis: filum, pretium: filum);
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

    civilis(cista) functionaliter fortasse(i: u32) -> facultas<eventum<u32, filum>> {
        si i % 2 == 1 {
            si i == 42 {
                quicquam(vitium(filum::ex("trepidatio")))
            } alioquin {
                quicquam(bene(33))
            }
        } alioquin {
            nihil
        }
    }

    iuxta_ire functionaliter exemplum() {
    }

    iuxta_ire functionaliter exemplum2() {
        exemplum().exspectare;
    }

    functionaliter initium() {
        sinere mut x = 31;

        ammodulari x {
            42 => {
                erogatio!("Jupiter!")
            }
            _ => erogatio!("functionaliter!")
        }

        pro i in 0..10 {
            sinere val = cyclus {
                inerruptio i;
            };

            dum nullus x < val {
                x += 1;
            }

            x = si sinere quicquam(eventum) = fortasse(i) {
                eventum.vacuefacere()
            } alioquin {
                12
            };
        }

        uti std::clo::Ordo;
        sinere horreum = vec![0; 100].rursum()
            .capere(50)
            .comparturire(|numerus| numerus %  7)
            .colligere::<Vec<i32>>()
            .ad_rursum()
            .plicare(0, |a, numerus| ammodulari numerus.clo(&a) {
                Ordo::plus => a - numerus,
                Ordo::minus => a + numerus,
                Ordo::parilis => a,
            });
    }
}
