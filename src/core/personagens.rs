use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Personagem{
    pub nome: String,
    pub descricao: String,
    pub atributos: Atributos
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Atributos{
    pub vida_atual: i32,
    pub vida_maxima: i32,
    pub efeitos: Vec<Efeito>,
    pub distancia: i32,
    pub visao: i32,
    pub limitecompra: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Efeito{
    MaxBangs(i32),
    MaxCartas(i32),
    MaxVida(i32),
    JoudonnaisBarril,
    KitCarlsonCartas,
    LuckyDukeChecagem,
    PaulRegretDistancia,
    PedroRamirezCompra,
    RoseDoolanVisao,
    BartCassidyCompra,
    SidKetchumDescarte,
    BlackJackCompra,
    SlabTheKillerBang,
    CalamityJanetBangEsquiva,
    SuzyLafayetteCompra,
    ElGringoRoubo,
    VultureSamRoubo
}

pub async fn lista_personagens()
    -> Json<Vec<Personagem>>{
    let personagens =
    vec![
        Personagem{
            nome: "Willy The Kid".to_string(),
            descricao: "Pode jogar quantas cartas de 'Bang!' quiser na sua vez.".to_string(),
            atributos: Atributos{
                vida_atual: 4,
                vida_maxima: 4,
                efeitos: vec![Efeito::MaxBangs(120)],
                distancia: 1,
                visao: 1,
                limitecompra: 2
            }
        },
        Personagem{
            nome: "Jesse Jones".to_string(),
            descricao: "Pode comprar a sua primeira carta
             (das duas que são compradas no início de cada rodada) da mão de um jogador.".to_string(),
            atributos: Atributos{
                vida_atual: 3,
                vida_maxima: 3,
                efeitos: vec![Efeito::KitCarlsonCartas],
                distancia: 1,
                visao: 1,
                limitecompra: 2
            }
        },
        Personagem{
            nome: "Joudonnais".to_string(),
            descricao: "Tem o efeito do Barril. Ele pode equipar a carta do Barril, 
            caso caso ele a tenha. Se ele fizer isso, toda vez que derem um Bang nele, 
            ele virará até 2 cartas para saber se tomou o dano ou não.".to_string(),
            atributos: Atributos{
                vida_atual: 4,
                vida_maxima: 4,
                efeitos: vec![Efeito::JoudonnaisBarril],
                distancia: 1,
                visao: 1,
                limitecompra: 2
            }
        },
        Personagem{
            nome: "Kit Carlson".to_string(),
            descricao: "Olha as 3 primeiras cartas do topo do baralho de compras, 
            escolhe duas que ficam com ele, devolve a última pro topo, e isso representa a sua 
            fase de compras.".to_string(),
            atributos: Atributos{
                vida_atual: 4,
                vida_maxima: 4,
                efeitos: vec![Efeito::KitCarlsonCartas],
                distancia: 1,
                visao: 1,
                limitecompra: 3
            }
        },
        Personagem{
            nome: "Lucky Duke".to_string(),
            descricao: "Toda vez que for “checar” (como para Barril, Dinamite, Prisão, etc), 
            ele vira 2 cartas e escolhe a que melhor representa seus interesses. 
            As cartas são descartadas logo após.".to_string(),
            atributos: Atributos{
                vida_atual: 4,
                vida_maxima: 4,
                efeitos: vec![Efeito::LuckyDukeChecagem],
                distancia: 1,
                visao: 1,
                limitecompra: 2
            }
        },
        Personagem{
            nome: "Paul Regret".to_string(),
            descricao: "Todos os outros jogadores o veem a um a mais de distância.".to_string(),
            atributos: Atributos{
                vida_atual: 3,
                vida_maxima: 3,
                efeitos: vec![Efeito::PaulRegretDistancia],
                distancia: 2,
                visao: 1,
                limitecompra: 2
            }
        },
        Personagem{
            nome: "Pedro Ramirez".to_string(),
            descricao: "Na fase de compras dele, ele pode comprar a primeira carta da 
            pilha de descartes, que fica sempre virada para cima.".to_string(),
            atributos: Atributos{
                vida_atual: 4,
                vida_maxima: 4,
                efeitos: vec![Efeito::PedroRamirezCompra],
                distancia: 1,
                visao: 1,
                limitecompra: 2
            }
        },
        Personagem{
            nome: "Rose Doolan".to_string(),
            descricao: "Enxerga todos os outros jogadores a um a menos de distância.".to_string(),
            atributos: Atributos{
                vida_atual: 4,
                vida_maxima: 4,
                efeitos: vec![Efeito::RoseDoolanVisao],
                distancia: 1,
                visao: 2,
                limitecompra: 2
            }
        },
        Personagem{
            nome: "Bart Cassidy".to_string(),
            descricao: "Toda vez que ele perder um ponto de vida, ele compra uma carta.".to_string(),
            atributos: Atributos{
                vida_atual: 4,
                vida_maxima: 4,
                efeitos: vec![Efeito::BartCassidyCompra],
                distancia: 1,
                visao: 1,
                limitecompra: 2
            }
        },
        Personagem{
            nome: "Sid Ketchum".to_string(),
            descricao: "Descarta 2 cartas da mão para recuperar 1 ponto de vida.
             Pode fazer quantas vezes quiser, porém, não pode ter mais pontos de 
             vida do que o número de balas na lateral da carta de Poder.".to_string(),
            atributos: Atributos{
                vida_atual: 4,
                vida_maxima: 4,
                efeitos: vec![Efeito::SidKetchumDescarte],
                distancia: 1,
                visao: 1,
                limitecompra: 2
            }
        },
        Personagem{
            nome: "Black Jack".to_string(),
            descricao: "Na fase de compras, ele mostra a segunda carta comprada do bolo. 
            Se ela for vermelha (de Copas ou Ouros), ele compra mais uma.".to_string(),
            atributos: Atributos{
                vida_atual: 4,
                vida_maxima: 4,
                efeitos: vec![Efeito::BlackJackCompra],
                distancia: 1,
                visao: 1,
                limitecompra: 2
            }
        },
        Personagem{
            nome: "Slab The Killer".to_string(),
            descricao: "Força seus oponentes a usarem 2 cartas de Esquiva para poderem 
            desviar de 1 Bang dele.".to_string(),
            atributos: Atributos{
                vida_atual: 4,
                vida_maxima: 4,
                efeitos: vec![Efeito::SlabTheKillerBang],
                distancia: 1,
                visao: 1,
                limitecompra: 2
            }
        },
        Personagem{
            nome: "Calamity Janet".to_string(),
            descricao: "Usa as cartas “Bang!” e “Esquiva” como se fossem a mesma coisa.".to_string(),
            atributos: Atributos{
                vida_atual: 4,
                vida_maxima: 4,
                efeitos: vec![Efeito::CalamityJanetBangEsquiva],
                distancia: 1,
                visao: 1,
                limitecompra: 2
            }
        },
        Personagem{
            nome: "Suzy Lafayette".to_string(),
            descricao: " Se tiver usado todas as cartas da mão, ela compra uma nova 
            da pilha de compra. Pode repetir quantas vezes for necessário/possível/desejável."
            .to_string(),
            atributos: Atributos{
                vida_atual: 4,
                vida_maxima: 4,
                distancia: 1,
                visao: 1,
                limitecompra: 2,
                efeitos: vec![Efeito::SuzyLafayetteCompra]
            }   
        },
        Personagem{
            nome: "El Gringo".to_string(),
            descricao: "Toda vez que perder 1 ponto de vida por causa da carta de um jogador, 
            ele rouba 1 carta da mão dele. Caso ele não tenha mais cartas na mão dele, azar. 
            A Dinamite pertence a mesa, então se ela explodir nele, ele não pega cartas de ninguém."
            .to_string(),
            atributos: Atributos{
                vida_atual: 4,
                vida_maxima: 4,
                distancia: 1,
                visao: 1,
                limitecompra: 2,
                efeitos: vec![Efeito::ElGringoRoubo]
            }
        },
        Personagem{
            nome: "Vulture Sam".to_string(),
            descricao: "Toda vez que um jogador for eliminado, Sam pegará todas as cartas
             da mão e da mesa do falecido para si.".to_string(),
            atributos: Atributos{
                vida_atual: 4,
                vida_maxima: 4,
                distancia: 1,
                visao: 1,
                limitecompra: 2,
                efeitos: vec![Efeito::VultureSamRoubo]
            }
        }
    ];

    Json(personagens)
}