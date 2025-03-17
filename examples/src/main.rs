use ferrugem::ferrugem;

ferrugem! {
    externo caixa ferrugem;
    use std::collections::Dicionario como Dic;

    característica ChaveValor {
        função escrever(&próprio, chave: Texto, valor: Texto);
        função obter(&próprio, chave: Texto) -> Resultado<Talvez<&Texto>, Texto>;
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

    público(caixa) função pode_ser(i: u32) -> Talvez<Resultado<u32, Texto>> {
        se i % 2 == 1 {
            se i == 42 {
                Algum(Falha(Texto::de("pane")))
            } senão {
                Algum(Ok(33))
            }
        } senão {
            Nenhum
        }
    }

    assíncrono função exemplo() {
    }

    assíncrono função exemplo2() {
        exemplo().aguarde;
    }

    função principal() {
        seja mutável x = 31;

        escolha x {
            42 => {
                exiba!("pão de queijo")
            }
            _ => exiba!("lá ele")
        }

        para i em 0..10 {
            seja val = laço {
                pare i;
            };

            enquanto x < val {
                x += 1;
            }

            x = se seja Algum(resultado) = pode_ser(i) {
                resultado.desembrulhar()
            } senão {
                12
            };
        }
        //secundaria();
    }

    #[permitir(código_inalcançável)]
    função secundaria() {
        putz!("deu ruim"); // para a verdadeira experiência brasileira
        oxe!("vixi mainha"); // para amigos falando pt-br regional
        bah!("nossa senhora"); // outras gírias
    }
}
