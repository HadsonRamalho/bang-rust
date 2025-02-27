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
        }
    ];

    Json(personagens)
}