pub struct Personagem{
    pub nome: String,
    pub descricao: String,
    pub atributos: Atributos
}

pub struct Atributos{
    pub vida_atual: i32,
    pub vida_maxima: i32,
    pub efeitos: Vec<Efeito>,
    pub distancia: i32,
    pub visao: i32
}

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

pub async fn lista_personagens(){
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
                visao: 1
            }
        },
        Personagem{
            nome: "Jesse Jones".to_string(),
            descricao: "Pode comprar a sua primeira carta
             (das duas que são compradas no início de cada rodada) da mão de um jogador.".to_string(),
            atributos: Atributos{
                vida_atual: 4,
                vida_maxima: 4,
                efeitos: vec![Efeito::KitCarlsonCartas],
                distancia: 1,
                visao: 1
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
                visao: 1
            }            
        }
    ];
}